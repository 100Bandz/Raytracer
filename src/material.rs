use crate::{constants::random_double, hittable::HitRecord, ray::Ray, vec3::*};

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f32 },
    Dielectric { refraction_index: f32 },
}

impl Material {
    #[inline]
    pub fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        match self {
            Material::Lambertian { albedo } => {
                let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

                if scatter_direction.near_zero() {
                    scatter_direction = rec.normal;
                }

                *scattered = Ray::new(rec.p, scatter_direction);
                *attenuation = *albedo;
                true
            }

            Material::Metal { albedo, fuzz } => {
                let reflected = Vec3::reflect(Vec3::unit_vector(r_in.direction), rec.normal);
                let reflected = Vec3::unit_vector(reflected) + Vec3::random_unit_vector() * *fuzz;
                *scattered = Ray::new(rec.p, reflected);
                *attenuation = *albedo;
                Vec3::dot(scattered.direction, rec.normal) > 0.0
            }

            Material::Dielectric { refraction_index } => {
                *attenuation = Color::new(1.0, 1.0, 1.0);

                let ri = if rec.front_face {
                    1.0 / refraction_index
                } else {
                    *refraction_index
                };

                let unit_direction = Vec3::unit_vector(r_in.direction);
                let cos_theta = Vec3::dot(-unit_direction, rec.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let cannot_refract = ri * sin_theta > 1.0;
                let direction = if cannot_refract || reflectance(cos_theta, ri) > random_double() {
                    Vec3::reflect(unit_direction, rec.normal)
                } else {
                    Vec3::refract(unit_direction, rec.normal, ri)
                };

                *scattered = Ray::new(rec.p, direction);
                
                true
            }
        }
    }
}

#[inline]
fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r0 = r0 * r0;
    return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
}
