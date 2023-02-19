use std::fs::File;

use raytrace::{Ppm, types::{Vec3, unit_vector, Ray}, hittable::{Sphere, Hittable, HittableList}};

fn color(ray: Ray, world: &dyn Hittable) -> Vec3 {
    if let Some(hit_rec) = world.hit((0.0, f64::MAX), &ray) {
        let normal = hit_rec.normal;
        return 0.5 * Vec3::from(normal.x()+1.0, normal.y()+1.0, normal.z()+1.0);
    } else {
        let unit_dir = unit_vector(&ray.direction);
        let t = 0.5 * (unit_dir.y() + 1.0);
        return (1.0 - t) * Vec3::from(1.0, 1.0, 1.0) + t * Vec3::from(0.5, 0.7, 1.0);
    }
}

fn main() {
    let width = 200;
    let height = 100;
    let mut ppm = Ppm::from(width, height);

    let lower_left_corner = Vec3::from(-2.0, -1.0, -1.0);
    let horizontal = Vec3::from(4.0, 0.0, 0.0);
    let vertical = Vec3::from(0.0, 2.0, 0.0);
    let origin = Vec3::from(0.0, 0.0, 0.0);

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::from(Vec3::from(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::from(Vec3::from(0.0, -100.5, -1.0), 100.0)));
    for y in (0..height).rev() {
        for x in 0..width {
            let u = (x as f64) / (width as f64);
            let v = (y as f64) / (height as f64);

            let r = Ray::from(origin, lower_left_corner + u * horizontal + v * vertical);
            let col = color(r, &world);

            ppm.set_pixel(x, y, col * 255.99);
        }
    }

    let mut file = File::create("output/chapter5-2.ppm").expect("Could not create ppm file");
    ppm.write(&mut file);
}
