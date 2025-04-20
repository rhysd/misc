use crate::color::Color;
use crate::vec3::{Point3, Vec3};
use std::ops::RangeBounds;

pub struct Hit {
    pub pos: Point3,
    pub normal: Vec3,
    pub time: f64,
}

pub trait Hittable {
    fn hit<R: RangeBounds<f64>>(&self, ray: &Ray, time: R) -> Option<Hit>;
}

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Self {
            center,
            radius: radius.max(0.0),
        }
    }
}

impl Hittable for Sphere {
    fn hit<R: RangeBounds<f64>>(&self, ray: &Ray, time: R) -> Option<Hit> {
        let oc = self.center - *ray.origin(); // C - Q
        let a = ray.direction().length_squared();
        let h = ray.direction().dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let d = discriminant.sqrt();
        let time = [h - d, h + d].into_iter().find(|t| time.contains(t))?;
        let pos = ray.at(time);
        let normal = (pos - self.center) / self.radius;

        Some(Hit { time, pos, normal })
    }
}

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

    pub fn color(&self) -> Color {
        let sphere = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);
        if let Some(Hit { time, .. }) = sphere.hit(self, 0.0..100.0) {
            let n = (self.at(time) - Vec3::new(0.0, 0.0, -1.0)).unit();
            let v = 0.5 * (n + 1.0);
            return Color::new(v.x(), v.y(), v.z());
        }

        let u = self.direction().unit();
        let a = u.y() / 2.0 + 1.0;
        let v = (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0);
        Color::new(v.x(), v.y(), v.z())
    }
}
