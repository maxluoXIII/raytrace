use std::rc::Rc;

use rand::random;

use crate::material::{Lambertian, Material};
use crate::types::{Ray, Vec3};

pub struct HitRecord {
    pub t: f64,
    pub p: Vec3, // hit point
    pub normal: Vec3,
    pub mat: Rc<dyn Material>,
}

pub trait Hittable {
    fn hit(&self, t_range: (f64, f64), ray: &Ray) -> Option<HitRecord>;
}

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Rc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        let mut sample = Vec3::from((random::<f64>(), random::<f64>(), random::<f64>()));
        while sample.length() > 1.0 {
            sample = Vec3::from((random::<f64>(), random::<f64>(), random::<f64>()));
        }

        sample
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self {
            center: Vec3::default(),
            radius: 1.0,
            material: Rc::new(Lambertian::new(Vec3::from((0.5, 0.5, 0.5)))),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, t_range: (f64, f64), ray: &Ray) -> Option<HitRecord> {
        // You can remove the 2s and 4s cuz they cancel out
        let oc = ray.origin - self.center;
        let a = Vec3::dot(&ray.direction, &ray.direction);
        let b = 2.0 * Vec3::dot(&oc, &ray.direction);
        let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant >= 0.0 {
            let mut temp = (-b - f64::sqrt(b * b - 4.0 * a * c)) / (2.0 * a);
            if temp >= t_range.0 && temp < t_range.1 {
                return Some(HitRecord {
                    t: temp,
                    p: ray.pos(temp),
                    normal: (ray.pos(temp) - self.center) / self.radius,
                    mat: self.material.clone(),
                });
            }

            temp = (-b + f64::sqrt(b * b - 4.0 * a * c)) / (2.0 * a);
            if temp >= t_range.0 && temp < t_range.1 {
                return Some(HitRecord {
                    t: temp,
                    p: ray.pos(temp),
                    normal: ((ray.pos(temp) - self.center) / self.radius),
                    mat: self.material.clone(),
                });
            }
        }

        return None;
    }
}

pub struct HittableList {
    list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { list: Vec::new() }
    }

    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.list.push(obj);
    }
}

impl Hittable for HittableList {
    fn hit(&self, t_range: (f64, f64), ray: &Ray) -> Option<HitRecord> {
        let mut ret = None;

        let mut closest_hit = f64::MAX;
        for obj in &self.list {
            if let Some(hit_rec) = obj.hit((t_range.0, closest_hit), ray) {
                closest_hit = hit_rec.t;
                ret = Some(hit_rec);
            }
        }

        ret
    }
}
