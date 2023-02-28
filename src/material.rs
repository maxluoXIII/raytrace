use crate::hittable::HitRecord;
use crate::hittable::Sphere;
use crate::types::Ray;
use crate::types::Vec3;

pub trait Material {
    fn scatter(&self, r_in: &Ray, hit_rec: &HitRecord) -> Option<(Ray, Vec3)>;
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn from(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, hit_rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let target = hit_rec.p + hit_rec.normal + Sphere::random_in_unit_sphere();
        let scattered = Ray::from(hit_rec.p, target - hit_rec.p);
        let attenuation = self.albedo;

        Some((scattered, attenuation))
    }
}

pub struct Metal {
    albedo: Vec3,
}

impl Metal {
    pub fn from(albedo: Vec3) -> Metal {
        Metal { albedo }
    }
}

fn reflect(vec: &Vec3, normal: &Vec3) -> Vec3 {
    vec - 2.0 * Vec3::dot(vec, normal) * normal
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit_rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = reflect(&r_in.direction, &hit_rec.normal);
        if Vec3::dot(&reflected, &hit_rec.normal) > 0.0 {
            Some((Ray::from(hit_rec.p, reflected), self.albedo))
        } else {
            None
        }
    }
}
