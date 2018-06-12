extern crate core;
extern crate png;
extern crate rand;
extern crate weekend_raytracer;

use png::HasParameters;
use std::fs::File;
use std::rc::Rc;
use std::env;
use std::io::BufWriter;
use rand::prelude::*;

use weekend_raytracer::vec3::{vec3, Ray, Vec3};
use weekend_raytracer::geo::{Hittable, HittableList, Sphere};
use weekend_raytracer::material::{Dielectric, Lambertian, Metal};
use weekend_raytracer::camera::Camera;

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 40;

    let mut img = vec![];

    let look_from = vec3(3, 3, 2);
    let look_at = vec3(0, 0, -1);
    let dist_to_focus = (look_from - look_at).length();
    let aperture = 2.0;
    let cam = Camera::new(
        look_from,
        look_at,
        vec3(0, 1, 0),
        20.0,
        nx as f64 / ny as f64,
        aperture,
        dist_to_focus
    );
    let world = HittableList {
        list: vec![
            Box::new(Sphere {
                center: Vec3::new([0.0, 0.0, -1.0]),
                radius: 0.5,
                material: Rc::new(Lambertian {
                    albedo: Vec3::new([0.1, 0.2, 0.5]),
                }),
            }),
            Box::new(Sphere {
                center: Vec3::new([0.0, -100.5, -1.0]),
                radius: 100.0,
                material: Rc::new(Lambertian {
                    albedo: Vec3::new([0.8, 0.8, 0.0]),
                }),
            }),
            Box::new(Sphere {
                center: Vec3::new([1.0, 0.0, -1.0]),
                radius: 0.5,
                material: Rc::new(Metal {
                    fuzz: 0.0,
                    albedo: Vec3::new([0.8, 0.6, 0.2]),
                }),
            }),
            Box::new(Sphere {
                center: Vec3::new([-1.0, 0.0, -1.0]),
                radius: 0.5,
                material: Rc::new(Dielectric { ref_idx: 1.5 }),
            }),
            Box::new(Sphere {
                center: Vec3::new([-1.0, 0.0, -1.0]),
                radius: -0.45,
                material: Rc::new(Dielectric { ref_idx: 1.5 }),
            }),
        ],
    };

    let mut rng = thread_rng();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut total_color = Vec3::new([0.0, 0.0, 0.0]);

            for _s in 0..ns {
                let a: f64 = rng.gen();
                let b: f64 = rng.gen();

                let u = (i as f64 + a) / nx as f64;
                let v = (j as f64 + b) / ny as f64;
                let r = cam.get_ray(u, v);
                total_color = total_color + color(&r, &world, 0);
            }

            let col = total_color / ns as f64;

            let col = Vec3::new([col.x().sqrt(), col.y().sqrt(), col.z().sqrt()]);

            let ir = 255.99 * col.x();
            let ig = 255.99 * col.y();
            let ib = 255.99 * col.z();

            img.append(&mut vec![ir as u8, ig as u8, ib as u8])
        }
    }

    let mut path = env::current_dir().unwrap();
    path.push(format!("test{}.png", 1));
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, nx, ny);
    encoder.set(png::ColorType::RGB).set(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data(&img).unwrap();
}

pub fn color<T: Hittable>(r: &Ray, world: &T, depth: i8) -> Vec3 {
    match world.hit(r, 0.001, core::f64::MAX) {
        Some(rec) => {
            if depth < 50 {
                let r_clone = rec.clone();
                match rec.material.scatter(r.clone(), r_clone) {
                    Some(h) => h.attenuation * color(&h.scattered, world, depth + 1),

                    None => Vec3::new([0.0, 0.0, 0.0]),
                }
            } else {
                Vec3::new([0.0, 0.0, 0.0])
            }
        }

        None => {
            let unit_direction = r.direction().unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Vec3::new([1.0, 1.0, 1.0]) + t * Vec3::new([0.5, 0.7, 1.0])
        }
    }
}
