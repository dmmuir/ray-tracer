use crate::hitable::{HitRecord, Hitable};
use crate::ray::Ray;
use crate::sphere::Sphere;

pub struct HitableList {
    list: Vec<Sphere>,
}

impl HitableList {
    pub fn new(list: Vec<Sphere>) -> HitableList {
        HitableList { list }
    }
}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        self.list
            .iter()
            .filter_map(|item| item.hit(ray, t_min, t_max))
            .map(|item| item)
            .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap())
    }
}
