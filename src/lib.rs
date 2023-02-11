use std::{
    io::{BufWriter, Write},
    ops,
};

const PPM_HEADER: &str = "P3\n";

pub struct Ppm {
    height: usize,
    width: usize,
    pixels: Vec<Vec<(u32, u32, u32)>>,
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
                pixels[y].push((0, 0, 0));
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
                new_row.push((0, 0, 0));
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
                    row.push((0, 0, 0));
                }
            }
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, pixel: (u32, u32, u32)) {
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
            for (x, (r, g, b)) in row.iter().enumerate() {
                writer
                    .write_all(format!("{} {} {}\n", r, g, b).as_bytes())
                    .expect(&format!("Could not write pixel {x} {y}"));
            }
        }
    }
}

pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new() -> Vec3 {
        Vec3 { e: [0.0, 0.0, 0.0] }
    }

    pub fn from(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn r(&self) -> f64 {
        self.e[0]
    }

    pub fn g(&self) -> f64 {
        self.e[1]
    }

    pub fn b(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        (self.e[0].powi(2) + self.e[1].powi(2) + self.e[2].powi(2)).sqrt()
    }

    pub fn squared_len(&self) -> f64 {
        self.e[0].powi(2) + self.e[1].powi(2) + self.e[2].powi(2)
    }

    pub fn make_unit_vector(&mut self) {
        let len = self.length();
        self.e[0] /= len;
        self.e[1] /= len;
        self.e[2] /= len;
    }

    pub fn dot(&self, v2: &Vec3) -> f64 {
        self.e[0] * v2.e[0] + self.e[1] * v2.e[1] + self.e[2] * v2.e[2]
    }

    pub fn cross(&self, v2: &Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[1] * v2.e[2] - self.e[2] * v2.e[1],
                -(self.e[0] * v2.e[2] - self.e[2] * v2.e[0]),
                self.e[0] * v2.e[1] - self.e[1] * v2.e[0],
            ],
        }
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self {
            e: [
                self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2],
            ],
        }
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Self {
            e: [
                self.e[0] - rhs.e[0],
                self.e[1] - rhs.e[1],
                self.e[2] - rhs.e[2],
            ],
        }
    }
}

impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.e[0] -= rhs.e[0];
        self.e[1] -= rhs.e[1];
        self.e[2] -= rhs.e[2];
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Self {
            e: [
                self.e[0] * rhs.e[0],
                self.e[1] * rhs.e[1],
                self.e[2] * rhs.e[2],
            ],
        }
    }
}

impl ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.e[0] *= rhs.e[0];
        self.e[1] *= rhs.e[1];
        self.e[2] *= rhs.e[2];
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs],
        }
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl ops::Div<Vec3> for Vec3 {
    type Output = Self;

    fn div(self, rhs: Vec3) -> Self::Output {
        Self {
            e: [
                self.e[0] / rhs.e[0],
                self.e[1] / rhs.e[1],
                self.e[2] / rhs.e[2],
            ],
        }
    }
}

impl ops::DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, rhs: Vec3) {
        self.e[0] /= rhs.e[0];
        self.e[1] /= rhs.e[1];
        self.e[2] /= rhs.e[2];
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        &self / rhs
    }
}

impl ops::Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            e: [self.e[0] / rhs, self.e[1] / rhs, self.e[2] / rhs],
        }
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.e[0] /= rhs;
        self.e[1] /= rhs;
        self.e[2] /= rhs;
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            e: [self.e[0] * -1.0, self.e[1] * -1.0, self.e[2] * -1.0],
        }
    }
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    v / v.length()
}
