use std::fs::File;

use raytrace::{Ppm, types::{Vec3, unit_vector, Ray}};

fn bg_color(ray: Ray) -> Vec3 {
    let unit_dir = unit_vector(&ray.direction);
    let t = 0.5 * (unit_dir.y() + 1.0);
    return (1.0 - t) * Vec3::from(1.0, 1.0, 1.0) + t * Vec3::from(0.5, 0.7, 1.0);
}

fn main() {
    let width = 200;
    let height = 100;
    let mut ppm = Ppm::from(width, height);

    let lower_left_corner = Vec3::from(-2.0, -1.0, -1.0);
    let horizontal = Vec3::from(4.0, 0.0, 0.0);
    let vertical = Vec3::from(0.0, 2.0, 0.0);
    let origin = Vec3::from(0.0, 0.0, 0.0);
    for y in (0..height).rev() {
        for x in 0..width {
            let u = (x as f64) / (width as f64);
            let v = (y as f64) / (height as f64);

            let r = Ray::from(origin, lower_left_corner + u * horizontal + v * vertical);
            let col = bg_color(r);

            ppm.set_pixel(x, y, col * 255.99);
        }
    }

    let mut file = File::create("output/chapter3.ppm").expect("Could not create ppm file");
    ppm.write(&mut file);
}
