use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Lambertian;
use crate::ray::Ray;
use crate::vec3::Vec3;
use std::sync::Arc;

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord {
            p: crate::vec3::Point3::new(0.0, 0.0, 0.0),
            normal: crate::vec3::Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
            mat: Arc::new(Lambertian {
                albedo: Vec3::new(0.5, 0.5, 0.5),
            }),
        };
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if object.hit(
                r,
                Interval::with_bounds(ray_t.min, closest_so_far),
                &mut temp_rec,
            ) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        return hit_anything;
    }
}
