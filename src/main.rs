use rand::random;
use std::fs::File;

use raytrace::{
    hittable::{Hittable, HittableList, Sphere},
    types::{unit_vector, Ray, Vec3},
    Camera, Ppm,
};

const MAX_BOUNCE: usize = 10;
fn color(ray: Ray, world: &dyn Hittable, bounce: usize) -> Vec3 {
    if MAX_BOUNCE > bounce {
        if let Some(hit_rec) = world.hit((0.0, f64::MAX), &ray) {
            let target = hit_rec.p + hit_rec.normal + Sphere::random_in_unit_sphere();
            return 0.5 * color(Ray::from(hit_rec.p, target - hit_rec.p), world, bounce + 1);
        }
    }

    let unit_dir = unit_vector(&ray.direction);
    let t = 0.5 * (unit_dir.y() + 1.0);
    return (1.0 - t) * Vec3::from(1.0, 1.0, 1.0) + t * Vec3::from(0.5, 0.7, 1.0);
}

fn main() {
    let width = 200;
    let height = 100;
    let num_samples = 100;
    let mut ppm = Ppm::from(width, height);

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::from(Vec3::from(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::from(Vec3::from(0.0, -100.5, -1.0), 100.0)));

    let camera = Camera::new();
    for y in (0..height).rev() {
        for x in 0..width {
            let mut col = Vec3::new();
            for _ in 0..num_samples {
                let u = (x as f64 + random::<f64>()) / (width as f64);
                let v = (y as f64 + random::<f64>()) / (height as f64);

                let ray = camera.get_ray(u, v);
                col += color(ray, &world, 0);
            }
            col /= num_samples as f64;

            ppm.set_pixel(x, y, col * 255.99);
        }
    }

    let mut file = File::create("output/chapter7-1.ppm").expect("Could not create ppm file");
    ppm.write(&mut file);
}
