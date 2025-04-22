use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::ops::{Deref, DerefMut};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Face {
    Front,
    Back,
}

pub struct Hit {
    pub pos: Point3,
    pub normal: Vec3,
    pub time: f64,
    pub face: Face,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, time: Interval) -> Option<Hit>;
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
    fn hit(&self, ray: &Ray, time: Interval) -> Option<Hit> {
        let oc = self.center - *ray.origin(); // C - Q
        let a = ray.direction().length_squared();
        let h = ray.direction().dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let d = discriminant.sqrt();
        let time = [h - d, h + d].into_iter().find(|&t| time.surrounds(t))?;
        let pos = ray.at(time);
        let normal = (pos - self.center) / self.radius;
        let face = ray.face(&normal);

        Some(Hit {
            time,
            pos,
            normal,
            face,
        })
    }
}

#[derive(Default)]
pub struct Hittables(Vec<Box<dyn Hittable>>);

impl Hittables {
    pub fn add<H: Hittable + 'static>(&mut self, h: H) {
        self.0.push(Box::new(h));
    }
}

impl Deref for Hittables {
    type Target = Vec<Box<dyn Hittable>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Hittables {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Hittable for Hittables {
    fn hit(&self, ray: &Ray, span: Interval) -> Option<Hit> {
        self.0
            .iter()
            .flat_map(|h| h.hit(ray, span))
            .min_by(|l, r| l.time.partial_cmp(&r.time).unwrap())
    }
}
