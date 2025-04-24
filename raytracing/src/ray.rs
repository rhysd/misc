use crate::hittable::{Face, Hit, Hittable};
use crate::interval::Interval;
use crate::vec3::{Color, Point3, Vec3};

#[derive(Default)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3) -> Self {
        Self { orig, dir }
    }

    pub fn origin(&self) -> &Point3 {
        &self.orig
    }

    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }

    pub fn color<H: Hittable>(&self, world: &H) -> Color {
        if let Some(Hit { normal, .. }) = world.hit(self, Interval::new(0.0, f64::INFINITY)) {
            return 0.5 * (normal + 1.0);
        }

        // Background color is linear gradient
        let u = self.direction().unit();
        let a = u.y() / 2.0 + 1.0;
        (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)
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
