mod color;
mod point3;
mod ray;
mod vector3;

use std::io::{Write, stderr, stdout};

use point3::Point3;
use ray::Ray;
use vector3::Vector3;

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 400;

    // height has to be at least 1
    let image_height = ((image_width as f64 / aspect_ratio) as u32).max(1);

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    // aspect_ratio is the ideal ratio. for viewport proportions to exactly match the integer-based
    // image proportions, scale to actual image proportions
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // span the viewport dimensions
    let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);

    // distance between pixels
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // location of upper left pixel
    let viewport_upper_left =
        camera_center - Vector3::new(0.0, 0.0, focal_length) - viewport_v / 2.0 - viewport_u / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render
    print!("P3\n{image_width} {image_height}\n255\n");

    // lock stdout before entering a print hot loop
    let mut lock = stdout().lock();
    for j in 0..image_height {
        // add a whitespace to overwrite the trailing
        // zero from previous line, e.g., 100 -> 99
        eprint!("\rScanlines remaining: {} ", image_height - j);
        stderr().flush().unwrap();
        for i in 0..image_width {
            let pixel_center = pixel00_loc + pixel_delta_u * i as f64 + pixel_delta_v * j as f64;
            let ray_direction = pixel_center - camera_center;
            let pixel_color = &Ray::new(camera_center, ray_direction).ray_color();

            let (r, g, b) = pixel_color.to_rgb_bytes();
            writeln!(lock, "{} {} {}", r, g, b).unwrap();
        }
    }

    eprintln!("\rDone.                 ");
}
