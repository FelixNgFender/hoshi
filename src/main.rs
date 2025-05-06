use std::io::{Write, stderr, stdout};

use color::Color;
mod color;
mod point3;
mod vec3;

fn main() {
    let image_width: u32 = 256;
    let image_height: u32 = 256;

    print!("P3\n{image_width} {image_height}\n255\n");

    // lock stdout before entering a print hot loop
    let mut lock = stdout().lock();
    for j in 0..image_height {
        // add a whitespace to overwrite the trailing
        // zero from previous line, e.g., 100 -> 99
        eprint!("\rScanlines remaining: {} ", image_height - j);
        stderr().flush().unwrap();
        for i in 0..image_width {
            // red goes from 0 -> 1 left right
            // green goes from 0 -> 1 top down
            // blue 0
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.0;
            let pixel_color = Color::new(r, g, b);
            let (r, g, b) = pixel_color.to_rgb_bytes();
            writeln!(lock, "{} {} {}", r, g, b).unwrap();
        }
    }

    eprintln!("\rDone.                 ");
}
