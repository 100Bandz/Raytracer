use std::cmp::min;

use crate::constants::{random_double, random_double_range};

#[derive(Clone, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        return self.x.abs() < s && self.y.abs() < s && self.z.abs() < s;
    }

    pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
        u.x * v.x + u.y * v.y + u.z * v.z
    }

    pub fn unit_vector(v: &Vec3) -> Vec3 {
        v.clone().divide(v.length())
    }

    pub fn random_unit_vector() -> Vec3 {
        loop {
            let p = Vec3::random_with_inputs(-1.0, 1.0);
            let lensq = p.length_squared();
            if 1e-160 < lensq && lensq <= 1.0 {
                return p.divide(lensq.sqrt());
            }
        }
    }

    pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = Vec3::dot(&uv.negate(), &n).min(1.0);

        let r_out_perp = (n.multiply_scalar(cos_theta).add(&uv)).multiply_scalar(etai_over_etat);
        let r_out_parallel = n.multiply_scalar(-((1.0 - r_out_perp.length_squared()).abs().sqrt()));

        return r_out_parallel.add(&r_out_perp);
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        return v.subtract(&n.multiply_scalar(2.0 * Self::dot(&v, &n)));
    }

    pub fn add(&self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }

    pub fn subtract(&self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }

    pub fn multiply_scalar(&self, t: f64) -> Vec3 {
        Vec3::new(self.x * t, self.y * t, self.z * t)
    }

    pub fn multiply(&self, other: &Vec3) -> Vec3 {
        Vec3::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }

    pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
        Vec3::new(
            u.y * v.z - u.z * v.y,
            u.z * v.x - u.x * v.z,
            u.x * v.y - u.y * v.x,
        )
    }

    pub fn divide(&self, t: f64) -> Vec3 {
        self.multiply_scalar(1.0 / t)
    }

    pub fn negate(&self) -> Vec3 {
        Vec3::new(-self.x, -self.y, -self.z)
    }

    pub fn random() -> Vec3 {
        return Vec3::new(random_double(), random_double(), random_double());
    }

    pub fn random_with_inputs(min: f64, max: f64) -> Vec3 {
        return Vec3::new(
            random_double_range(min, max),
            random_double_range(min, max),
            random_double_range(min, max),
        );
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit_vector();
        if Vec3::dot(&on_unit_sphere, normal) > 0.0 {
            return on_unit_sphere;
        } else {
            return on_unit_sphere.negate();
        }
    }

    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::new(random_double_range(-1.0, 1.0),
             random_double_range(-1.0, 1.0), 
             0.0);

            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;
