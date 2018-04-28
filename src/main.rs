extern crate png;
extern crate weekend_raytracer;

use weekend_raytracer::{Vec3, Ray};

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

        let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
        let col = color(&r);

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

pub fn hit_sphere(center: Vec3, radius: f64, r: &Ray) -> Option<f64> {
    let oc = r.origin() - center;
    let a = r.direction().dot(r.direction());
    let b = 2.0 * oc.dot(r.direction());
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant > 0.0 {
        let closest_t = (-b - discriminant.sqrt()) / (2.0 * a);
        Some(closest_t)
    } else {
        None
    }
}

pub fn color(r: &Ray) -> Vec3 {
    match hit_sphere(Vec3::new([0.0, 0.0, -1.0]), 0.5, r) {
        Some(t) => {
            let n = (r.point_at_parameter(t) - Vec3::new([0.0, 0.0, -1.0])).unit_vector();
            0.5 * Vec3::new([n.x() + 1.0, n.y() + 1.0, n.z() + 1.0])
        }

        None => {
            let unit_direction = r.direction().unit_vector();
            let t = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - t) * Vec3::new([1.0, 1.0, 1.0]) + t * Vec3::new([0.5, 0.7, 1.0])
        }
    }
}






// fn print_png() {
//     let mut path = env::current_dir().unwrap();
//     path.push(format!("test{}.png", 1));
//     let file = File::create(path).unwrap();
//     let ref mut w = BufWriter::new(file);

//     let mut encoder = png::Encoder::new(w, 200, 100);
//     encoder.set(png::ColorType::RGB)
//     .set(png::BitDepth::Eight);

//     let mut writer = encoder.write_header().unwrap();
//     writer.write_image_data(&big_vec).unwrap();
// }
