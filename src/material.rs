use crate::{
    constants::random_double, hittable::HitRecord, ray::Ray, vec3::{Color, Vec3}
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


// Diaelectric material
pub struct Diaelectric {
    refraction_index: f64,
}

impl Diaelectric {
    pub fn new(refraction_index: f64) -> Self {
        Self {
            refraction_index
        }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        return r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
    
}

impl Material for Diaelectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray,) -> bool {

        *attenuation =  Color::new(1.0, 1.0, 1.0);

        let ri = if rec.front_face { 1.0 / self.refraction_index } else { self.refraction_index };

        let unit_direction = Vec3::unit_vector(&r_in.direction);

        let cos_theta = (-Vec3::dot(&unit_direction, &rec.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannon_refract = ri * sin_theta > 1.0;

        let direction;

        if cannon_refract || Self::reflectance(cos_theta, ri) > random_double() {
            direction = Vec3::reflect(unit_direction, rec.normal.clone());
        }
        else {
            direction = Vec3::refract(unit_direction, rec.normal.clone(), ri);
        }
        
        *scattered = Ray::new(rec.p.clone(), direction);
        return true;
    }

}
