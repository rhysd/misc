use std::{hint::black_box, mem::ManuallyDrop};

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use swash::{
    FontRef, GlyphId,
    scale::{Render, ScaleContext, Source, image::Image},
};
use windows::{
    Win32::{
        Foundation::RECT,
        Graphics::DirectWrite::{
            DWRITE_FACTORY_TYPE_SHARED, DWRITE_FONT_FACE_TYPE_TRUETYPE,
            DWRITE_FONT_SIMULATIONS_NONE, DWRITE_GLYPH_OFFSET, DWRITE_GLYPH_RUN,
            DWRITE_MEASURING_MODE_NATURAL, DWRITE_RENDERING_MODE_CLEARTYPE_NATURAL_SYMMETRIC,
            DWRITE_TEXTURE_CLEARTYPE_3x1, DWriteCreateFactory, IDWriteFactory,
        },
    },
    core::{BOOL, HSTRING},
};

const FONT_PATH: &str = r"C:\Windows\Fonts\segoeui.ttf";
const GLYPH: char = 'A';
const SIZES: &[f32] = &[12.0, 24.0, 48.0, 96.0];

struct DirectWriteRasterizer {
    factory: IDWriteFactory,
    _glyph_indices: Box<[u16; 1]>,
    _glyph_advances: Box<[f32; 1]>,
    _glyph_offsets: Box<[DWRITE_GLYPH_OFFSET; 1]>,
    glyph_run: DWRITE_GLYPH_RUN,
    buffer: Vec<u8>,
}

impl DirectWriteRasterizer {
    fn new(font_path: &str, glyph: char, size: f32) -> Self {
        unsafe {
            let factory: IDWriteFactory = DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED)
                .expect("failed to create DirectWrite factory");
            let font_path = HSTRING::from(font_path);
            let font_file = factory
                .CreateFontFileReference(&font_path, None)
                .expect("failed to create DirectWrite font file reference");
            let font_face = factory
                .CreateFontFace(
                    DWRITE_FONT_FACE_TYPE_TRUETYPE,
                    &[Some(font_file)],
                    0,
                    DWRITE_FONT_SIMULATIONS_NONE,
                )
                .expect("failed to create DirectWrite font face");

            let codepoint = glyph as u32;
            let mut glyph_index = 0;
            font_face
                .GetGlyphIndices(&codepoint, 1, &mut glyph_index)
                .expect("failed to map DirectWrite glyph");
            assert_ne!(
                glyph_index, 0,
                "DirectWrite font does not contain {glyph:?}"
            );

            let glyph_indices = Box::new([glyph_index]);
            let glyph_advances = Box::new([0.0]);
            let glyph_offsets = Box::new([DWRITE_GLYPH_OFFSET {
                advanceOffset: 0.0,
                ascenderOffset: 0.0,
            }]);
            let glyph_run = DWRITE_GLYPH_RUN {
                fontFace: ManuallyDrop::new(Some(font_face)),
                fontEmSize: size,
                glyphCount: 1,
                glyphIndices: glyph_indices.as_ptr(),
                glyphAdvances: glyph_advances.as_ptr(),
                glyphOffsets: glyph_offsets.as_ptr(),
                isSideways: BOOL(0),
                bidiLevel: 0,
            };

            let mut rasterizer = Self {
                factory,
                _glyph_indices: glyph_indices,
                _glyph_advances: glyph_advances,
                _glyph_offsets: glyph_offsets,
                glyph_run,
                buffer: Vec::new(),
            };
            assert_ne!(rasterizer.rasterize(), 0);
            rasterizer
        }
    }

    fn rasterize(&mut self) -> usize {
        unsafe {
            let analysis = self
                .factory
                .CreateGlyphRunAnalysis(
                    &self.glyph_run,
                    1.0,
                    None,
                    DWRITE_RENDERING_MODE_CLEARTYPE_NATURAL_SYMMETRIC,
                    DWRITE_MEASURING_MODE_NATURAL,
                    0.0,
                    0.0,
                )
                .expect("failed to create DirectWrite glyph run analysis");
            let bounds = analysis
                .GetAlphaTextureBounds(DWRITE_TEXTURE_CLEARTYPE_3x1)
                .expect("failed to get DirectWrite alpha texture bounds");
            let len = texture_len(bounds, 3);
            self.buffer.resize(len, 0);
            analysis
                .CreateAlphaTexture(DWRITE_TEXTURE_CLEARTYPE_3x1, &bounds, &mut self.buffer)
                .expect("failed to create DirectWrite alpha texture");
            len
        }
    }
}

impl Drop for DirectWriteRasterizer {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.glyph_run.fontFace);
        }
    }
}

fn texture_len(bounds: RECT, bytes_per_pixel: usize) -> usize {
    let width = (bounds.right - bounds.left).max(0) as usize;
    let height = (bounds.bottom - bounds.top).max(0) as usize;
    width * height * bytes_per_pixel
}

fn swash_glyph(font: FontRef<'_>, glyph: char) -> GlyphId {
    let glyph_id = font.charmap().map(glyph as u32);
    assert_ne!(glyph_id, 0, "swash font does not contain {glyph:?}");
    glyph_id
}

fn rasterize_benchmark(c: &mut Criterion) {
    let font_data = std::fs::read(FONT_PATH).expect("failed to read benchmark font");
    let font = FontRef::from_index(&font_data, 0).expect("failed to parse benchmark font");
    let glyph_id = swash_glyph(font, GLYPH);
    let sources = [Source::Outline];

    let mut group = c.benchmark_group("glyph_rasterize");
    for &size in SIZES {
        let size_label = format!("{size:.0}px");

        group.bench_with_input(
            BenchmarkId::new("swash_outline_alpha_hinted", &size_label),
            &size,
            |b, &size| {
                let mut context = ScaleContext::new();
                let mut scaler = context.builder(font).size(size).hint(true).build();
                let renderer = Render::new(&sources);
                let mut image = Image::new();
                assert!(renderer.render_into(&mut scaler, glyph_id, &mut image));

                b.iter(|| {
                    let ok = renderer.render_into(&mut scaler, black_box(glyph_id), &mut image);
                    black_box(ok);
                    black_box(image.data.as_slice());
                    black_box(image.placement);
                });
            },
        );

        group.bench_with_input(
            BenchmarkId::new("directwrite_cleartype", &size_label),
            &size,
            |b, &size| {
                let mut rasterizer = DirectWriteRasterizer::new(FONT_PATH, GLYPH, size);

                b.iter(|| {
                    let len = rasterizer.rasterize();
                    black_box(len);
                    black_box(rasterizer.buffer.as_slice());
                });
            },
        );
    }
    group.finish();
}

criterion_group!(benches, rasterize_benchmark);
criterion_main!(benches);
