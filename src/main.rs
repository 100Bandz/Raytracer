use image::{Rgb, RgbImage};

fn main() {
    let image_width = 256;
    let image_height = 256;

    let mut img = RgbImage::new(image_width, image_height);

    for y in 0..image_height {
        for x in 0..image_width {
            let r = x as f64 / (image_width - 1) as f64;
            let g = y as f64 / (image_height - 1) as f64;
            let b = 0.0;

            let ir = (255.999 * r) as u8;
            let ig = (255.999 * g) as u8;
            let ib = (255.999 * b) as u8;

            img.put_pixel(x, y, Rgb([ir, ig, ib]));
        }
    }

    img.save("output.png").unwrap();
}
