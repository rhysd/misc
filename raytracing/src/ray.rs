use crate::hittable::{Face, Hittable};
use crate::interval::Interval;
use crate::vec3::{Color, Point3, Vec3};

#[derive(Default)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
    time: f64, // The time when the ray is generated
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Self::new_at(0.0, orig, dir)
    }

    pub fn new_at(time: f64, orig: Point3, dir: Vec3) -> Self {
        Self { orig, dir, time }
    }

    pub fn origin(&self) -> &Point3 {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }

    pub fn color<H: Hittable>(&self, depth: u8, world: &H) -> Color {
        if depth == 0 {
            return Color::ZERO;
        }

        // Note: Use 0.001 to avoid the ray reflects just after the diffusion due to floating point round error.
        if let Some(hit) = world.hit(self, Interval::new(0.001, f64::INFINITY)) {
            if let Some((scattered, attenuation)) = hit.mat.scatter(self, &hit) {
                return attenuation * scattered.color(depth - 1, world);
            }
            return Color::ZERO;
        }

        // Background color is linear gradient
        let u = self.direction().unit();
        let a = 0.5 * (u.y() + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }

    pub fn face(&self, outward_normal: &Vec3) -> Face {
        // NOTE: `outward_normal` is assumed to have unit length
        if self.dir.dot(outward_normal) < 0.0 {
            Face::Front
        } else {
            Face::Back
        }
    }
}
