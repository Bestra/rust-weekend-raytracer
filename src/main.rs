extern crate core;
extern crate png;
extern crate rand;
extern crate weekend_raytracer;
extern crate rayon;

use png::HasParameters;
use std::fs::File;
use std::env;
use std::io::BufWriter;
use rand::prelude::*;
use rayon::prelude::*;

use weekend_raytracer::vec3::{vec3, Ray, Vec3};
use weekend_raytracer::geo::{Hittable, random_scene, simple_spheres, sphere_tree};
use weekend_raytracer::camera::Camera;

fn main() {
    let mul = 4;
    let nx = mul * 200;
    let ny = mul * 100;
    let ns = 10;

    let mut img = vec![0u8; (nx * ny * 3) as usize];

    let look_from = vec3(13, 4, 5);
    let look_at = vec3(0, 0, 0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let cam = Camera::new(
        look_from,
        look_at,
        vec3(0, 1, 0),
        20.0,
        nx as f64 / ny as f64,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    // let world = random_scene();
    // let world = simple_spheres();
    let world = sphere_tree();
    println!("{:#?}", world);
    img.par_chunks_mut((nx * 3) as usize)
        .rev()
        .enumerate()
        .for_each(|(j, row)| {
            let mut rng = thread_rng();
            for (i, rgb) in row.chunks_mut(3usize).enumerate() {
                let mut total_color = vec3(0.0, 0.0, 0.0);

                for _ in 0..ns {
                    let a: f64 = rng.gen();
                    let b: f64 = rng.gen();

                    let u = (i as f64 + a) / nx as f64;
                    let v = (j as f64 + b) / ny as f64;
                    let r = cam.get_ray(u, v);
                    total_color = total_color + color(&r, &world, 0);
                }

                let col = total_color / ns as f64;

                let col = vec3(col.x().sqrt(), col.y().sqrt(), col.z().sqrt());

                let ir = 255.99 * col.x();
                let ig = 255.99 * col.y();
                let ib = 255.99 * col.z();
                let mut iter = rgb.iter_mut();
                *iter.next().unwrap() = ir as u8;
                *iter.next().unwrap() = ig as u8;
                *iter.next().unwrap() = ib as u8;
            }
        });

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

                    None => vec3(0.0, 0.0, 0.0),
                }
            } else {
                vec3(0.0, 0.0, 0.0)
            }
        }

        None => {
            let unit_direction = r.direction().unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * vec3(1.0, 1.0, 1.0) + t * vec3(0.5, 0.7, 1.0)
        }
    }
}
