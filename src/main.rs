extern crate png;

use png::HasParameters;
use std::fs::File;
use std::env;
use std::io::{BufReader, BufWriter};

fn main() {
  let nx = 200;
  let ny = 100;

  let mut img = vec![];

  for j in (0..ny).rev() {
    for i in 0..nx {
      let r = i as f64 / nx as f64;
      let g = j as f64 / ny as f64;
      let b = 0.2;

      let ir = 256.0 * r;
      let ig = 256.0 * g;
      let ib = 256.0 * b;
      img.append(&mut vec![ir as u8, ig as u8, ib as u8])
    }
  }

  let mut path = env::current_dir().unwrap();
  path.push(format!("test{}.png", 1));
  let file = File::create(path).unwrap();
  let ref mut w = BufWriter::new(file);

  let mut encoder = png::Encoder::new(w, 200, 100);
  encoder.set(png::ColorType::RGB)
  .set(png::BitDepth::Eight);

  let mut writer = encoder.write_header().unwrap();
  writer.write_image_data(&img).unwrap();
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
