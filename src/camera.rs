use rayon::prelude::*;
use std::io::Write;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::{
    constants::{degrees_to_radians, random_double},
    hittable::Hittable,
    interval::Interval,
    material::Material,
    ray::Ray,
    vec3::{Color, Point3, Vec3},
};
use image::RgbImage;

pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub vfov: f32,
    pub lookfrom: Point3,
    pub lookat: Point3,
    pub vup: Vec3,
    pub defocus_angle: f32,
    pub focus_dist: f32,

    image_height: u32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_samples_scale: f32,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f32, image_width: u32) -> Self {
        let samples_per_pixel = 10;
        let max_depth = 50;
        let vfov = 20.0;
        Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            vfov,
            lookfrom: Point3::new(13.0, 2.0, 3.0),
            lookat: Point3::new(0.0, 0.0, 0.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.6,
            focus_dist: 10.0,

            image_height: 0,
            center: Point3::new(0.0, 0.0, 0.0),
            pixel00_loc: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
            pixel_samples_scale: 1.0 / samples_per_pixel as f32,
            u: Vec3::new(0.0, 0.0, 0.0),
            v: Vec3::new(0.0, 0.0, 0.0),
            w: Vec3::new(0.0, 0.0, 0.0),
            defocus_disk_u: Vec3::new(0.0, 0.0, 0.0),
            defocus_disk_v: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    pub fn render(&mut self, world: &dyn Hittable) -> RgbImage {
        self.initialize();

        let width = self.image_width as usize;
        let height = self.image_height as usize;
        let row_stride = width * 3;

        let samples_per_pixel = self.samples_per_pixel;
        let max_depth = self.max_depth;
        let pixel_samples_scale = self.pixel_samples_scale;

        let center = self.center;
        let pixel00_loc = self.pixel00_loc;
        let pixel_delta_u = self.pixel_delta_u;
        let pixel_delta_v = self.pixel_delta_v;

        let defocus_angle = self.defocus_angle;
        let defocus_disk_u = self.defocus_disk_u;
        let defocus_disk_v = self.defocus_disk_v;

        let intensity = Interval::with_bounds(0.0, 0.999);

        let progress = AtomicUsize::new(0);
        let print_lock = Mutex::new(());

        let mut data = vec![0u8; width * height * 3];

        data.par_chunks_mut(row_stride)
            .enumerate()
            .for_each(|(j, row)| {
                for i in 0..width {
                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                    for _ in 0..samples_per_pixel {
                        let offset = Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0);

                        let pixel_sample = pixel00_loc
                            + pixel_delta_u * (i as f32 + offset.x)
                            + pixel_delta_v * (j as f32 + offset.y);

                        let ray_origin = if defocus_angle <= 0.0 {
                            center
                        } else {
                            let p = Vec3::random_in_unit_disk();
                            center + defocus_disk_u * p.x + defocus_disk_v * p.y
                        };

                        let ray_direction = pixel_sample - ray_origin;
                        let ray = Ray::new(ray_origin, ray_direction);

                        pixel_color = pixel_color + Self::ray_color(&ray, max_depth, world);
                    }

                    pixel_color = pixel_color * pixel_samples_scale;

                    let r = Self::linear_to_gamma(pixel_color.x);
                    let g = Self::linear_to_gamma(pixel_color.y);
                    let b = Self::linear_to_gamma(pixel_color.z);

                    let idx = i * 3;
                    row[idx] = (256.0 * intensity.clamp(r)) as u8;
                    row[idx + 1] = (256.0 * intensity.clamp(g)) as u8;
                    row[idx + 2] = (256.0 * intensity.clamp(b)) as u8;
                }

                let done = progress.fetch_add(1, Ordering::Relaxed) + 1;
                {
                    let _guard = print_lock.lock().unwrap();
                    let percent = 100.0 * done as f32 / height as f32;
                    eprint!("\rScanlines done: {}/{} ({:>5.1}%)", done, height, percent);
                    let _ = std::io::stderr().flush();
                }
            });

        eprintln!("\nDone.");

        RgbImage::from_raw(self.image_width, self.image_height, data)
            .expect("RgbImage::from_raw: invalid buffer length")
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f32 / self.aspect_ratio) as u32;
        if self.image_height < 1 {
            self.image_height = 1;
        }

        self.center = self.lookfrom;
        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f32;

        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f32 / self.image_height as f32);

        let w = Vec3::unit_vector(self.lookfrom - self.lookat);
        let u = Vec3::unit_vector(Vec3::cross(self.vup, w));
        let v = Vec3::cross(w, u);

        self.w = w;
        self.u = u;
        self.v = v;

        let viewport_u = u * viewport_width;
        let viewport_v = -v * viewport_height;

        self.pixel_delta_u = viewport_u / self.image_width as f32;
        self.pixel_delta_v = viewport_v / self.image_height as f32;

        let viewport_upper_left =
            self.center - w * self.focus_dist - viewport_u / 2.0 - viewport_v / 2.0;

        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;

        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.0).to_radians().tan();

        self.defocus_disk_u = u * defocus_radius;
        self.defocus_disk_v = v * defocus_radius;
    }

    fn ray_color(r: &Ray, depth: u32, world: &dyn Hittable) -> Color {
        if depth == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut rec = crate::hittable::HitRecord {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
            mat: Material::Lambertian {
                albedo: Vec3::new(0.0, 0.0, 0.0),
            },
        };

        if world.hit(r, Interval::with_bounds(0.001, f32::INFINITY), &mut rec) {
            let mut scattered = Ray::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
            let mut attenuation = Color::new(0.0, 0.0, 0.0);

            if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                return attenuation * Self::ray_color(&scattered, depth - 1, world);
            }
            return Color::new(0.0, 0.0, 0.0);
        }

        let unit_direction = Vec3::unit_vector(r.direction);
        let a = 0.5 * (unit_direction.y + 1.0);
        return Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a;
    }

    pub fn linear_to_gamma(linear_component: f32) -> f32 {
        if linear_component > 0.0 {
            return linear_component.sqrt();
        } else {
            return 0.0;
        }
    }
}
