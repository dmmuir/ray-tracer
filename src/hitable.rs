use crate::material::MaterialTypes;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

pub struct HitRecord<'a> {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    mat_type: &'a MaterialTypes,
}

impl<'a> HitRecord<'a> {
    pub fn record_hit(t: f32, ray: &Ray, sphere: &'a Sphere) -> HitRecord<'a> {
        let p = ray.point_at_parameter(t);
        let normal = (p - sphere.center) / sphere.radius;
        let mat_type = &sphere.material;

        HitRecord {
            t,
            p,
            normal,
            mat_type,
        }
    }

    pub fn get_mat(&'a self) -> &MaterialTypes {
        self.mat_type
    }
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}
