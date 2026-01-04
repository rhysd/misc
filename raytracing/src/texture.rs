use crate::vec3::{Color, Point3};

pub trait Texture {
    fn color(&self, u: f64, v: f64, p: &Point3) -> Color;
}

pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Texture for SolidColor {
    fn color(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.albedo
    }
}

pub struct CheckerTexture<T, U> {
    inv_scale: f64,
    even: T,
    odd: U,
}

impl CheckerTexture<SolidColor, SolidColor> {
    pub fn solid(scale: f64, even: Color, odd: Color) -> Self {
        Self::new(scale, SolidColor::new(even), SolidColor::new(odd))
    }
}

impl<T: Texture, U: Texture> CheckerTexture<T, U> {
    pub fn new(scale: f64, even: T, odd: U) -> Self {
        let inv_scale = 1.0 / scale;
        Self { inv_scale, even, odd }
    }
}

impl<T: Texture, U: Texture> Texture for CheckerTexture<T, U> {
    fn color(&self, u: f64, v: f64, p: &Point3) -> Color {
        let x = (p.x() * self.inv_scale).floor() as i32;
        let y = (p.y() * self.inv_scale).floor() as i32;
        let z = (p.z() * self.inv_scale).floor() as i32;

        if (x + y + z) % 2 == 0 {
            self.even.color(u, v, p)
        } else {
            self.odd.color(u, v, p)
        }
    }
}
