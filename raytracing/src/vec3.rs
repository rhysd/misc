use rand::random_range;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Range, Sub, SubAssign};

#[derive(Default, Clone, Copy, PartialEq)]
pub struct Vec3([f64; 3]);

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self([x, y, z])
    }

    pub fn random(range: Range<f64>) -> Self {
        let x = random_range(range.clone());
        let y = random_range(range.clone());
        let z = random_range(range);
        Self::new(x, y, z)
    }

    pub fn random_unit() -> Self {
        loop {
            let p = Self::random(-1.0..1.0);
            let l = p.length_squared();
            // Note: When the random vector is inside the unit sphere
            // Note: Ensure `l.sqrt()` doesn't overflow to 0.0 by checking machine epsilon
            if f64::EPSILON * f64::EPSILON < l && l <= 1.0 {
                break p / l.sqrt();
            }
        }
    }

    pub fn random_in_unit_circle() -> Self {
        loop {
            let x = random_range(-1.0..1.0);
            let y = random_range(-1.0..1.0);
            let p = Self::new(x, y, 0.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn x(&self) -> f64 {
        self.0[0]
    }

    pub fn y(&self) -> f64 {
        self.0[1]
    }

    pub fn z(&self) -> f64 {
        self.0[2]
    }

    pub fn length_squared(&self) -> f64 {
        let (x, y, z) = (self.x(), self.y(), self.z());
        x * x + y * y + z * z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, rhs: &Self) -> f64 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        let x = self.y() * rhs.z() - self.z() * rhs.y();
        let y = self.z() * rhs.x() - self.x() * rhs.z();
        let z = self.x() * rhs.y() - self.y() * rhs.x();
        Self::new(x, y, z)
    }

    pub fn unit(&self) -> Self {
        *self / self.length()
    }

    pub fn is_near_zero(&self) -> bool {
        self.0.iter().all(|f| f.abs() < f64::EPSILON)
    }

    // Mirrored vector reflection. See 10.4
    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        *self - 2.0 * self.dot(normal) * *normal
    }

    // Snell's Law. See 11.2
    pub fn refract(&self, normal: &Vec3, relative_refractive_index: f64) -> Vec3 {
        let cos_theta = self.neg().dot(normal).min(1.0);
        let r_out_perpendicular = relative_refractive_index * (*self + cos_theta * *normal);
        let r_out_parallel = -(1.0 - r_out_perpendicular.length_squared()).abs().sqrt() * *normal;
        r_out_perpendicular + r_out_parallel
    }

    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0);
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.x(), -self.y(), -self.z())
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.0[idx]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.0[idx]
    }
}

impl Add for Vec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let x = self.x() + rhs.x();
        let y = self.y() + rhs.y();
        let z = self.z() + rhs.z();
        Self::new(x, y, z)
    }
}

impl Add<f64> for Vec3 {
    type Output = Self;
    fn add(self, rhs: f64) -> Self::Output {
        let x = self.x() + rhs;
        let y = self.y() + rhs;
        let z = self.z() + rhs;
        Self::new(x, y, z)
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.x() - rhs.x();
        let y = self.y() - rhs.y();
        let z = self.z() - rhs.z();
        Self::new(x, y, z)
    }
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let x = self.x() * rhs.x();
        let y = self.y() * rhs.y();
        let z = self.z() * rhs.z();
        Self::new(x, y, z)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        let x = self.x() * rhs;
        let y = self.y() * rhs;
        let z = self.z() * rhs;
        Self::new(x, y, z)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        let x = self * rhs.x();
        let y = self * rhs.y();
        let z = self * rhs.z();
        Vec3::new(x, y, z)
    }
}

impl Div for Vec3 {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let x = self.x() / rhs.x();
        let y = self.y() / rhs.y();
        let z = self.z() / rhs.z();
        Self::new(x, y, z)
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        let x = self.x() / rhs;
        let y = self.y() / rhs;
        let z = self.z() / rhs;
        Self::new(x, y, z)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;
