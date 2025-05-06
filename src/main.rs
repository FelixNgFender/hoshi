use std::io::{Write, stderr, stdout};

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
            let r = i as f32 / (image_width - 1) as f32;
            let g = j as f32 / (image_height - 1) as f32;
            let b = 0.0;

            // [0, 1] is only internal representation
            // have to scale to [0, 255] before outputting
            // use 255.999 to approximate: values near to 1
            // map to 255 due to truncation and 1 still map
            // to 255.
            let ir = (r * 255.999) as u32;
            let ig = (g * 255.999) as u32;
            let ib = (b * 255.999) as u32;

            writeln!(lock, "{ir} {ig} {ib}").unwrap();
        }
    }

    eprintln!("\rDone.                 ");
}
