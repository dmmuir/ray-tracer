use crate::hitable::{HitRecord, Hitable};
use crate::material::MaterialTypes;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: MaterialTypes,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: MaterialTypes) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = Vec3::dot(&ray.direction(), &ray.direction());
        let b = Vec3::dot(&oc, &ray.direction());
        let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let mut temp = (-b - (b * b - a * c).sqrt()) / a;
            if temp < t_max && temp > t_min {
                return Some(HitRecord::record_hit(temp, ray, self));
            }

            temp = (-b + (b * b * -a * c)) / a;

            if temp < t_max && temp > t_min {
                return Some(HitRecord::record_hit(temp, ray, self));
            }
        }

        None
    }
}
