use rand::random;
use rayon::prelude::*;
use std::{
    fs::File,
    sync::Arc,
    thread,
    time::{Duration, SystemTime},
};

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

        return Vec3::default();
    } else {
        let unit_dir = Vec3::unit_vector(&ray.direction);
        let t = 0.5 * (unit_dir.y() + 1.0);
        return (1.0 - t) * Vec3::from((1.0, 1.0, 1.0)) + t * Vec3::from((0.5, 0.7, 1.0));
    }
}

fn main() {
    let aspect_ratio = 3. / 2.;
    let width = 1200;
    let height = f64::round(width as f64 / aspect_ratio) as usize;
    let num_samples = 500;
    let mut ppm = Ppm::from(width, height);

    let world = random_scene();

    let look_from = Vec3::from((13., 2., 3.));
    let look_at = Vec3::from((0., 0., 0.));
    let dist_to_focus = 10.;
    let aperture = 0.1;
    let camera = Camera::new(
        look_from,
        look_at,
        Vec3::from((0., 1., 0.)),
        20.,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    let start = SystemTime::now();

    let num_threads: usize = thread::available_parallelism().unwrap().into();
    let samples_per_thread = f64::round(num_samples as f64 / num_threads as f64) as usize;
    let actual_total_samples = num_threads * samples_per_thread;
    println!("Using {num_threads} threads to calculate {samples_per_thread} samples per thread");
    println!("Total samples: {actual_total_samples}");
    // let num_threads = 16;
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .unwrap();

    for y in (0..height).rev() {
        for x in 0..width {
            let mut col = pool.install(|| {
                let mut local_col = Vec3::default();
                for _ in 0..samples_per_thread {
                    local_col += pool
                        .broadcast(|_ctx| {
                            let u = (x as f64 + random::<f64>()) / (width as f64);
                            let v = (y as f64 + random::<f64>()) / (height as f64);

                            let ray = camera.get_ray(u, v);
                            color(ray, &world, 0)
                        })
                        .par_iter()
                        .sum();
                }

                local_col
            });
            col /= actual_total_samples as f64;
            col = Vec3::from((f64::sqrt(col.x()), f64::sqrt(col.y()), f64::sqrt(col.z())));

            ppm.set_pixel(x, y, col * 255.99);
        }
    }

    let mut file = File::create("output/random_scene.ppm").expect("Could not create ppm file");
    ppm.write(&mut file);

    let end = SystemTime::now();
    let delta = Duration::new(end.duration_since(start).unwrap().as_secs(), 0);
    println!("Rendering took {}", humantime::format_duration(delta));
}

fn random_scene() -> HittableList {
    let mut list = HittableList::new();
    list.add(Box::new(Sphere::new(
        Vec3::from((0., -1000., 0.)),
        1000.,
        Arc::new(Lambertian::new((0.5, 0.5, 0.5).into())),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = random();
            let center = Vec3::from((
                a as f64 + 0.9 * random::<f64>(),
                0.2,
                b as f64 + 0.9 * random::<f64>(),
            ));
            if (center - Vec3::from((4., 0.2, 0.))).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    list.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Lambertian::new(Vec3::new(
                            random::<f64>() * random::<f64>(),
                            random::<f64>() * random::<f64>(),
                            random::<f64>() * random::<f64>(),
                        ))),
                    )))
                } else if choose_mat < 0.95 {
                    // metal
                    list.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Metal::new(
                            Vec3::new(
                                0.5 * (1. + random::<f64>()),
                                0.5 * (1. + random::<f64>()),
                                0.5 * (1. + random::<f64>()),
                            ),
                            0.5 * random::<f64>(),
                        )),
                    )));
                } else {
                    // glass
                    list.add(Box::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Dielectric::new(1.5)),
                    )));
                }
            }
        }
    }

    list.add(Box::new(Sphere::new(
        Vec3::new(0, 1, 0),
        1.,
        Arc::new(Dielectric::new(1.5)),
    )));
    list.add(Box::new(Sphere::new(
        Vec3::new(-4, 1, 0),
        1.,
        Arc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))),
    )));
    list.add(Box::new(Sphere::new(
        Vec3::new(4, 1, 0),
        1.,
        Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.)),
    )));

    list
}
