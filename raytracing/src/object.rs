use crate::aabb::Aabb;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

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
    pub u: f64,
    pub v: f64,
}

pub trait Hittable: Sync + Send {
    fn hit(&self, ray: &Ray, time: Interval) -> Option<Hit<'_>>;
    fn bbox(&self) -> Aabb;
}

pub struct Sphere<M> {
    center: Ray,
    radius: f64,
    bbox: Aabb,
    mat: M,
}

impl<M> Sphere<M> {
    pub fn stationary(center: Point3, radius: f64, mat: M) -> Self {
        let radvec = Vec3::new(radius, radius, radius);
        Self {
            center: Ray::new(center, Vec3::ZERO),
            radius: radius.max(0.0),
            bbox: Aabb::from_extrema(center - radvec, center + radvec),
            mat,
        }
    }

    pub fn moving(from: Point3, to: Point3, radius: f64, mat: M) -> Self {
        let center = Ray::new(from, to - from);
        let radvec = Vec3::new(radius, radius, radius);

        let center0 = center.at(0.0);
        let bbox0 = Aabb::from_extrema(center0 - radvec, center0 + radvec);

        let center1 = center.at(1.0);
        let bbox1 = Aabb::from_extrema(center1 - radvec, center1 + radvec);

        Self {
            center,
            radius: radius.max(0.0),
            bbox: Aabb::new_contained(&bbox0, &bbox1),
            mat,
        }
    }
}

impl<M: Material> Hittable for Sphere<M> {
    fn hit(&self, ray: &Ray, time: Interval) -> Option<Hit<'_>> {
        let center = self.center.at(ray.time());
        let oc = center - *ray.origin(); // C - Q
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
        let outward_normal = (pos - center) / self.radius;
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
            u: 0.0, // TODO
            v: 0.0, // TODO
        })
    }

    fn bbox(&self) -> Aabb {
        self.bbox.clone()
    }
}
