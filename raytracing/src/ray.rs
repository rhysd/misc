use crate::color::Color;
use crate::vec3::{Point3, Vec3};
use std::ops::{Deref, DerefMut, Range};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Face {
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
    fn hit(&self, ray: &Ray, time: Range<f64>) -> Option<Hit>;
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
    fn hit(&self, ray: &Ray, time: Range<f64>) -> Option<Hit> {
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
    fn hit(&self, ray: &Ray, time: Range<f64>) -> Option<Hit> {
        self.0
            .iter()
            .flat_map(|h| h.hit(ray, time.clone()))
            .min_by(|l, r| l.time.partial_cmp(&r.time).unwrap())
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

    pub fn color<H: Hittable>(&self, world: &H) -> Color {
        if let Some(Hit { normal, .. }) = world.hit(self, 0.0..std::f64::INFINITY) {
            let v = 0.5 * (normal + 1.0);
            return Color::new(v.x(), v.y(), v.z());
        }

        let u = self.direction().unit();
        let a = u.y() / 2.0 + 1.0;
        let v = (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0);
        Color::new(v.x(), v.y(), v.z())
    }

    fn face(&self, outward_normal: &Vec3) -> Face {
        // NOTE: `outward_normal` is assumed to have unit length
        if self.dir.dot(outward_normal) < 0.0 {
            Face::Front
        } else {
            Face::Back
        }
    }
}
