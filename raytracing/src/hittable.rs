use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::ops::{Deref, DerefMut};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Face {
    Front,
    Back,
}

pub struct Hit<'a> {
    pub pos: Point3,
    pub normal: Vec3,
    pub time: f64,
    pub face: Face,
    pub mat: &'a dyn Material,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, time: Interval) -> Option<Hit<'_>>;
}

pub struct Sphere<M> {
    center: Point3,
    radius: f64,
    mat: M,
}

impl<M> Sphere<M> {
    pub fn new(center: Point3, radius: f64, mat: M) -> Self {
        Self {
            center,
            radius: radius.max(0.0),
            mat,
        }
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, ray: &Ray, time: Interval) -> Option<Hit<'_>> {
        let oc = self.center - *ray.origin(); // C - Q
        let a = ray.direction().length_squared();
        let h = ray.direction().dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let d = discriminant.sqrt();
        let time = [(h - d) / a, (h + d) / a].into_iter().find(|&t| time.surrounds(t))?;
        let pos = ray.at(time);
        let outward_normal = (pos - self.center) / self.radius;
        let face = ray.face(&outward_normal);
        let normal = match face {
            Face::Front => outward_normal,
            Face::Back => -outward_normal,
        };

        Some(Hit {
            time,
            pos,
            normal,
            face,
            mat: &self.mat,
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
    fn hit(&self, ray: &Ray, span: Interval) -> Option<Hit<'_>> {
        let mut nearest: Option<Hit> = None;
        for hit in self.0.iter().flat_map(|h| h.hit(ray, span)) {
            if nearest.as_ref().is_none_or(|n| hit.time < n.time) {
                nearest = Some(hit);
            }
        }
        nearest
    }
}
