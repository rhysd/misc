use std::{hint::black_box, path::PathBuf};

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use font_kit::{
    canvas::{Canvas, Format, RasterizationOptions},
    font::Font as FontKitFont,
    handle::Handle as FontKitHandle,
    hinting::HintingOptions,
};
use pathfinder_geometry::transform2d::Transform2F;
use swash::{
    FontRef, GlyphId,
    scale::{Render, ScaleContext, Source, StrikeWith, image::Image},
    shape::ShapeContext,
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
    input: GlyphInput,
    sources: &'static [Source],
    platform: platform::SuitePlatform,
}

struct FontKitRasterizer {
    font: FontKitFont,
    glyph_id: u32,
    size: f32,
    transform: Transform2F,
    hinting_options: HintingOptions,
    rasterization_options: RasterizationOptions,
    canvas: Canvas,
}

impl FontKitRasterizer {
    fn new(suite: FontSuite, glyph_id: GlyphId, size: f32) -> Self {
        let font =
            FontKitHandle::from_path(PathBuf::from(suite.font_path), suite.font_index as u32)
                .load()
                .expect("failed to load font-kit font");
        let glyph_id = glyph_id as u32;
        assert_ne!(glyph_id, 0, "suite {:?} resolved .notdef", suite.name);
        assert!(
            glyph_id < font.glyph_count(),
            "suite {:?} resolved glyph {} outside font-kit glyph count {}",
            suite.name,
            glyph_id,
            font.glyph_count()
        );

        let hinting_options = font_kit_hinting_options(&font, size);
        let rasterization_options = RasterizationOptions::GrayscaleAa;
        let transform = Transform2F::default();
        let raster_bounds = font
            .raster_bounds(
                glyph_id,
                size,
                transform,
                hinting_options,
                rasterization_options,
            )
            .expect("failed to get font-kit raster bounds");
        let canvas = Canvas::new(raster_bounds.size(), font_kit_canvas_format(suite));
        assert!(
            !canvas.pixels.is_empty(),
            "suite {:?} produced empty font-kit raster bounds",
            suite.name
        );

        let mut rasterizer = Self {
            font,
            glyph_id,
            size,
            transform: Transform2F::from_translation(-raster_bounds.origin().to_f32()) * transform,
            hinting_options,
            rasterization_options,
            canvas,
        };
        assert_ne!(rasterizer.rasterize(), 0);
        rasterizer
    }

    fn rasterize(&mut self) -> usize {
        self.font
            .rasterize_glyph(
                &mut self.canvas,
                self.glyph_id,
                self.size,
                self.transform,
                self.hinting_options,
                self.rasterization_options,
            )
            .expect("failed to rasterize font-kit glyph");
        self.canvas.pixels.len()
    }

    fn buffer(&self) -> &[u8] {
        self.canvas.pixels.as_slice()
    }
}

fn font_kit_hinting_options(font: &FontKitFont, size: f32) -> HintingOptions {
    let full = HintingOptions::Full(size);
    if font.supports_hinting_options(full, true) {
        return full;
    }

    let vertical = HintingOptions::Vertical(size);
    if font.supports_hinting_options(vertical, true) {
        return vertical;
    }

    HintingOptions::None
}

fn font_kit_canvas_format(suite: FontSuite) -> Format {
    if suite.sources.len() == COLOR_SOURCES.len() {
        Format::Rgba32
    } else {
        Format::A8
    }
}

#[cfg(target_os = "windows")]
mod platform {
    use super::*;
    use std::mem::ManuallyDrop;

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

    #[derive(Clone, Copy)]
    pub struct SuitePlatform {
        dwrite_face_type: DWRITE_FONT_FACE_TYPE,
    }

    const fn suite_platform(dwrite_face_type: DWRITE_FONT_FACE_TYPE) -> SuitePlatform {
        SuitePlatform { dwrite_face_type }
    }

    pub const BACKEND_NAME: &str = "directwrite";

