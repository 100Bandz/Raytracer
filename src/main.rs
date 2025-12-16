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
    constants::{random_double, random_double_range},
    interval::Interval,
    material::{Diaelectric, Lambertian, Material, Metal},
};

fn main() {
    // World
    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    // Random small spheres
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if center.subtract(&Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material>;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random().multiply(&Color::random());
                    sphere_material = Arc::new(Lambertian::new(albedo));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_with_inputs(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    sphere_material = Arc::new(Diaelectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    // Three big spheres
    let material1 = Arc::new(Diaelectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    // Camera
    let mut cam = Camera::new(16.0 / 9.0, 500);

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
