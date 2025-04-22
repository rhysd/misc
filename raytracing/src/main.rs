mod color;
mod hittable;
mod interval;
mod ray;
mod vec3;

use hittable::{Hittables, Sphere};
use ray::Ray;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use vec3::{Point3, Vec3};

fn main() -> io::Result<()> {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const { assert!(IMAGE_HEIGHT >= 1) }

    // Camera
    const FOCAL_LENGTH: f64 = 1.0;
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * (IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64); // We don't use ASPECT_RATIO since it's an ideal value
    const CAMERA_CENTER: Point3 = Point3::new(0.0, 0.0, 0.0);
    // Vectors across the horizontal and down the vertical viewport edges
    const VIEWPORT_U: Vec3 = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    const VIEWPORT_V: Vec3 = Vec3::new(0.0, -VIEWPORT_HEIGHT, 0.0);
    // Delta vectors from pixel to pixel
    let pixel_delta_u = VIEWPORT_U / IMAGE_WIDTH as f64;
    let pixel_delta_v = VIEWPORT_V / IMAGE_HEIGHT as f64;
    let viewport_upper_left =
        CAMERA_CENTER - Vec3::new(0.0, 0.0, FOCAL_LENGTH) - VIEWPORT_U / 2.0 - VIEWPORT_V / 2.0;
    // Center of the pixel at the top-left corner
    let pixel_00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let mut f = BufWriter::new(File::create("out.ppm")?);

    writeln!(f, "P3")?;
    writeln!(f, "{IMAGE_WIDTH} {IMAGE_HEIGHT}")?;
    writeln!(f, "255")?;

    let mut world = Hittables::default();
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    for h in 0..IMAGE_HEIGHT {
        for w in 0..IMAGE_WIDTH {
            let pixel_center = pixel_00_loc + w as f64 * pixel_delta_u + h as f64 * pixel_delta_v;
            let ray_direction = pixel_center - CAMERA_CENTER;
            Ray::new(CAMERA_CENTER, ray_direction)
                .color(&world)
                .write_to(&mut f)?;
        }
    }

    Ok(())
}
