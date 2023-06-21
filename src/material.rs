use rand::random;

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
    pub fn new(albedo: Vec3) -> Lambertian {
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
    fuzziness: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzziness: f64) -> Metal {
        let fuzziness = f64::min(fuzziness, 1.0);
        Metal { albedo, fuzziness }
    }
}

fn reflect(vec: &Vec3, normal: &Vec3) -> Vec3 {
    vec - 2.0 * Vec3::dot(vec, normal) * normal
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, hit_rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = reflect(&r_in.direction, &hit_rec.normal)
            + self.fuzziness * Sphere::random_in_unit_sphere();
        if Vec3::dot(&reflected, &hit_rec.normal) > 0.0 {
            Some((Ray::from(hit_rec.p, reflected), self.albedo))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(ref_idx: f64) -> Dielectric {
        Dielectric { ref_idx }
    }
}

fn refract(vec: &Vec3, normal: &Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let unit_vec = Vec3::unit_vector(vec);
    let dot = Vec3::dot(&unit_vec, normal);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dot * dot);
    if discriminant > 0.0 {
        let refracted = ni_over_nt * (unit_vec - normal * dot) - normal * f64::sqrt(discriminant);
        Some(refracted)
    } else {
        None
    }
}

fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * f64::powi(1.0 - cosine, 5)
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, hit_rec: &HitRecord) -> Option<(Ray, Vec3)> {
        let mut outward_normal = hit_rec.normal.clone();
        let reflected = reflect(&r_in.direction, &hit_rec.normal);
        let mut ni_over_nt = 1.0 / self.ref_idx;
        let attenuation = Vec3::from((1.0, 1.0, 1.0));
        let mut cosine = -Vec3::dot(&r_in.direction, &hit_rec.normal) / r_in.direction.length();

        // Handle total internal reflection
        if Vec3::dot(&r_in.direction, &hit_rec.normal) > 0.0 {
            outward_normal *= -1.0;
            ni_over_nt = self.ref_idx;
            cosine = self.ref_idx * Vec3::dot(&r_in.direction, &hit_rec.normal)
                / r_in.direction.length();
        }

        if let Some(refracted) = refract(&r_in.direction, &outward_normal, ni_over_nt) {
            let reflect_prob = schlick(cosine, self.ref_idx);

            if random::<f64>() > reflect_prob {
                return Some((Ray::from(hit_rec.p, refracted), attenuation));
            }
        }

        Some((Ray::from(hit_rec.p, reflected), attenuation))
    }
}
