use crate::color::Color;
use crate::vec3::{Point3, Vec3};

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> Option<f64> {
    let oc = *center - *r.origin(); // C - Q
    let a = r.direction().length_squared();
    let h = r.direction().dot(&oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = h * h - a * c;
    (discriminant >= 0.0).then(|| {
        // Return `t` when the ray hits the sphere for the first time
        (h - discriminant.sqrt()) / a
    })
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
        if let Some(t) = hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, self) {
            let n = (self.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit();
            let v = 0.5 * (n + 1.0);
            return Color::new(v.x(), v.y(), v.z());
        }

        let u = self.direction().unit();
        let a = u.y() / 2.0 + 1.0;
        let v = (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0);
        Color::new(v.x(), v.y(), v.z())
    }
}
