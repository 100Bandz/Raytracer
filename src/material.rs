use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec3::{Color, Vec3},
};
use std::sync::Arc;

/// Trait for materials
pub trait Material: Send + Sync {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        false
    }
}

/// Lambertian diffuse material
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal.add(&Vec3::random_unit_vector());

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal.clone();
        }

        *scattered = Ray::new(rec.p.clone(), scatter_direction);
        *attenuation = self.albedo.clone();
        true
    }
}

// Metal material
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Vec3::reflect(r_in.direction().clone(), rec.normal.clone());
        let reflected = Vec3::unit_vector(&reflected)
            .add(&Vec3::random_unit_vector().multiply_scalar(self.fuzz));
        *scattered = Ray::new(rec.p.clone(), reflected);
        *attenuation = self.albedo.clone();

        return Vec3::dot(scattered.direction(), &rec.normal) > 0.0;
    }
}
