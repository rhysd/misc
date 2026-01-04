use crate::aabb::{Aabb, Axis};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::cmp::Ordering;
use std::sync::Arc;

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

#[derive(Default)]
pub struct Hittables {
    objects: Vec<Arc<dyn Hittable>>,
    bbox: Aabb,
}

impl Hittables {
    pub fn add<H: Hittable + 'static>(&mut self, h: H) {
        self.bbox = Aabb::new_contained(&self.bbox, &h.bbox());
        self.objects.push(Arc::new(h));
    }
}

impl Hittable for Hittables {
    fn hit(&self, ray: &Ray, span: Interval) -> Option<Hit<'_>> {
        // Note: We manually reimplement `Iterator::min_by` here because `Iterator::fold` used inside
        // `Iterator::min_by` is much slower than manual `for` loop in this case (more than 2x).
        // ```
        // self.0
        //     .iter()
        //     .flat_map(|h| h.hit(ray, span))
        //     .min_by(|l, r| l.time.partial_cmp(&r.time).unwrap())
        // ```
        // See the following resources for more details:
        // - Commit message: ed1590acc42ac39dadd3f069e74d1c9c4c572437
        // - Assembly comparison: https://gist.github.com/rhysd/c49733ce3086c12bf95edccca99c1641
        let mut nearest: Option<Hit> = None;
        for hit in self.objects.iter().flat_map(|o| o.hit(ray, span)) {
            if nearest.as_ref().is_none_or(|n| hit.time < n.time) {
                nearest = Some(hit);
            }
        }
        nearest
    }

    fn bbox(&self) -> Aabb {
        self.bbox.clone()
    }
}

pub struct Bvh {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: Aabb,
}

impl Bvh {
    pub fn new(objs: &mut [Arc<dyn Hittable>]) -> Self {
        let bbox = objs
            .iter()
            .skip(1)
            .fold(objs[0].bbox(), |acc, obj| Aabb::new_contained(&acc, &obj.bbox()));
        let (left, right): (Arc<dyn Hittable>, Arc<dyn Hittable>) = match objs {
            [] | [_] => panic!("BVH node requires at least two objects"),
            [l, r] => (l.clone(), r.clone()),
            [l, m, r] => {
                let left = l.clone();
                let right = Arc::new(Self {
                    left: m.clone(),
                    right: r.clone(),
                    bbox: Aabb::new_contained(&m.bbox(), &r.bbox()),
                });
                (left, right)
            }
            _ => {
                // Note: Chaning this to partitioning makes building BVH faster (O(n*log(n)) to O(n)) but it causes many edge cases.
                let compare: fn(&Arc<dyn Hittable>, &Arc<dyn Hittable>) -> Ordering = match bbox.longest_axis() {
                    Axis::X => |l, r| l.bbox().x().min().total_cmp(&r.bbox().x().min()),
                    Axis::Y => |l, r| l.bbox().y().min().total_cmp(&r.bbox().y().min()),
                    Axis::Z => |l, r| l.bbox().z().min().total_cmp(&r.bbox().z().min()),
                };
                objs.sort_unstable_by(compare);
                let (left, right) = objs.split_at_mut(objs.len() / 2);
                let left = Arc::new(Self::new(left));
                let right = Arc::new(Self::new(right));
                (left, right)
            }
        };
        Self { left, right, bbox }
    }
}

impl From<Hittables> for Bvh {
    fn from(mut h: Hittables) -> Self {
        Self::new(&mut h.objects)
    }
}

impl Hittable for Bvh {
    fn hit(&self, ray: &Ray, mut time: Interval) -> Option<Hit<'_>> {
        if !self.bbox.hit(ray, time) {
            return None;
        }
        let left = self.left.hit(ray, time);
        if let Some(left) = &left {
            time.clamp_max(left.time);
        }
        let right = self.right.hit(ray, time);
        right.or(left)
    }

    fn bbox(&self) -> Aabb {
        self.bbox.clone()
    }
}
