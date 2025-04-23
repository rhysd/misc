use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

#[derive(Default, Clone, Copy, PartialEq)]
pub struct Vec3([f64; 3]);

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self([x, y, z])
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
        let z = self.x() * rhs;
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
