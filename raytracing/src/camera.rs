use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::path::Path;

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    out: BufWriter<File>,
    image_height: u32,   // Rendered image height
    center: Point3,      // Camera center
    pixel00_loc: Point3, // Location of pixel (0, 0)
    pixel_delta_u: Vec3, // Offset to pixel to the right
    pixel_delta_v: Vec3, // Offset to pixel below
}

impl Camera {
    pub fn new(path: impl AsRef<Path>) -> io::Result<Self> {
        Ok(Self {
            aspect_ratio: 1.0,
            image_width: 100,
            out: BufWriter::new(File::create(path.as_ref())?),
            // These will be calculated by `initialize()`
            image_height: 0,
            center: Point3::ZERO,
            pixel00_loc: Point3::ZERO,
            pixel_delta_u: Vec3::ZERO,
            pixel_delta_v: Vec3::ZERO,
        })
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as u32;
        assert!(self.image_height >= 1);

        // Camera
        const FOCAL_LENGTH: f64 = 1.0;
        const VIEWPORT_HEIGHT: f64 = 2.0;
        let viewport_width: f64 = VIEWPORT_HEIGHT * (self.image_width as f64 / self.image_height as f64); // We don't use `aspect_ratio` since it's an ideal value
        self.center = Point3::new(0.0, 0.0, 0.0);

        // Vectors across the horizontal and down the vertical viewport edges
        let viewport_u: Vec3 = Vec3::new(viewport_width, 0.0, 0.0);
        const VIEWPORT_V: Vec3 = Vec3::new(0.0, -VIEWPORT_HEIGHT, 0.0);

        // Delta vectors from pixel to pixel
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = VIEWPORT_V / self.image_height as f64;
        let viewport_upper_left = self.center - Vec3::new(0.0, 0.0, FOCAL_LENGTH) - viewport_u / 2.0 - VIEWPORT_V / 2.0;

        // Center of the pixel at the top-left corner
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    pub fn render<H: Hittable>(&mut self, world: &H) -> io::Result<()> {
        self.initialize();

        writeln!(self.out, "P3")?;
        writeln!(self.out, "{} {}", self.image_width, self.image_height)?;
        writeln!(self.out, "255")?;

        for h in 0..self.image_height {
            for w in 0..self.image_width {
                let pixel_center = self.pixel00_loc + w as f64 * self.pixel_delta_u + h as f64 * self.pixel_delta_v;
                let ray_direction = pixel_center - self.center;
                Ray::new(self.center, ray_direction)
                    .color(world)
                    .write_to(&mut self.out)?;
            }
        }

        Ok(())
    }
}
