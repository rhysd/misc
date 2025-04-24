use crate::interval::Interval;
use std::io::{self, Write};

#[derive(Clone, Copy, PartialEq, Default)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }

    pub fn write_to<W: Write>(self, w: &mut W) -> io::Result<()> {
        let Self { r, g, b } = self;

        // Ensure `r`, `g`, and `b` are in range of [0..255]
        const INTENSITY: Interval = Interval::new(0.0, 0.999);
        let r = (256.0 * INTENSITY.clamp(r)) as u8;
        let g = (256.0 * INTENSITY.clamp(g)) as u8;
        let b = (256.0 * INTENSITY.clamp(b)) as u8;

        writeln!(w, "{r} {g} {b}")
    }
}
