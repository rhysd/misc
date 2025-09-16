mod aabb;
mod camera;
mod hittable;
mod interval;
mod material;
mod ray;
mod vec3;

use camera::Camera;
use hittable::{Hittables, Sphere};
use material::{Dielectric, Lambertian, Metal};
use rand::random_range;
use std::io;
use std::path::PathBuf;
use vec3::{Color, Point3, Vec3};

enum Action {
    Render,
    RenderTo(PathBuf),
    Help(&'static str),
}

fn parse_args(cam: &mut Camera) -> Result<Action, lexopt::Error> {
    use lexopt::prelude::*;

    let mut action = Action::Render;
    let mut parser = lexopt::Parser::from_env();
    while let Some(arg) = parser.next()? {
        match arg {
            Long("width") => cam.image_width = parser.value()?.parse()?,
            Long("aspect") => cam.aspect_ratio = parser.value()?.parse()?,
            Long("samples") => cam.samples_per_pixel = parser.value()?.parse()?,
            Long("depth") => cam.max_depth = parser.value()?.parse()?,
            Value(val) => action = Action::RenderTo(PathBuf::from(val)),
            Short('h') | Long("help") => {
                return Ok(Action::Help(
                    r#"Usage: raytracing [OPTIONS] [PATH]

Options:
    --width VALUE    Width in pixels
    --aspect VALUE   Aspect ratio in float number
    --samples VALUE  Samples per pixel
    --depth VALUE    Max depth of ray scattering
    --help           Show this help
"#,
                ));
            }
            _ => return Err(arg.unexpected()),
        }
    }
    Ok(action)
}

fn main() -> io::Result<()> {
    let mut world = Hittables::default();

    // Ground
    world.add(Sphere::stationary(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(Color::new(0.5, 0.5, 0.5)),
    ));

    for a in -11..11 {
        for b in -11..11 {
            let center = {
                let x = a as f64 + random_range(0.0..0.9);
                let z = b as f64 + random_range(0.0..0.9);
                Point3::new(x, 0.2, z)
            };

            if (center - Point3::new(4.0, 0.2, 0.0)).length() <= 0.9 {
                continue;
            }

            let random = random_range(0.0..1.0);
            if random < 0.8 {
                // Diffuse
                let albedo = Color::random(0.0..1.0) * Color::random(0.0..1.0);
                let center_end = center + Vec3::new(0.0, rand::random_range(0.0..0.5), 0.0);
                let sphere = Sphere::moving(center, center_end, 0.2, Lambertian::new(albedo));
                world.add(sphere);
            } else if random < 0.95 {
                // Metal
                let albedo = Color::random(0.5..1.0);
                let fuzz = random_range(0.0..0.5);
                let sphere = Sphere::stationary(center, 0.2, Metal::new(albedo, fuzz));
                world.add(sphere);
            } else {
                // Glass
                let sphere = Sphere::stationary(center, 0.2, Dielectric::new(1.5));
                world.add(sphere);
            }
        }
    }

    world.add(Sphere::stationary(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Dielectric::new(1.5),
    ));
    world.add(Sphere::stationary(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Lambertian::new(Color::new(0.4, 0.2, 0.1)),
    ));
    world.add(Sphere::stationary(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Metal::new(Color::new(0.7, 0.6, 0.5), 0.0),
    ));

    let mut cam = Camera::new()?;
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

    match parse_args(&mut cam).map_err(io::Error::other)? {
        Action::Render => cam.render("out.ppm", &world),
        Action::RenderTo(path) => cam.render(&path, &world),
        Action::Help(help) => {
            println!("{help}");
            Ok(())
        }
    }
}
