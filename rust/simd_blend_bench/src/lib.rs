use packed_simd::u32x4;

pub type Color = (u8, u8, u8);

pub fn blend_scalar(c1: Color, c2: Color, ratio: u8) -> Color {
    let r = (c1.0 as u32 * ratio as u32 + c2.0 as u32 * (255 - ratio as u32)) / 255;
    let g = (c1.1 as u32 * ratio as u32 + c2.1 as u32 * (255 - ratio as u32)) / 255;
    let b = (c1.2 as u32 * ratio as u32 + c2.2 as u32 * (255 - ratio as u32)) / 255;
    (r as u8, g as u8, b as u8)
}

pub fn blend_simd(c1: Color, c2: Color, ratio: u8) -> Color {
    let c1 = u32x4::new(c1.0 as u32, c1.1 as u32, c1.2 as u32, 0);
    let c2 = u32x4::new(c2.0 as u32, c2.1 as u32, c2.2 as u32, 0);
    let c3 = (c1 * (ratio as u32) + c2 * (255 - ratio as u32)) / 255;
    (
        c3.extract(0) as u8,
        c3.extract(1) as u8,
        c3.extract(2) as u8,
    )
}
