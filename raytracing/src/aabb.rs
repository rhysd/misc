use crate::interval::Interval;
use crate::ray::Ray;
use crate::vec3::Point3;

#[inline]
fn minmax(a: f64, b: f64) -> (f64, f64) {
    if a < b { (a, b) } else { (b, a) }
}

// Struct for Axis-Aligned Bounding Box. See 3.3
#[derive(Default, Clone)]
pub struct Aabb {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl Aabb {
    pub fn from_axis(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn from_extrema(a: Point3, b: Point3) -> Self {
        let (xmin, xmax) = minmax(a.x(), b.x());
        let (ymin, ymax) = minmax(a.y(), b.y());
        let (zmin, zmax) = minmax(a.z(), b.z());
        Self {
            x: Interval::new(xmin, xmax),
            y: Interval::new(ymin, ymax),
            z: Interval::new(zmin, zmax),
        }
    }

    pub fn new_contained(a: &Aabb, b: &Aabb) -> Self {
        let x = Interval::new_covered(a.x, b.x);
        let y = Interval::new_covered(a.y, b.y);
        let z = Interval::new_covered(a.z, b.z);
        Self { x, y, z }
    }

    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> bool {
        #[inline]
        fn intersect(ax: Interval, dir: f64, orig: f64, ray_t: Interval) -> bool {
            // Compute the start/end of bounding box of the axis
            let ad_inv = 1.0 / dir;
            let (tmin, tmax) = minmax(
                (ax.min() - orig) * ad_inv, // t0 = (x0 - Qx) / dx
                (ax.max() - orig) * ad_inv, // t1 = (x1 - Qx) / dx
            );
            // Check the intersection of the bounding box and the ray
            let min = ray_t.min().max(tmin);
            let max = ray_t.max().min(tmax);
            min < max
        }

        let orig = ray.origin();
        let dir = ray.direction();

        intersect(self.x, dir.x(), orig.x(), ray_t)
            && intersect(self.y, dir.y(), orig.y(), ray_t)
            && intersect(self.z, dir.z(), orig.z(), ray_t)
    }
}
