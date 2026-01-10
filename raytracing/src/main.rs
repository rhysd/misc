mod aabb;
mod bvh;
mod camera;
mod interval;
mod material;
mod object;
mod ray;
mod texture;
mod vec3;

use bvh::{Bvh, BvhBuilder};
use camera::Camera;
use material::{Dielectric, Lambertian, Metal};
use object::Sphere;
use rand::random_range;
use std::io;
use std::path::PathBuf;
use texture::CheckerTexture;
use vec3::{Color, Point3, Vec3};

enum Action {
    Render { path: PathBuf, open: bool, parallel: bool },
    Help(&'static str),
}

fn parse_args(cam: &mut Camera) -> Result<Action, lexopt::Error> {
    use lexopt::prelude::*;

    let mut path = PathBuf::from("out.ppm");
    let mut open = false;
    let mut parallel = true;
    let mut parser = lexopt::Parser::from_env();
    while let Some(arg) = parser.next()? {
        match arg {
            Short('w') | Long("width") => cam.image_width = parser.value()?.parse()?,
            Short('h') | Long("height") => cam.image_height = parser.value()?.parse()?,
            Short('s') | Long("samples") => cam.samples_per_pixel = parser.value()?.parse()?,
            Short('d') | Long("depth") => cam.max_depth = parser.value()?.parse()?,
            Short('o') | Long("open") => open = true,
            Short('1') | Long("serial") => parallel = false,
            Value(val) => path = val.into(),
            Long("help") => {
                return Ok(Action::Help(
                    r#"Usage: raytracing [OPTIONS] [PATH]

Arguments:
    PATH                Output file path (default: "out.ppm")

Options:
    -w,--width VALUE    Width in pixels (default: 800)
    -h,--height VALUE   Height in pixels (default: 450)
    -s,--samples VALUE  Samples per pixel (default: 100)
    -d,--depth VALUE    Max depth of ray scattering (default: 10)
    -o,--open           Open the output after finishing the rendering
    -1,--serial         Render output in a single thread
    --help              Show this help
"#,
                ));
            }
            _ => return Err(arg.unexpected()),
        }
    }
    Ok(Action::Render { path, open, parallel })
}

fn demo_scene() -> Bvh {
    let mut builder = BvhBuilder::default();

    // Ground
    builder.add(Sphere::stationary(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(CheckerTexture::solid(
            0.32,
            Color::new(0.1, 0.1, 0.2),
            Color::new(0.7, 0.7, 0.7),
        )),
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
                let material = Lambertian::solid(albedo);
                let sphere = if random < 0.3 {
                    Sphere::moving(center, center_end, 0.2, material)
                } else {
                    Sphere::stationary(center, 0.2, material)
                };
                builder.add(sphere);
            } else if random < 0.95 {
                // Metal
                let albedo = Color::random(0.5..1.0);
                let fuzz = random_range(0.0..0.5);
                let sphere = Sphere::stationary(center, 0.2, Metal::new(albedo, fuzz));
                builder.add(sphere);
            } else {
                // Glass
                let sphere = Sphere::stationary(center, 0.2, Dielectric::new(1.5));
                builder.add(sphere);
            }
        }
    }

    builder.add(Sphere::stationary(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Dielectric::new(1.5),
    ));
    builder.add(Sphere::stationary(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Lambertian::solid(Color::new(0.4, 0.2, 0.1)),
    ));
    builder.add(Sphere::stationary(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Metal::new(Color::new(0.7, 0.6, 0.5), 0.0),
    ));

    builder.build()
}

fn main() -> io::Result<()> {
    let mut cam = Camera::new()?;
    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.6;
    cam.focus_distance = 10.0;

    match parse_args(&mut cam).map_err(io::Error::other)? {
        Action::Render { path, open, parallel } => {
            let world = demo_scene();
            if parallel {
                cam.render_parallel(&path, &world)?;
            } else {
                cam.render(&path, &world)?;
            }
            if open {
                open::that(&path)?;
            }
        }
        Action::Help(help) => println!("{help}"),
    }
    Ok(())
}
