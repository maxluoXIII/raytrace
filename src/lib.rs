use std::io::{BufWriter, Write};

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
