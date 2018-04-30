extern crate png;
extern crate weekend_raytracer;
extern crate core;

use weekend_raytracer::{Vec3, Ray};
use weekend_raytracer::geo::{Sphere, Hittable, HittableList};

use png::HasParameters;
use std::fs::File;
use std::env;
use std::io::{BufWriter};

fn main() {
    let nx = 200;
    let ny = 100;

    let mut img = vec![];

    let lower_left_corner = Vec3::new([-2.0, -1.0, -1.0]);
    let horizontal = Vec3::new([4.0, 0.0, 0.0]);
    let vertical = Vec3::new([0.0, 2.0, 0.0]);
    let origin = Vec3::new([0.0, 0.0, 0.0]);

  for j in (0..ny).rev() {
    for i in 0..nx {

        let u = i as f64 / nx as f64;
        let v = j as f64 / ny as f64;

        let world = HittableList {
            list: vec![
                Box::new(Sphere {
                    center: Vec3::new([0.0, 0.0, -1.0]),
                    radius: 0.5
                }),
                Box::new(Sphere {
                    center: Vec3::new([0.0, -100.5, -1.0]),
                    radius: 100.0
                }),
            ]
        };

        let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
        let col = color(&r, world);

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
  encoder.set(png::ColorType::RGB)
  .set(png::BitDepth::Eight);

  let mut writer = encoder.write_header().unwrap();
  writer.write_image_data(&img).unwrap();
}

pub fn color<T: Hittable>(r: &Ray, world: T) -> Vec3 {
    match world.hit(r, 0.0, core::f64::MAX) {
        Some(h) => {
            0.5 * Vec3::new([h.normal.x() + 1.0, h.normal.y() + 1.0, h.normal.z() + 1.0])
        }

        None => {
            let unit_direction = r.direction().unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Vec3::new([1.0, 1.0, 1.0]) + t * Vec3::new([0.5, 0.7, 1.0])
        }
    }
}
