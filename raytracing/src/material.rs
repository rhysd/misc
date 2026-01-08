use crate::object::{Face, Hit};
use crate::ray::Ray;
use crate::texture::{SolidColor, Texture};
use crate::vec3::{Color, Vec3};
use rand::random_range;

pub trait Material: Sync + Send {
    fn scatter(&self, ray: &Ray, hit: &Hit<'_>) -> Option<(Ray, Color)>;
}

// Lambertian (diffuse) reflectance
pub struct Lambertian<T: Sync + Send> {
    tex: T,
}

impl Lambertian<SolidColor> {
    pub fn solid(albedo: Color) -> Self {
        let tex = SolidColor::new(albedo);
        Self { tex }
    }
}

impl<T: Texture> Lambertian<T> {
    pub fn new(tex: T) -> Self {
        Self { tex }
    }
}

impl<T: Texture> Material for Lambertian<T> {
    fn scatter(&self, ray: &Ray, hit: &Hit<'_>) -> Option<(Ray, Color)> {
        // Diffuse the ray around the normal (the Lambertian reflection)
        let mut scatter_direction = hit.normal + Vec3::random_unit();

        // Note: If the random unit vector we generate is exactly opposite the normal vector, the two
        // will sum to zero, which will result in a zero scatter direction vector. This leads to bad
        // scenarios later on (infinities and NaNs).
        if scatter_direction.is_near_zero() {
            scatter_direction = hit.normal;
        }

        let scattered = Ray::new_at(ray.time(), hit.pos, scatter_direction);
        let attenuation = self.tex.color(hit.u, hit.v, &hit.pos);
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
        let scattered = Ray::new_at(ray.time(), hit.pos, reflected);

        // When dot-product is negative, that means the unit vector is inside the hemisphere
        // and it is incorrect as a reflection of ray.
        if scattered.direction().dot(&hit.normal) <= 0.0 {
            return None;
        }

        let attenuation = self.albedo;
        Some((scattered, attenuation))
    }
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }
}

fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    // Use Schlick's approximation for reflectance (11.4)
    let r0 = ((1.0 - refraction_index) / (1.0 + refraction_index)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &Hit<'_>) -> Option<(Ray, Color)> {
        // Note: Outside objects is vacuum
        let refraction_index = if hit.face == Face::Front {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };
        let unit_direction = ray.direction().unit();
        let cos_theta = (-unit_direction).dot(&hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let direction = if refraction_index * sin_theta > 1.0
            || reflectance(cos_theta, refraction_index) > random_range(0.0..1.0)
        {
            // Cannot refract. Yield total internal reflection (11.3)
            unit_direction.reflect(&hit.normal)
        } else {
            unit_direction.refract(&hit.normal, refraction_index)
        };

        let scattered = Ray::new_at(ray.time(), hit.pos, direction);
        let attenuation = Color::new(1.0, 1.0, 1.0);
        Some((scattered, attenuation))
    }
}
