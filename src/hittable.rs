use crate::types::{Ray, Vec3};

pub trait Hittable {
    fn hit(&self, ray: &Ray) -> Option<f64>;
}

pub struct Sphere {
    center: Vec3,
    radius: f64
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            center: Vec3::new(),
            radius: 1.0
        }
    }

    pub fn from(center: Vec3, radius: f64) -> Sphere {
        Sphere {
            center,
            radius
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray) -> Option<f64> {
        let oc = ray.origin - self.center;
        let a = Vec3::dot(&ray.direction, &ray.direction);
        let b = 2.0 * Vec3::dot(&oc, &ray.direction);
        let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b*b - 4.0 * a*c;

        if discriminant < 0.0 {
            None
        } else {
            Some((-b - f64::sqrt(discriminant)) / (2.0 * a))
        }

    }
}