    pub const SUITES: &[FontSuite] = &[
        FontSuite {
            name: "latin_A",
            font_path: r"C:\Windows\Fonts\segoeui.ttf",
            font_index: 0,
            input: GlyphInput::Char('A'),
            sources: &OUTLINE_SOURCES,
            platform: suite_platform(DWRITE_FONT_FACE_TYPE_TRUETYPE),
        },
        FontSuite {
            name: "cjk_kanji",
            font_path: r"C:\Windows\Fonts\YuGothR.ttc",
            font_index: 0,
            input: GlyphInput::Char('\u{6f22}'),
            sources: &OUTLINE_SOURCES,
            platform: suite_platform(DWRITE_FONT_FACE_TYPE_TRUETYPE_COLLECTION),
        },
        FontSuite {
            name: "latin_fi_ligature",
            font_path: r"C:\Windows\Fonts\calibri.ttf",
            font_index: 0,
            input: GlyphInput::ShapedText {
                text: "fi",
                expect_ligature: true,
            },
            sources: &OUTLINE_SOURCES,
            platform: suite_platform(DWRITE_FONT_FACE_TYPE_TRUETYPE),
        },
        FontSuite {
            name: "emoji_grinning_face",
            font_path: r"C:\Windows\Fonts\seguiemj.ttf",
            font_index: 0,
            input: GlyphInput::Char('\u{1f600}'),
            sources: &COLOR_SOURCES,
            platform: suite_platform(DWRITE_FONT_FACE_TYPE_TRUETYPE),
        },
    ];

    pub struct PlatformRasterizer {
        factory: IDWriteFactory,
        _glyph_indices: Box<[u16; 1]>,
        _glyph_advances: Box<[f32; 1]>,
        _glyph_offsets: Box<[DWRITE_GLYPH_OFFSET; 1]>,
        glyph_run: DWRITE_GLYPH_RUN,
        pub buffer: Vec<u8>,
    }

