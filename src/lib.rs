pub mod hittable;
pub mod material;
pub mod types;

use std::{
    f64::consts::PI,
    io::{BufWriter, Write},
};

use rand::{random, Rng};
use types::{Ray, Vec3};

const PPM_HEADER: &str = "P3\n";

pub struct Ppm {
    height: usize,
    width: usize,
    pixels: Vec<Vec<Vec3>>,
}

impl Ppm {
    pub fn new() -> Ppm {
        Ppm {
            height: 0,
            width: 0,
            pixels: Vec::new(),
        }
    }

    pub fn from(width: usize, height: usize) -> Ppm {
        let mut pixels = Vec::new();
        for y in 0..height {
            pixels.push(Vec::new());
            for _x in 0..width {
                pixels[y].push(Vec3::new());
            }
        }
        Ppm {
            height,
            width,
            pixels,
        }
    }

    pub fn set_height(&mut self, height: usize) {
        self.height = height;
        while self.pixels.len() < self.height {
            let mut new_row = Vec::new();
            for _ in 0..self.width {
                new_row.push(Vec3::new());
            }
            self.pixels.push(new_row);
        }
        self.pixels.truncate(self.height);
    }

    pub fn set_width(&mut self, width: usize) {
        self.width = width;
        for row in &mut self.pixels {
            if row.len() > self.width {
                row.truncate(self.width);
            } else {
                while row.len() < self.width {
                    row.push(Vec3::new());
                }
            }
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, pixel: Vec3) {
        self.pixels[self.height - 1 - y][x] = pixel;
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn write(&self, output: &mut dyn Write) {
        let mut writer = BufWriter::new(output);

        writer
            .write_all(PPM_HEADER.as_bytes())
            .expect("Could not write header");
        writer
            .write_all(format!("{} {}\n", self.width, self.height).as_bytes())
            .expect("Could not write dimensions");
        writer
            .write_all("255\n".as_bytes())
            .expect("Could not write max color");

        for (y, row) in self.pixels.iter().enumerate() {
            for (x, color) in row.iter().enumerate() {
                writer
                    .write_all(
                        format!(
                            "{} {} {}\n",
                            color.r() as u32,
                            color.g() as u32,
                            color.b() as u32
                        )
                        .as_bytes(),
                    )
                    .expect(&format!("Could not write pixel {x} {y}"));
            }
        }
    }
}

fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    loop {
        let p = 2. * Vec3::from((rng.gen(), rng.gen(), 0.)) - Vec3::from((1., 1., 0.));
        if p.squared_len() < 1.0 {
            return p;
        }
    }
}

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lens_radius: f64,
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Camera {
    // vert_fov is top to bottom in degrees
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        view_up: Vec3,
        vert_fov: f64,
        aspect: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = vert_fov * PI / 180.;
        let half_height = f64::tan(theta / 2.);
        let half_width = aspect * half_height;

        let w = Vec3::unit_vector(&(look_from - look_at));
        let u = Vec3::unit_vector(&view_up.cross(&w));
        let v = w.cross(&u);

        Camera {
            origin: look_from,
            lower_left_corner: look_from
                - half_width * focus_dist * u
                - half_height * focus_dist * v
                - focus_dist * w,
            horizontal: 2. * half_width * focus_dist * u,
            vertical: 2. * half_height * focus_dist * v,
            lens_radius: aperture / 2.,
            u,
            v,
            w,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let ray_dir = self.lens_radius * random_in_unit_disk();
        let offset = self.u * ray_dir.x() + self.v * ray_dir.y();
        Ray::from(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
