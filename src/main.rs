mod camera;
mod constants;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod sphere;
mod vec3;

use hittable::Hittable;
use hittable_list::HittableList;
use image::{Rgb, RgbImage};
use ray::Ray;
use sphere::Sphere;
use std::{f64::INFINITY, sync::Arc};
use vec3::{Color, Point3, Vec3};

use crate::{camera::Camera, interval::Interval};

fn main() {
    // World
    let mut world = HittableList::new();
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let mut cam = Camera::new(16.0 / 9.0, 400);

    let img = cam.render(&world);
    img.save("output.png").unwrap();
}
