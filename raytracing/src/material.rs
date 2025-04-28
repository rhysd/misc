use crate::hittable::Hit;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &Hit<'_>) -> Option<(Ray, Color)>;
}

// Lambertian (diffuse) reflectance
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &Hit<'_>) -> Option<(Ray, Color)> {
        // Diffuse the ray around the normal (the Lambertian reflection)
        let mut scatter_direction = hit.normal + Vec3::random_unit();

        // Note: If the random unit vector we generate is exactly opposite the normal vector, the two
        // will sum to zero, which will result in a zero scatter direction vector. This leads to bad
        // scenarios later on (infinities and NaNs).
        if scatter_direction.is_near_zero() {
            scatter_direction = hit.normal;
        }

        let scattered = Ray::new(hit.pos, scatter_direction);
        let attenuation = self.albedo;
        Some((scattered, attenuation))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        let fuzz = fuzz.clamp(0.0, 1.0);
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &Hit<'_>) -> Option<(Ray, Color)> {
        let fuzz = self.fuzz * Vec3::random_unit();
        let reflected = ray.direction().reflect(&hit.normal) + fuzz;
        let scattered = Ray::new(hit.pos, reflected);

        // When dot-product is negative, that means the unit vector is inside the hemisphere
        // and it is incorrect as a reflection of ray.
        if scattered.direction().dot(&hit.normal) <= 0.0 {
            return None;
        }

        let attenuation = self.albedo;
        Some((scattered, attenuation))
    }
}
