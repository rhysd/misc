mod camera;
mod hittable;
mod interval;
mod material;
mod ray;
mod vec3;

use camera::Camera;
use hittable::{Hittables, Sphere};
use material::{Dielectric, Lambertian, Metal};
use std::io;
use vec3::{Color, Point3, Vec3};

fn main() -> io::Result<()> {
    let mut world = Hittables::default();

    let ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let left = Dielectric::new(1.5);
    let bubble = Dielectric::new(1.0 / 1.5);
    let right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);

    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, ground));
    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, center));
    world.add(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, left));
    world.add(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.4, bubble));
    world.add(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, right));

    let mut cam = Camera::new("out.ppm")?;
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(-2.0, 2.0, 1.0);
    cam.lookat = Point3::new(0.0, 0.0, -1.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.render(&world)
}
