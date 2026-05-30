use std::{hint::black_box, mem::ManuallyDrop};

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use swash::{
    FontRef, GlyphId,
    scale::{Render, ScaleContext, Source, StrikeWith, image::Image},
    shape::ShapeContext,
};
use windows::{
    Win32::{
        Foundation::RECT,
        Graphics::DirectWrite::{
            DWRITE_FACTORY_TYPE_SHARED, DWRITE_FONT_FACE_TYPE, DWRITE_FONT_FACE_TYPE_TRUETYPE,
            DWRITE_FONT_FACE_TYPE_TRUETYPE_COLLECTION, DWRITE_FONT_SIMULATIONS_NONE,
            DWRITE_GLYPH_OFFSET, DWRITE_GLYPH_RUN, DWRITE_MEASURING_MODE_NATURAL,
            DWRITE_RENDERING_MODE_CLEARTYPE_NATURAL_SYMMETRIC, DWRITE_TEXTURE_CLEARTYPE_3x1,
            DWriteCreateFactory, IDWriteFactory,
        },
    },
    core::{BOOL, HSTRING},
};

const SIZES: &[f32] = &[12.0, 24.0, 48.0, 96.0];

static OUTLINE_SOURCES: [Source; 1] = [Source::Outline];
static COLOR_SOURCES: [Source; 3] = [
    Source::ColorBitmap(StrikeWith::BestFit),
    Source::ColorOutline(0),
    Source::Outline,
];

#[derive(Clone, Copy)]
enum GlyphInput {
    Char(char),
    ShapedText {
        text: &'static str,
        expect_ligature: bool,
    },
}

#[derive(Clone, Copy)]
struct FontSuite {
    name: &'static str,
    font_path: &'static str,
    font_index: usize,
    dwrite_face_type: DWRITE_FONT_FACE_TYPE,
    input: GlyphInput,
    sources: &'static [Source],
}

const SUITES: &[FontSuite] = &[
    FontSuite {
        name: "latin_A",
        font_path: r"C:\Windows\Fonts\segoeui.ttf",
        font_index: 0,
        dwrite_face_type: DWRITE_FONT_FACE_TYPE_TRUETYPE,
        input: GlyphInput::Char('A'),
        sources: &OUTLINE_SOURCES,
    },
    FontSuite {
        name: "cjk_kanji",
        font_path: r"C:\Windows\Fonts\YuGothR.ttc",
        font_index: 0,
        dwrite_face_type: DWRITE_FONT_FACE_TYPE_TRUETYPE_COLLECTION,
        input: GlyphInput::Char('漢'),
        sources: &OUTLINE_SOURCES,
    },
    FontSuite {
        name: "latin_fi_ligature",
        font_path: r"C:\Windows\Fonts\calibri.ttf",
        font_index: 0,
        dwrite_face_type: DWRITE_FONT_FACE_TYPE_TRUETYPE,
        input: GlyphInput::ShapedText {
            text: "fi",
            expect_ligature: true,
        },
        sources: &OUTLINE_SOURCES,
    },
    FontSuite {
        name: "emoji_grinning_face",
        font_path: r"C:\Windows\Fonts\seguiemj.ttf",
        font_index: 0,
        dwrite_face_type: DWRITE_FONT_FACE_TYPE_TRUETYPE,
        input: GlyphInput::Char('😀'),
        sources: &COLOR_SOURCES,
    },
];

struct DirectWriteRasterizer {
    factory: IDWriteFactory,
    _glyph_indices: Box<[u16; 1]>,
    _glyph_advances: Box<[f32; 1]>,
    _glyph_offsets: Box<[DWRITE_GLYPH_OFFSET; 1]>,
    glyph_run: DWRITE_GLYPH_RUN,
    buffer: Vec<u8>,
}

impl DirectWriteRasterizer {
    fn new(suite: FontSuite, glyph_id: GlyphId, size: f32) -> Self {
        unsafe {
            let factory: IDWriteFactory = DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED)
                .expect("failed to create DirectWrite factory");
            let font_path = HSTRING::from(suite.font_path);
            let font_file = factory
                .CreateFontFileReference(&font_path, None)
                .expect("failed to create DirectWrite font file reference");
            let font_face = factory
                .CreateFontFace(
                    suite.dwrite_face_type,
                    &[Some(font_file)],
                    suite.font_index as u32,
                    DWRITE_FONT_SIMULATIONS_NONE,
                )
                .expect("failed to create DirectWrite font face");

            assert_ne!(glyph_id, 0, "suite {:?} resolved .notdef", suite.name);
            let glyph_indices = Box::new([glyph_id]);
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

fn swash_shaped_glyph(font: FontRef<'_>, text: &str, expect_ligature: bool) -> GlyphId {
    let mut context = ShapeContext::new();
    let mut shaper = context.builder(font).features(&[("liga", 1u16)]).build();
    shaper.add_str(text);

    let mut glyphs = Vec::new();
    let mut has_ligature = false;
    shaper.shape_with(|cluster| {
        has_ligature |= cluster.is_ligature();
        glyphs.extend(cluster.glyphs.iter().map(|glyph| glyph.id));
    });

    assert_eq!(
        glyphs.len(),
        1,
        "expected {text:?} to shape into one glyph, got {glyphs:?}"
    );
    assert!(
        !expect_ligature || has_ligature,
        "expected {text:?} to shape as a ligature"
    );
    assert_ne!(glyphs[0], 0, "swash shaped {text:?} into .notdef");
    glyphs[0]
}

fn suite_glyph(font: FontRef<'_>, input: GlyphInput) -> GlyphId {
    match input {
        GlyphInput::Char(ch) => swash_glyph(font, ch),
        GlyphInput::ShapedText {
            text,
            expect_ligature,
        } => swash_shaped_glyph(font, text, expect_ligature),
    }
}

fn rasterize_benchmark(c: &mut Criterion) {
    for &suite in SUITES {
        let font_data = std::fs::read(suite.font_path).expect("failed to read benchmark font");
        let font = FontRef::from_index(&font_data, suite.font_index).expect("failed to parse font");
        let glyph_id = suite_glyph(font, suite.input);

        let mut group = c.benchmark_group(format!("glyph_rasterize/{}", suite.name));
        for &size in SIZES {
            let size_label = format!("{size:.0}px");

            group.bench_with_input(BenchmarkId::new("swash", &size_label), &size, |b, &size| {
                let mut context = ScaleContext::new();
                let mut scaler = context.builder(font).size(size).hint(true).build();
                let renderer = Render::new(suite.sources);
                let mut image = Image::new();
                assert!(renderer.render_into(&mut scaler, glyph_id, &mut image));

                b.iter(|| {
                    let ok = renderer.render_into(&mut scaler, black_box(glyph_id), &mut image);
                    black_box(ok);
                    black_box(image.data.as_slice());
                    black_box(image.placement);
                });
            });

            group.bench_with_input(
                BenchmarkId::new("directwrite", &size_label),
                &size,
                |b, &size| {
                    let mut rasterizer = DirectWriteRasterizer::new(suite, glyph_id, size);

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
}

criterion_group!(benches, rasterize_benchmark);
criterion_main!(benches);
