use std::{fs::File, io::Read};

use raytrace::Ppm;

#[test]
fn create_and_write() {
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

    let mut output = Vec::new();
    ppm.write(&mut output);

    let mut expected_file = File::open("output/test-expected.ppm").expect("Could not open expected file");
    let mut expected_output = Vec::new();
    expected_file.read_to_end(&mut expected_output).expect("Could not read expected file");

    assert!(output == expected_output);
}
