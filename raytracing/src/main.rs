mod camera;
mod color;
mod hittable;
mod interval;
mod ray;
mod vec3;

use camera::Camera;
use hittable::{Hittables, Sphere};
use std::io;
use vec3::Point3;

fn main() -> io::Result<()> {
    let mut world = Hittables::default();
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    let mut cam = Camera::new("out.ppm")?;
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.render(&world)
}
