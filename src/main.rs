use std::fs::File;

use raytrace::Ppm;

fn main() {
    let width = 200;
    let height = 100;
    let mut ppm = Ppm::from(width, height);
    for y in (0..height).rev() {
        for x in 0..width {
            let r = ((x as f64) / (width as f64) * 255.99) as u32;
            let g = ((y as f64) / (height as f64) * 255.99) as u32;
            let b = (0.2 * 255.99) as u32;
            ppm.set_pixel(x, y, (r, g, b));
        }
    }

    let mut file = File::create("output/test-expected.ppm").expect("Could not create ppm file");
    ppm.write(&mut file);
}
