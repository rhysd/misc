use crate::aabb::{Aabb, Axis};
use crate::interval::Interval;
use crate::object::{Hit, Hittable};
use crate::ray::Ray;
use std::cmp::Ordering;
use std::sync::Arc;

pub type AnyObject = Arc<dyn Hittable>;

// SAH (Surface Area Heuristic)
fn split_bounds_sah(parent: &Aabb, objects: &mut [AnyObject]) -> usize {
    let compare: fn(&AnyObject, &AnyObject) -> Ordering = match parent.longest_axis() {
        Axis::X => |l, r| l.bbox().x().mid().total_cmp(&r.bbox().x().mid()),
        Axis::Y => |l, r| l.bbox().y().mid().total_cmp(&r.bbox().y().mid()),
        Axis::Z => |l, r| l.bbox().z().mid().total_cmp(&r.bbox().z().mid()),
    };
    objects.sort_unstable_by(compare);

    fn cost(idx: usize, objects: &[AnyObject]) -> f64 {
        let (l, r) = objects.split_at(idx);
        let sl: f64 = l.iter().map(|h| h.bbox().surface()).sum();
        let sr: f64 = r.iter().map(|h| h.bbox().surface()).sum();
        sl * l.len() as f64 + sr * r.len() as f64
    }

    // Note: Binned-SAH improves building BVH by 5x faster but it didn't improve entire performance so far
    let len = objects.len();
    (1..len - 1)
        .min_by(|&i, &j| cost(i, objects).total_cmp(&cost(j, objects)))
        .unwrap_or(len / 2)
}

// BVH (Bounding Volume Hierarchy)
pub struct Bvh {
    left: AnyObject,
    right: AnyObject,
    bbox: Aabb,
}

impl Bvh {
    pub fn new(objects: &mut [AnyObject]) -> Self {
        let bbox = objects
            .iter()
            .map(|o| o.bbox())
            .reduce(|a, b| Aabb::new_contained(&a, &b))
            .unwrap();

        let (left, right): (AnyObject, AnyObject) = match objects {
            [] | [_] => panic!("BVH node requires at least two objects"),
            [l, r] => (l.clone(), r.clone()),
            _ => {
                let idx = split_bounds_sah(&bbox, objects);
                match objects.split_at_mut(idx) {
                    ([h], objects) | (objects, [h]) => (Arc::new(Self::new(objects)), h.clone()),
                    (left, right) => (Arc::new(Self::new(left)), Arc::new(Self::new(right))),
                }
            }
        };

        Self { left, right, bbox }
    }
}

impl Hittable for Bvh {
    fn hit(&self, ray: &Ray, mut time: Interval) -> Option<Hit<'_>> {
        if !self.bbox.hit(ray, time) {
            return None;
        }
        let left = self.left.hit(ray, time);
        if let Some(left) = &left {
            time.upper_bound(left.time);
        }
        let right = self.right.hit(ray, time);
        right.or(left)
    }

    fn bbox(&self) -> Aabb {
        self.bbox.clone()
    }
}

#[derive(Default)]
pub struct BvhBuilder {
    objects: Vec<AnyObject>,
}

impl BvhBuilder {
    pub fn add(&mut self, h: impl Hittable + 'static) {
        self.objects.push(Arc::new(h));
    }

    pub fn build(mut self) -> Bvh {
        assert!(self.objects.len() >= 2);
        Bvh::new(&mut self.objects)
    }
}
