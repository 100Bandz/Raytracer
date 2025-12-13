mod ray;
mod vec3;

use image::{Rgb, RgbImage};
use ray::Ray;
use vec3::Vec3;

type Color = Vec3;
type Point3 = Vec3;

fn ray_color(r: &Ray) -> Color {
    let unit_direction = Vec3::unit_vector(r.direction());
    let a = 0.5 * (unit_direction.get_y() + 1.0);

    Color::new(1.0, 1.0, 1.0)
        .multiply_scalar(1.0 - a)
        .add(Color::new(0.5, 0.7, 1.0).multiply_scalar(a))
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 400;
    let mut image_height = (image_width as f64 / aspect_ratio) as u32;
    if image_height < 1 {
        image_height = 1;
    }

    let mut img = RgbImage::new(image_width, image_height);

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Viewport vectors
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Pixel deltas
    let pixel_delta_u = viewport_u.divide(image_width as f64);
    let pixel_delta_v = viewport_v.divide(image_height as f64);

    // Upper left pixel
    let viewport_upper_left = camera_center
        .subtract(Vec3::new(0.0, 0.0, focal_length))
        .subtract(viewport_u.divide(2.0))
        .subtract(viewport_v.divide(2.0));

    let pixel00_loc =
        viewport_upper_left.add(pixel_delta_u.add(pixel_delta_v).multiply_scalar(0.5));

    // Render
    for y in 0..image_height {
        eprintln!("Scanlines remaining: {}", image_height - y);
        for x in 0..image_width {
            let pixel_center = pixel00_loc
                .add(pixel_delta_u.multiply_scalar(x as f64))
                .add(pixel_delta_v.multiply_scalar(y as f64));

            let ray_direction = pixel_center.subtract(camera_center);
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&r);

            let ir = (255.999 * pixel_color.get_x()) as u8;
            let ig = (255.999 * pixel_color.get_y()) as u8;
            let ib = (255.999 * pixel_color.get_z()) as u8;

            img.put_pixel(x, y, Rgb([ir, ig, ib]));
        }
    }

    img.save("output.png").unwrap();
    eprintln!("Done.");
}
