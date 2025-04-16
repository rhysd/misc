use std::io::{self, Write};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        let r = (255.999 * r) as u8;
        let g = (255.999 * g) as u8;
        let b = (255.999 * b) as u8;
        Self { r, g, b }
    }

    pub fn write_to<W: Write>(self, w: &mut W) -> io::Result<()> {
        let Self { r, g, b } = self;
        writeln!(w, "{r} {g} {b}")
    }
}
