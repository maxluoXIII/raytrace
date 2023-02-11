use std::fs::File;

use raytrace::{Ppm, Vec3};

fn main() {
    let width = 200;
    let height = 100;
    let mut ppm = Ppm::from(width, height);
    for y in (0..height).rev() {
        for x in 0..width {
            let col = Vec3::from(
                (x as f64) / (width as f64),
                (y as f64) / (height as f64),
                0.2,
            );
            let ir = (col.r() * 255.99) as u32;
            let ig = (col.g() * 255.99) as u32;
            let ib = (col.b() * 255.99) as u32;
            ppm.set_pixel(x, y, (ir, ig, ib));
        }
    }

    let mut file = File::create("output/chapter2.ppm").expect("Could not create ppm file");
    ppm.write(&mut file);
}
