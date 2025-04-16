mod color;
mod vec3;

use color::Color;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use vec3::Vec3;

fn main() -> io::Result<()> {
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    let mut f = BufWriter::new(File::create("out.ppm")?);

    writeln!(f, "P3")?;
    writeln!(f, "{IMAGE_WIDTH} {IMAGE_HEIGHT}")?;
    writeln!(f, "255")?;

    for h in 0..IMAGE_HEIGHT {
        for w in 0..IMAGE_WIDTH {
            Color::new(
                w as f64 / (IMAGE_WIDTH - 1) as f64,
                h as f64 / (IMAGE_HEIGHT - 1) as f64,
                0.0,
            )
            .write_to(&mut f)?;
        }
    }

    Ok(())
}
