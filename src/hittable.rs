use crate::types::{Ray, Vec3};

pub struct HitRecord {
    pub t: f64,
    pub p: Vec3, // hit point
    pub normal: Vec3,
}

pub trait Hittable {
    fn hit(&self, t_range: (f64, f64), ray: &Ray) -> Option<HitRecord>;
}

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            center: Vec3::new(),
            radius: 1.0,
        }
    }

    pub fn from(center: Vec3, radius: f64) -> Sphere {
        Sphere { center, radius }
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
                });
            }

            temp = (-b + f64::sqrt(b * b - 4.0 * a * c)) / (2.0 * a);
            if temp >= t_range.0 && temp < t_range.1 {
                return Some(HitRecord {
                    t: temp,
                    p: ray.pos(temp),
                    normal: ((ray.pos(temp) - self.center) / self.radius),
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
        HittableList {
            list: Vec::new(),
        }
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
