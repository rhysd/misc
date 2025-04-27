use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Color, Point3, Vec3};
use rand::random_range;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::iter::repeat_with;
use std::ops::Add;
use std::path::Path;

pub struct Camera {
    pub aspect_ratio: f64,      // Ratio of image width over height
    pub image_width: u32,       // Rendered image width in pixel count
    pub samples_per_pixel: u32, // Count of random samples for each pixel
    pub max_depth: u8,          // Maximum number of ray bounces into scene
    out: BufWriter<File>,
    image_height: u32,        // Rendered image height
    pixel_samples_scale: f64, // Color scale factor for a sum of pixel samples
    center: Point3,           // Camera center
    pixel00_loc: Point3,      // Location of pixel (0, 0)
    pixel_delta_u: Vec3,      // Offset to pixel to the right
    pixel_delta_v: Vec3,      // Offset to pixel below
}

impl Camera {
    pub fn new(path: impl AsRef<Path>) -> io::Result<Self> {
        Ok(Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            out: BufWriter::new(File::create(path.as_ref())?),
            // These will be calculated by `initialize()`
            image_height: 0,
            pixel_samples_scale: 0.0,
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
        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;
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
                let sum = repeat_with(|| self.ray_to(w, h).color(self.max_depth, world))
                    .take(self.samples_per_pixel as _)
                    .reduce(Add::add)
                    .unwrap_or_default();
                self.write_color(sum * self.pixel_samples_scale)?;
            }
        }

        Ok(())
    }

    fn ray_to(&self, w: u32, h: u32) -> Ray {
        // Construct a camera ray originating from the origin and directed at randomly sampled
        // point around the pixel location (w, h).

        // Random pixel location (x, y) in the [-0.5,-0.5]..[+0.5,+0.5] unit square around the center of target pixel
        let pixel_x = w as f64 + random_range(-0.5..0.5);
        let pixel_y = h as f64 + random_range(-0.5..0.5);

        let pixel_sample = self.pixel00_loc + pixel_x * self.pixel_delta_u + pixel_y * self.pixel_delta_v;
        let origin = self.center;
        let direction = pixel_sample - origin;

        Ray::new(origin, direction)
    }

    fn write_color(&mut self, c: Color) -> io::Result<()> {
        let (r, g, b) = (c.x(), c.y(), c.z());

        // Ensure `r`, `g`, and `b` are in range of [0..255]
        const INTENSITY: Interval = Interval::new(0.0, 0.999);
        let r = (256.0 * INTENSITY.clamp(r)) as u8;
        let g = (256.0 * INTENSITY.clamp(g)) as u8;
        let b = (256.0 * INTENSITY.clamp(b)) as u8;

        writeln!(self.out, "{r} {g} {b}")
    }
}
