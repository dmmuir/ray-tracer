use rand::Rng;
use std::f32::consts::PI;

use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
    lens_radius: f32,
    w: Vec3,
    u: Vec3,
    v: Vec3,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        v_up: Vec3,
        v_fov: f32,
        aspect: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Camera {
        let lens_radius = aperture / 2.0;
        let theta = v_fov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;

        let origin = look_from;
        let w = Vec3::unit_vector(look_from - look_at);
        let u = Vec3::unit_vector(Vec3::cross(&v_up, &w));
        let v = Vec3::cross(&w, &u);

        let lower_left_corner = origin - half_width * u - half_height * v - focus_dist * w;
        let horizontal = 2.0 * half_width * focus_dist * u;
        let vertical = 2.0 * half_height * focus_dist * v;

        Camera {
            lower_left_corner,
            horizontal,
            vertical,
            origin,
            lens_radius,
            w,
            u,
            v,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}

fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();

    let mut p = Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), 0.0);

    while Vec3::dot(&p, &p) >= 1.0 {
        p = 2.0 * Vec3::new(rng.gen::<f32>(), rng.gen::<f32>(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
    }

    p
}
