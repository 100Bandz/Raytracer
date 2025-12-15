mod camera;
mod constants;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod sphere;
mod vec3;

use hittable::Hittable;
use hittable_list::HittableList;
use image::{Rgb, RgbImage};
use ray::Ray;
use sphere::Sphere;
use std::{f64::INFINITY, sync::Arc, time::Instant};
use vec3::{Color, Point3, Vec3};

use crate::{
    camera::Camera,
    interval::Interval,
    material::{Lambertian, Metal},
};

fn main() {
    // World
    let mut world = HittableList::new();

    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    // Camera
    let mut cam = Camera::new(16.0 / 9.0, 400);

    let start_time = Instant::now();

    let img = cam.render(&world);
    let elapsed = start_time.elapsed();
    println!(
        "Render complete. Time elapsed: {:.2?} (≈ {:.1} minutes)",
        elapsed,
        elapsed.as_secs_f64() / 60.0
    );
    img.save("output.png").unwrap();
}
