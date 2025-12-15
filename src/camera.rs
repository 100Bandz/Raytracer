use std::{f64::INFINITY, sync::Arc};

use crate::{
    constants::random_double,
    hittable::{self, Hittable},
    interval::{self, Interval},
    material::Lambertian,
    ray::Ray,
    vec3::{Color, Point3, Vec3},
};
use image::{Rgb, RgbImage};

pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,

    image_height: u32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_samples_scale: f64,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u32) -> Self {
        let samples_per_pixel = 50;
        let max_depth = 25;
        Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            image_height: 0,
            center: Point3::new(0.0, 0.0, 0.0),
            pixel00_loc: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
            pixel_samples_scale: 1.0 / samples_per_pixel as f64,
        }
    }

    pub fn render(&mut self, world: &dyn Hittable) -> RgbImage {
        self.initialize();

        let mut img = RgbImage::new(self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprintln!("\rScanlines remaining: {} ", self.image_height - j);

            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i as f64, j as f64);
                    pixel_color = pixel_color.add(&Self::ray_color(&ray, self.max_depth, world));
                }

                pixel_color = pixel_color.multiply_scalar(self.pixel_samples_scale);

                let r = Self::linear_to_gamma(pixel_color.x);
                let g = Self::linear_to_gamma(pixel_color.y);
                let b = Self::linear_to_gamma(pixel_color.z);

                let intensity = Interval::with_bounds(0.0, 0.999);
                let ir = (256.0 * intensity.clamp(r)) as u8;
                let ig = (256.0 * intensity.clamp(g)) as u8;
                let ib = (256.0 * intensity.clamp(b)) as u8;

                img.put_pixel(i, j, Rgb([ir, ig, ib]));
            }
        }

        eprintln!("\nDone.");
        img
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as u32;
        if self.image_height < 1 {
            self.image_height = 1;
        }

        self.center = Point3::new(0.0, 0.0, 0.0);

        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        self.pixel_delta_u = viewport_u.divide(self.image_width as f64);
        self.pixel_delta_v = viewport_v.divide(self.image_height as f64);

        let viewport_upper_left = self
            .center
            .subtract(&Vec3::new(0.0, 0.0, focal_length))
            .subtract(&viewport_u.divide(2.0))
            .subtract(&viewport_v.divide(2.0));

        self.pixel00_loc = viewport_upper_left.add(
            &self
                .pixel_delta_u
                .add(&self.pixel_delta_v)
                .multiply_scalar(0.5),
        );
    }

    fn ray_color(r: &Ray, depth: u32, world: &dyn Hittable) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        let mut rec = hittable::HitRecord {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
            mat: Arc::new(Lambertian {
                albedo: Vec3::new(0.5, 0.5, 0.5),
            }),
        };

        if world.hit(r, Interval::with_bounds(0.001, INFINITY), &mut rec) {
            let mut scattered = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
            let mut attenuation = Color::new(0.0, 0.0, 0.0);

            if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                return attenuation.multiply(&Self::ray_color(&scattered, depth - 1, world));
            } else {
                return Color::new(0.0, 0.0, 0.0);
            }
        }

        let unit_direction = Vec3::unit_vector(r.direction());
        let t = 0.5 * (unit_direction.y + 1.0);
        Vec3::new(1.0, 1.0, 1.0)
            .multiply_scalar(1.0 - t)
            .add(&Vec3::new(0.5, 0.7, 1.0).multiply_scalar(t))
    }

    fn get_ray(&self, i: f64, j: f64) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc.add(
            &(self.pixel_delta_u.multiply_scalar(i + offset.x))
                .add(&(self.pixel_delta_v.multiply_scalar(j + offset.y))),
        );

        let ray_origin = self.center.clone();
        let ray_direction = pixel_sample.subtract(&self.center);

        return Ray::new(ray_origin, ray_direction);
    }

    fn sample_square() -> Vec3 {
        return Vec3 {
            x: (random_double() - 0.5),
            y: (random_double() - 0.5),
            z: (0.0),
        };
    }

    pub fn linear_to_gamma(linear_component: f64) -> f64 {
        if linear_component > 0.0 {
            return linear_component.sqrt();
        }
        return 0.0;
    }
}
