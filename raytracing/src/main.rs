mod camera;
mod hittable;
mod interval;
mod material;
mod ray;
mod vec3;

use camera::Camera;
use hittable::{Hittables, Sphere};
use material::{Dielectric, Lambertian, Metal};
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use std::cell::UnsafeCell;
use std::io;
use std::ops::Range;
use vec3::{Color, Point3, Vec3};

thread_local! {
    pub static RNG: UnsafeCell<SmallRng> = UnsafeCell::new(SmallRng::from_os_rng());
}

pub(crate) fn random(range: Range<f64>) -> f64 {
    RNG.with(|rng| {
        // Safety: This program is single-threaded.
        let rng = unsafe { &mut *rng.get() };
        rng.random_range(range)
    })
}

fn main() -> io::Result<()> {
    let mut world = Hittables::default();

    // Ground
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(Color::new(0.5, 0.5, 0.5)),
    ));

    for a in -11..11 {
        for b in -11..11 {
            let center = {
                let x = a as f64 + random(0.0..0.9);
                let z = b as f64 + random(0.0..0.9);
                Point3::new(x, 0.2, z)
            };

            if (center - Point3::new(4.0, 0.2, 0.0)).length() <= 0.9 {
                continue;
            }

            let x = random(0.0..1.0);
            if x < 0.8 {
                // Diffuse
                let albedo = Color::random(0.0..1.0) * Color::random(0.0..1.0);
                let sphere = Sphere::new(center, 0.2, Lambertian::new(albedo));
                world.add(sphere);
            } else if x < 0.95 {
                // Metal
                let albedo = Color::random(0.5..1.0);
                let fuzz = random(0.0..0.5);
                let sphere = Sphere::new(center, 0.2, Metal::new(albedo, fuzz));
                world.add(sphere);
            } else {
                // Glass
                let sphere = Sphere::new(center, 0.2, Dielectric::new(1.5));
                world.add(sphere);
            }
        }
    }

    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, Dielectric::new(1.5)));
    world.add(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Lambertian::new(Color::new(0.4, 0.2, 0.1)),
    ));
    world.add(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Metal::new(Color::new(0.7, 0.6, 0.5), 0.0),
    ));

    let mut cam = Camera::new("out.ppm")?;
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1200;
    cam.samples_per_pixel = 500;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_distance = 10.0;

    cam.render(&world)
}
