use crate::hitable::HitRecord;
use crate::material::MaterialTypes::{D, L, M};
use crate::ray::Ray;
use crate::vec3::Vec3;

use rand::Rng;

pub enum MaterialTypes {
    D(Dielectric),
    L(Lambertian),
    M(Metal),
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)>;
}

impl Material for MaterialTypes {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        match rec.get_mat() {
            D(diel) => diel.scatter(r_in, rec),
            L(lamb) => lamb.scatter(r_in, rec),
            M(met) => met.scatter(r_in, rec),
        }
    }
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
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let target = rec.p + rec.normal + random_in_unit_sphere();

        let attenuation = self.albedo;
        let scattered = Ray::new(rec.p, target - rec.p);
        Some((attenuation, scattered))
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, f: f32) -> Metal {
        let fuzz = f32::min(f, 1.0);

        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = reflect(&Vec3::unit_vector(r_in.direction()), &rec.normal);
        let attenuation = self.albedo;
        let scattered = Ray::new(rec.p, reflected + self.fuzz * random_in_unit_sphere());

        match Vec3::dot(&scattered.direction(), &rec.normal) {
            float if float > 0.0 => Some((attenuation, scattered)),
            _ => None,
        }
    }
}

pub struct Dielectric {
    ref_idx: f32,
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Dielectric {
        Dielectric { ref_idx }
    }

    fn schlick(&self, cosine: f32) -> f32 {
        let mut r0 = (1.0 - self.ref_idx) / (1.0 + self.ref_idx);
        r0 = r0 * r0;

        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Vec3, Ray)> {
        let reflected = reflect(&r_in.direction(), &rec.normal);
        let attenuation = Vec3::new(1.0, 1.0, 1.0);

        let (ni_over_nt, outward_normal, cosine) = match Vec3::dot(&r_in.direction(), &rec.normal) {
            dot_product if dot_product > 0.0 => {
                let cosine = self.ref_idx * Vec3::dot(&r_in.direction(), &rec.normal)
                    / r_in.direction().length();
                (self.ref_idx, -rec.normal, cosine)
            }
            _ => {
                let cosine = -Vec3::dot(&r_in.direction(), &rec.normal) / r_in.direction().length();
                (1.0 / self.ref_idx, rec.normal, cosine)
            }
        };

        let (mut scattered, reflect_prob) =
            match refract(&r_in.direction(), &outward_normal, ni_over_nt) {
                Some(refracted) => {
                    let reflect_prop = self.schlick(cosine);
                    (Ray::new(rec.p, refracted), reflect_prop)
                }
                None => (Ray::new(rec.p, reflected), 1.0),
            };

        let mut rng = rand::thread_rng();

        if rng.gen::<f32>() < reflect_prob {
            scattered = Ray::new(rec.p, reflected);
        }

        Some((attenuation, scattered))
    }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0 * Vec3::dot(v, n) * *n
}

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f32) -> Option<Vec3> {
    let uv = Vec3::unit_vector(*v);
    let dt = Vec3::dot(&uv, n);

    match 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt) {
        discriminant if discriminant > 0.0 => {
            Some(ni_over_nt * (uv - *n * dt) - *n * discriminant.sqrt())
        }
        _ => None,
    }
}

fn random_in_unit_sphere() -> Vec3 {
    let mut rng = rand::thread_rng();

    loop {
        let p = 2.0 * Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>())
            - Vec3::new(1.0, 1.0, 1.0);

        if p.squared_length() >= 1.0 {
            return p;
        }
    }
}
