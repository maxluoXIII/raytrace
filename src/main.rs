use rand::random;
use std::{fs::File, rc::Rc};

use raytrace::{
    hittable::{Hittable, HittableList, Sphere},
    material::{Dielectric, Lambertian, Metal},
    types::{Ray, Vec3},
    Camera, Ppm,
};

fn color(ray: Ray, world: &dyn Hittable, depth: usize) -> Vec3 {
    if let Some(hit_rec) = world.hit((0.001, f64::MAX), &ray) {
        if depth < 50 {
            if let Some((scattered, attenuation)) = hit_rec.mat.scatter(&ray, &hit_rec) {
                return attenuation * color(scattered, world, depth + 1);
            }
        }

        return Vec3::from(0.0, 0.0, 0.0);
    } else {
        let unit_dir = Vec3::unit_vector(&ray.direction);
        let t = 0.5 * (unit_dir.y() + 1.0);
        return (1.0 - t) * Vec3::from(1.0, 1.0, 1.0) + t * Vec3::from(0.5, 0.7, 1.0);
    }
}

fn main() {
    let width = 200;
    let height = 100;
    let num_samples = 100;
    let mut ppm = Ppm::from(width, height);

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::from(
        Vec3::from(0.0, 0.0, -1.0),
        0.5,
        Rc::new(Lambertian::from(Vec3::from(0.8, 0.3, 0.3))),
    )));
    world.add(Box::new(Sphere::from(
        Vec3::from(0.0, -100.5, -1.0),
        100.0,
        Rc::new(Lambertian::from(Vec3::from(0.8, 0.8, 0.0))),
    )));
    world.add(Box::new(Sphere::from(
        Vec3::from(1.0, 0.0, -1.0),
        0.5,
        Rc::new(Metal::from(Vec3::from(0.8, 0.6, 0.2), 0.0)),
    )));
    world.add(Box::new(Sphere::from(
        Vec3::from(-1.0, 0.0, -1.0),
        0.5,
        Rc::new(Dielectric::from(1.5)),
    )));
    world.add(Box::new(Sphere::from(
        Vec3::from(-1.0, 0.0, -1.0),
        -0.45,
        Rc::new(Dielectric::from(1.5)),
    )));

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
            col = Vec3::from(f64::sqrt(col.x()), f64::sqrt(col.y()), f64::sqrt(col.z()));

            ppm.set_pixel(x, y, col * 255.99);
        }
    }

    let mut file = File::create("output/chapter9-2.ppm").expect("Could not create ppm file");
    ppm.write(&mut file);
}