    impl PlatformRasterizer {
        pub fn new(suite: FontSuite, glyph_id: GlyphId, size: f32) -> Self {
            unsafe {
                let factory: IDWriteFactory = DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED)
                    .expect("failed to create DirectWrite factory");
                let font_path = HSTRING::from(suite.font_path);
                let font_file = factory
                    .CreateFontFileReference(&font_path, None)
                    .expect("failed to create DirectWrite font file reference");
                let font_face = factory
                    .CreateFontFace(
                        suite.platform.dwrite_face_type,
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

        pub fn rasterize(&mut self) -> usize {
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

        pub fn buffer(&self) -> &[u8] {
            self.buffer.as_slice()
        }
    }

    impl Drop for PlatformRasterizer {
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
}

#[cfg(target_os = "macos")]
mod platform {
    use super::*;
    use std::{ffi::c_void, ptr::NonNull};

    use objc2_core_foundation::{CFIndex, CFRetained, CFString, CGFloat, CGPoint, CGRect};
    use objc2_core_graphics::{CGColorSpace, CGContext, CGGlyph};
    use objc2_core_text::{CTFont, CTFontOrientation};

    const BITMAP_PADDING: usize = 4;
    const BYTES_PER_PIXEL: usize = 4;
    const K_CG_IMAGE_ALPHA_PREMULTIPLIED_LAST: u32 = 1;
    const K_CG_BITMAP_BYTE_ORDER_32_BIG: u32 = 4 << 12;
    const K_CG_TEXT_FILL: i32 = 0;

    #[allow(non_snake_case)]
    #[link(name = "CoreGraphics", kind = "framework")]
    unsafe extern "C-unwind" {
        fn CGColorSpaceCreateDeviceRGB() -> Option<NonNull<CGColorSpace>>;
        fn CGBitmapContextCreate(
            data: *mut c_void,
            width: usize,
            height: usize,
            bits_per_component: usize,
            bytes_per_row: usize,
            space: Option<&CGColorSpace>,
            bitmap_info: u32,
        ) -> Option<NonNull<CGContext>>;
        fn CGContextSetRGBFillColor(
            c: Option<&CGContext>,
            red: CGFloat,
            green: CGFloat,
            blue: CGFloat,
            alpha: CGFloat,
        );
        fn CGContextSetTextDrawingMode(c: Option<&CGContext>, mode: i32);
    }

    #[derive(Clone, Copy)]
    pub struct SuitePlatform {
        ct_font_name: &'static str,
    }

    const fn suite_platform(ct_font_name: &'static str) -> SuitePlatform {
        SuitePlatform { ct_font_name }
    }

    pub const BACKEND_NAME: &str = "core_text";

    pub const SUITES: &[FontSuite] = &[
        FontSuite {
            name: "latin_A",
            font_path: "/System/Library/Fonts/Supplemental/Arial.ttf",
            font_index: 0,
            input: GlyphInput::Char('A'),
            sources: &OUTLINE_SOURCES,
            platform: suite_platform("ArialMT"),
        },
        FontSuite {
            name: "cjk_kanji",
            font_path: "/System/Library/Fonts/PingFang.ttc",
            font_index: 0,
            input: GlyphInput::Char('\u{6f22}'),
            sources: &OUTLINE_SOURCES,
            platform: suite_platform("PingFangSC-Regular"),
        },
        FontSuite {
            name: "latin_fi_ligature",
            font_path: "/System/Library/Fonts/Supplemental/Times New Roman.ttf",
            font_index: 0,
            input: GlyphInput::ShapedText {
                text: "fi",
                expect_ligature: true,
            },
            sources: &OUTLINE_SOURCES,
            platform: suite_platform("TimesNewRomanPSMT"),
        },
        FontSuite {
            name: "emoji_grinning_face",
            font_path: "/System/Library/Fonts/Apple Color Emoji.ttc",
            font_index: 0,
            input: GlyphInput::Char('\u{1f600}'),
            sources: &COLOR_SOURCES,
            platform: suite_platform("AppleColorEmoji"),
        },
    ];

    pub struct PlatformRasterizer {
        font: CFRetained<CTFont>,
        _color_space: CFRetained<CGColorSpace>,
        context: CFRetained<CGContext>,
        glyphs: Box<[CGGlyph; 1]>,
        positions: Box<[CGPoint; 1]>,
        pub buffer: Vec<u8>,
    }

    impl PlatformRasterizer {
        pub fn new(suite: FontSuite, glyph_id: GlyphId, size: f32) -> Self {
            unsafe {
                assert_ne!(glyph_id, 0, "suite {:?} resolved .notdef", suite.name);

                let font_name = CFString::from_static_str(suite.platform.ct_font_name);
                let font = CTFont::with_name(&font_name, size as CGFloat, std::ptr::null());

                let mut glyphs = Box::new([glyph_id as CGGlyph]);
                let glyph_ptr = NonNull::from(&mut glyphs[0]);
                let mut glyph_bounds = CGRect::ZERO;
                let overall_bounds = font.bounding_rects_for_glyphs(
                    CTFontOrientation::Default,
                    glyph_ptr,
                    &mut glyph_bounds,
                    1 as CFIndex,
                );
                let bounds = if glyph_bounds.is_empty() {
                    overall_bounds
                } else {
                    glyph_bounds
                };

                let width = bitmap_dimension(bounds.size.width, size);
                let height = bitmap_dimension(bounds.size.height, size);
                let bytes_per_row = width * BYTES_PER_PIXEL;
                let mut buffer = vec![0; bytes_per_row * height];
                let color_space = create_device_rgb_color_space();
                let context_raw = CGBitmapContextCreate(
                    buffer.as_mut_ptr().cast::<c_void>(),
                    width,
                    height,
                    8,
                    bytes_per_row,
                    Some(&*color_space),
                    K_CG_IMAGE_ALPHA_PREMULTIPLIED_LAST | K_CG_BITMAP_BYTE_ORDER_32_BIG,
                )
                .expect("failed to create Core Graphics bitmap context");
                let context = CFRetained::from_raw(context_raw);
                CGContextSetRGBFillColor(Some(&*context), 0.0, 0.0, 0.0, 1.0);
                CGContextSetTextDrawingMode(Some(&*context), K_CG_TEXT_FILL);

                let positions = Box::new([glyph_position(bounds)]);
                let mut rasterizer = Self {
                    font,
                    _color_space: color_space,
                    context,
                    glyphs,
                    positions,
                    buffer,
                };
                assert_ne!(rasterizer.rasterize(), 0);
                rasterizer
            }
        }

        pub fn rasterize(&mut self) -> usize {
            self.buffer.fill(0);
            let glyphs = NonNull::from(&mut self.glyphs[0]);
            let positions = NonNull::from(&mut self.positions[0]);
            unsafe {
                self.font.draw_glyphs(glyphs, positions, 1, &self.context);
            }
            self.buffer.len()
        }

        pub fn buffer(&self) -> &[u8] {
            self.buffer.as_slice()
        }
    }

    fn bitmap_dimension(value: CGFloat, fallback: f32) -> usize {
        let fallback = (fallback as CGFloat).ceil() as usize;
        let value = value.abs().ceil() as usize;
        value.max(fallback).max(1) + BITMAP_PADDING * 2
    }

    fn glyph_position(bounds: CGRect) -> CGPoint {
        let padding = BITMAP_PADDING as CGFloat;
        CGPoint::new(padding - bounds.origin.x, padding - bounds.origin.y)
    }

    fn create_device_rgb_color_space() -> CFRetained<CGColorSpace> {
        unsafe {
            let raw = CGColorSpaceCreateDeviceRGB()
                .expect("failed to create Core Graphics device RGB color space");
            CFRetained::from_raw(raw)
        }
    }
}

#[cfg(target_os = "linux")]
mod platform {
    use super::*;

    use freetype::{Face, Library, face::LoadFlag};

    #[derive(Clone, Copy)]
    pub struct SuitePlatform {
        size_mode: SizeMode,
        load_mode: LoadMode,
    }

    #[derive(Clone, Copy)]
    enum SizeMode {
        PixelSize,
        BestFixedStrike,
    }

    #[derive(Clone, Copy)]
    enum LoadMode {
        Gray,
        Color,
    }

    impl LoadMode {
        fn flags(self) -> LoadFlag {
            match self {
                Self::Gray => LoadFlag::RENDER,
                Self::Color => LoadFlag::RENDER | LoadFlag::COLOR,
            }
        }
    }

    const fn suite_platform(size_mode: SizeMode, load_mode: LoadMode) -> SuitePlatform {
        SuitePlatform {
            size_mode,
            load_mode,
        }
    }

    pub const BACKEND_NAME: &str = "freetype";

    pub const SUITES: &[FontSuite] = &[
        FontSuite {
            name: "latin_A",
            font_path: "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
            font_index: 0,
            input: GlyphInput::Char('A'),
            sources: &OUTLINE_SOURCES,
            platform: suite_platform(SizeMode::PixelSize, LoadMode::Gray),
        },
        FontSuite {
            name: "cjk_kanji",
            font_path: "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
            font_index: 0,
            input: GlyphInput::Char('\u{6f22}'),
            sources: &OUTLINE_SOURCES,
            platform: suite_platform(SizeMode::PixelSize, LoadMode::Gray),
        },
        FontSuite {
            name: "latin_fi_ligature",
            font_path: "/usr/share/fonts/truetype/dejavu/DejaVuSerif.ttf",
            font_index: 0,
            input: GlyphInput::ShapedText {
                text: "fi",
                expect_ligature: true,
            },
            sources: &OUTLINE_SOURCES,
            platform: suite_platform(SizeMode::PixelSize, LoadMode::Gray),
        },
        FontSuite {
            name: "emoji_grinning_face",
            font_path: "/usr/share/fonts/truetype/noto/NotoColorEmoji.ttf",
            font_index: 0,
            input: GlyphInput::Char('\u{1f600}'),
            sources: &COLOR_SOURCES,
            platform: suite_platform(SizeMode::BestFixedStrike, LoadMode::Color),
        },
    ];

    pub struct PlatformRasterizer {
        face: Face,
        _library: Library,
        glyph_id: GlyphId,
        load_mode: LoadMode,
    }

    impl PlatformRasterizer {
        pub fn new(suite: FontSuite, glyph_id: GlyphId, size: f32) -> Self {
            assert_ne!(glyph_id, 0, "suite {:?} resolved .notdef", suite.name);

            let library = Library::init().expect("failed to initialize FreeType");
            let mut face = library
                .new_face(suite.font_path, suite.font_index as isize)
                .expect("failed to create FreeType face");
            select_size(&mut face, suite.platform.size_mode, size);

            let mut rasterizer = Self {
                face,
                _library: library,
                glyph_id,
                load_mode: suite.platform.load_mode,
            };
            assert_ne!(rasterizer.rasterize(), 0);
            rasterizer
        }

        pub fn rasterize(&mut self) -> usize {
            self.face
                .load_glyph(self.glyph_id as u32, self.load_mode.flags())
                .expect("failed to render FreeType glyph");
            self.buffer().len()
        }

        pub fn buffer(&self) -> &[u8] {
            self.face.glyph().bitmap().buffer()
        }
    }

    fn select_size(face: &mut Face, size_mode: SizeMode, size: f32) {
        match size_mode {
            SizeMode::PixelSize => {
                face.set_pixel_sizes(0, size.round() as u32)
                    .expect("failed to set FreeType pixel size");
            }
            SizeMode::BestFixedStrike => {
                if let Some(strike_index) = best_fixed_strike(face, size) {
                    select_fixed_strike(face, strike_index);
                } else {
                    face.set_pixel_sizes(0, size.round() as u32)
                        .expect("failed to set FreeType pixel size");
                }
            }
        }
    }

    fn best_fixed_strike(face: &Face, size: f32) -> Option<i32> {
        let raw = face.raw();
        if raw.num_fixed_sizes <= 0 || raw.available_sizes.is_null() {
            return None;
        }

        let target_ppem = (size * 64.0).round() as i64;
        let strikes = unsafe {
            std::slice::from_raw_parts(raw.available_sizes, raw.num_fixed_sizes as usize)
        };
        strikes
            .iter()
            .enumerate()
            .min_by_key(|(_, strike)| (strike.y_ppem as i64 - target_ppem).abs())
            .map(|(index, _)| index as i32)
    }

    fn select_fixed_strike(face: &mut Face, strike_index: i32) {
        face.select_size(strike_index)
            .expect("failed to select FreeType fixed strike");
    }
}

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
mod platform {
    use super::*;

    #[derive(Clone, Copy)]
    pub struct SuitePlatform;

    pub const BACKEND_NAME: &str = "platform";
    pub const SUITES: &[FontSuite] = &[];

    pub struct PlatformRasterizer {
        pub buffer: Vec<u8>,
    }

    impl PlatformRasterizer {
        pub fn new(_suite: FontSuite, _glyph_id: GlyphId, _size: f32) -> Self {
            unreachable!("only Windows and macOS platform rasterizers are implemented");
        }

        pub fn rasterize(&mut self) -> usize {
            unreachable!("only Windows, macOS, and Linux platform rasterizers are implemented");
        }

        pub fn buffer(&self) -> &[u8] {
            self.buffer.as_slice()
        }
    }
}

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
compile_error!("only Windows, macOS, and Linux platform rasterizers are implemented");

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
    for &suite in platform::SUITES {
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
                BenchmarkId::new("font-kit", &size_label),
                &size,
                |b, &size| {
                    let mut rasterizer = FontKitRasterizer::new(suite, glyph_id, size);

                    b.iter(|| {
                        let len = rasterizer.rasterize();
                        black_box(len);
                        black_box(rasterizer.buffer());
                    });
                },
            );

            group.bench_with_input(
                BenchmarkId::new(platform::BACKEND_NAME, &size_label),
                &size,
                |b, &size| {
                    let mut rasterizer = platform::PlatformRasterizer::new(suite, glyph_id, size);

                    b.iter(|| {
                        let len = rasterizer.rasterize();
                        black_box(len);
                        black_box(rasterizer.buffer());
                    });
                },
            );
        }
        group.finish();
    }
}

criterion_group!(benches, rasterize_benchmark);
criterion_main!(benches);
