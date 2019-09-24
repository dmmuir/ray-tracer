mod camera;
mod hitable;
mod hitable_list;
mod material;
mod ray;
mod sphere;
mod vec3;

use pbr::ProgressBar;
use rand::Rng;
use rayon::prelude::*;

use crate::camera::Camera;
use crate::hitable::Hitable;
use crate::hitable_list::HitableList;
use crate::material::{
    Dielectric, Lambertian, Material,
    MaterialTypes::{D, L, M},
    Metal,
};
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

fn main() {
    let nx = 1200;
    let ny = 600;
    let ns = 100;

    // Setup Progress Bar
    let mut pb = ProgressBar::new(ny as u64);
    pb.format("=>-");

    // Setup Camera
    let look_from = Vec3::new(14.0, 1.5, 4.0);
    let look_at = Vec3::new(0.0, -1.5, 4.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let cam = Camera::new(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        (nx / ny) as f32,
        aperture,
        dist_to_focus,
    );

    // Setup scene
    let world = random_scene();

    // Generate Image
    let mut image: Vec<String> = Vec::with_capacity(nx * ny + 1);
    image.push(format!("P3\n{} {}\n255\n", nx, ny));

    println!("rendering image...\n");
    for y in (0..ny).rev() {
        for x in 0..nx {
            let mut col: Vec3 = (0..ns)
                .into_par_iter()
                .map(|_s| {
                    let mut rng = rand::thread_rng();
                    let u: f32 = (x as f32 + rng.gen::<f32>()) / nx as f32;
                    let v: f32 = (y as f32 + rng.gen::<f32>()) / ny as f32;

                    let ray = cam.get_ray(u, v);

                    color(&ray, &world, 0)
                })
                .sum::<Vec3>();

            col /= ns as f32;
            col = Vec3::new(col[0].sqrt(), col[1].sqrt(), col[2].sqrt());
            let ir = (255.99 * col[0]) as i32;
            let ig = (255.99 * col[1]) as i32;
            let ib = (255.99 * col[2]) as i32;

            image.push(format!("{} {} {}\n", ir, ig, ib));
        }
        pb.inc();
    }

    pb.finish_print("render complete");

    // Save image
    println!("Writing file...");
    let filename = format!("./images/{}x{}x{}.ppm", nx, ny, ns);
    std::fs::write(filename, image.join("")).expect("Unable to write file");
}

fn color(ray: &Ray, world: &HitableList, depth: i32) -> Vec3 {
    let zero_vec3 = Vec3::new(0.0, 0.0, 0.0);

    match world.hit(ray, 0.001, std::f32::MAX) {
        Some(ref rec) if depth < 50 => match rec.get_mat().scatter(&ray, &rec) {
            Some((a, s)) => a * color(&s, world, depth + 1),
            None => zero_vec3,
        },
        Some(_) => zero_vec3,
        None => {
            let unit_direction = Vec3::unit_vector(ray.direction());
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}

fn random_scene() -> HitableList {
    let n = 500;

    let mut list: Vec<Sphere> = Vec::with_capacity(n + 1);

    list.push(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        L(Lambertian::new(Vec3::new(0.5, 0.5, 0.5))),
    ));

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f32>();
            let center = Vec3::new(
                a as f32 + 0.9 * rng.gen::<f32>(),
                0.2,
                b as f32 + 0.9 * rng.gen::<f32>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.5 {
                    list.push(Sphere::new(
                        center,
                        0.2,
                        L(Lambertian::new(Vec3::new(
                            rng.gen::<f32>() * rng.gen::<f32>(),
                            rng.gen::<f32>() * rng.gen::<f32>(),
                            rng.gen::<f32>() * rng.gen::<f32>(),
                        ))),
                    ));
                } else if choose_mat < 0.9 {
                    list.push(Sphere::new(
                        center,
                        0.2,
                        M(Metal::new(
                            Vec3::new(
                                0.5 * (1.0 + rng.gen::<f32>()),
                                0.5 * (1.0 + rng.gen::<f32>()),
                                0.5 * (1.0 + rng.gen::<f32>()),
                            ),
                            0.5 * rng.gen::<f32>(),
                        )),
                    ));
                } else {
                    list.push(Sphere::new(center, 0.2, D(Dielectric::new(1.5))));
                }
            }
        }
    }

    list.push(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        D(Dielectric::new(1.5)),
    ));
    list.push(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        L(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))),
    ));
    list.push(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        M(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
    ));

    HitableList::new(list)
}
