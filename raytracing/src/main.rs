use std::fs::File;
use std::io::{self, BufWriter, Write};

fn main() -> io::Result<()> {
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    let mut f = BufWriter::new(File::create("out.ppm")?);

    writeln!(f, "P3")?;
    writeln!(f, "{IMAGE_WIDTH} {IMAGE_HEIGHT}")?;
    writeln!(f, "255")?;

    for h in 0..IMAGE_HEIGHT {
        for w in 0..IMAGE_WIDTH {
            let r = w as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = h as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.0;

            let r = (255.999 * r) as u32;
            let g = (255.999 * g) as u32;
            let b = (255.999 * b) as u32;

            writeln!(f, "{r} {g} {b}")?;
        }
    }

    Ok(())
}
