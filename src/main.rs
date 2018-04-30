extern crate core;
extern crate png;
extern crate rand;
extern crate weekend_raytracer;

use png::HasParameters;
use std::fs::File;
use std::env;
use std::io::BufWriter;
use rand::distributions::{IndependentSample, Range};

use weekend_raytracer::{Ray, Vec3};
use weekend_raytracer::geo::{Hittable, HittableList, Sphere};

fn main() {
    let nx = 200;
    let ny = 100;
    let ns = 30;

    let mut img = vec![];

    let cam = Camera::new();
    let world = HittableList {
        list: vec![
            Box::new(Sphere {
                center: Vec3::new([0.0, 0.0, -1.0]),
                radius: 0.5,
            }),
            Box::new(Sphere {
                center: Vec3::new([0.0, -100.5, -1.0]),
                radius: 100.0,
            }),
        ],
    };

    let between = Range::new(0.0, 1.0);
    let mut rng = rand::thread_rng();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut total_color = Vec3::new([0.0, 0.0, 0.0]);

            for _s in 0..ns {
                let a = between.ind_sample(&mut rng);
                let b = between.ind_sample(&mut rng);

                let u = (i as f64 + a) / nx as f64;
                let v = (j as f64 + b)/ ny as f64;
                let r = cam.get_ray(u, v);
                total_color = total_color + color(&r, &world);
            }

            let col = total_color / ns as f64;

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

pub fn color<T: Hittable>(r: &Ray, world: &T) -> Vec3 {
    match world.hit(r, 0.0, core::f64::MAX) {
        Some(h) => {
            let target = h.p + h.normal + Vec3::random_in_unit_sphere();
            0.5 * color( &Ray::new( h.p, target - h.p), world)
        },

        None => {
            let unit_direction = r.direction().unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Vec3::new([1.0, 1.0, 1.0]) + t * Vec3::new([0.5, 0.7, 1.0])
        }
    }
}

pub struct Camera {
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    origin: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        let lower_left_corner = Vec3::new([-2.0, -1.0, -1.0]);
        let horizontal = Vec3::new([4.0, 0.0, 0.0]);
        let vertical = Vec3::new([0.0, 2.0, 0.0]);
        let origin = Vec3::new([0.0, 0.0, 0.0]);

        Camera {
            lower_left_corner,
            horizontal,
            vertical,
            origin,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
