//!
//! Contains helpers for constructing scenes out of geometry primitives
use geo::{BVHNode, Hittable, HittableList, MovingSphere, Sphere};
use material::{Dielectric, Lambertian, Metal};
use rand::prelude::*;
use std::sync::Arc;
use vec3::vec3;

pub fn random_scene() -> HittableList {
    let mut list: Vec<Box<Hittable>> = vec![Box::new(Sphere {
        center: vec3(0, -1000, 0),
        radius: 1000.0,
        material: Arc::new(Lambertian {
            albedo: vec3(0.5, 0.5, 0.5),
        }),
    })];

    let mut rng = thread_rng();
    for a in -10..10 {
        for b in -10..10 {
            let choose_mat: f64 = rng.gen();
            let v1: f64 = rng.gen();
            let v2: f64 = rng.gen();
            let center = vec3(a as f64 + 0.9 * v1, 0.2, b as f64 + 0.9 * v2);

            if (center - vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                match choose_mat {
                    x if x < 0.8 => {
                        //diffuse
                        let v3: f64 = rng.gen();
                        let a1: f64 = rng.gen();
                        let a2: f64 = rng.gen();
                        let a3: f64 = rng.gen();
                        list.push(Box::new(MovingSphere::new(
                            center,
                            center + vec3(0.0, 0.5 * v3, 0.0),
                            0.0,
                            1.0,
                            0.2,
                            Arc::new(Lambertian {
                                albedo: vec3(a1, a2, a3),
                            }),
                        )));
                    }

                    x if x < 0.95 => {
                        // metal
                        let a1: f64 = rng.gen();
                        let a2: f64 = rng.gen();
                        let a3: f64 = rng.gen();
                        let a4: f64 = rng.gen();
                        list.push(Box::new(Sphere::new(
                            center,
                            0.2,
                            Arc::new(Metal {
                                albedo: vec3(0.5 * 1.0 + a1, 0.5 * 1.0 + a2, 0.5 * 1.0 + a3),
                                fuzz: 0.5 * 1.0 + a4,
                            }),
                        )));
                    }

                    _ => {
                        // glass
                        list.push(Box::new(Sphere::new(
                            center,
                            0.2,
                            Arc::new(Dielectric { ref_idx: 1.5 }),
                        )));
                    }
                }
            }
        }
    }

    list.append(&mut vec![
        Box::new(Sphere {
            center: vec3(0, 1, 0),
            radius: 1.0,
            material: Arc::new(Dielectric { ref_idx: 1.5 }),
        }),
        Box::new(Sphere {
            center: vec3(-4, 1, 0),
            radius: 1.0,
            material: Arc::new(Lambertian {
                albedo: vec3(0.4, 0.2, 0.1),
            }),
        }),
        Box::new(Sphere {
            center: vec3(4, 1, 0),
            radius: 1.0,
            material: Arc::new(Metal {
                fuzz: 0.0,
                albedo: vec3(0.8, 0.6, 0.2),
            }),
        }),
    ]);

    HittableList { list }
}

pub fn simple_spheres() -> HittableList {
    HittableList {
        list: vec![
            Box::new(Sphere {
                center: vec3(0, -1000, 0),
                radius: 1000.0,
                material: Arc::new(Lambertian {
                    albedo: vec3(0.8, 0.8, 0.0),
                }),
            }),
            Box::new(Sphere {
                center: vec3(4, 1, 0),
                radius: 1.0,
                material: Arc::new(Lambertian {
                    albedo: vec3(0.1, 0.2, 0.5),
                }),
            }),
            Box::new(Sphere {
                center: vec3(-4, 1, 0),
                radius: 1.0,
                material: Arc::new(Metal {
                    fuzz: 0.0,
                    albedo: vec3(0.8, 0.6, 0.2),
                }),
            }),
            Box::new(Sphere {
                center: vec3(0, 1, 0),
                radius: 1.0,
                material: Arc::new(Dielectric { ref_idx: 1.5 }),
            }),
            Box::new(Sphere {
                center: vec3(0, 1, 0),
                radius: -0.95,
                material: Arc::new(Dielectric { ref_idx: 1.5 }),
            }),
            // Box::new(Sphere {
            //     center: vec3(0, 1, 0),
            //     radius: 1.0,
            //     material: Arc::new(Dielectric { ref_idx: 1.5 }),
            // }),
            // Box::new(Sphere {
            //     center: vec3(-4, 1, 0),
            //     radius: 1.0,
            //     material: Arc::new(Lambertian {
            //         albedo: vec3(0.4, 0.2, 0.1),
            //     }),
            // }),
            // Box::new(Sphere {
            //     center: vec3(4, 1, 0),
            //     radius: 1.0,
            //     material: Arc::new(Metal {
            //         fuzz: 0.0,
            //         albedo: vec3(0.8, 0.6, 0.2),
            //     }),
            // }),
        ],
    }
}

pub fn sphere_tree() -> BVHNode {
    let v: Vec<Box<Hittable>> = vec![
        Box::new(Sphere {
            center: vec3(0, -1000, 0),
            radius: 1000.0,
            material: Arc::new(Lambertian {
                albedo: vec3(0.8, 0.8, 0.0),
            }),
        }),
        Box::new(Sphere {
            center: vec3(4, 1, 0),
            radius: 1.0,
            material: Arc::new(Lambertian {
                albedo: vec3(0.1, 0.2, 0.5),
            }),
        }),
        Box::new(Sphere {
            center: vec3(-4, 1, 0),
            radius: 1.0,
            material: Arc::new(Metal {
                fuzz: 0.0,
                albedo: vec3(0.8, 0.6, 0.2),
            }),
        }),
        Box::new(Sphere {
            center: vec3(0, 1, 0),
            radius: 1.0,
            material: Arc::new(Dielectric { ref_idx: 1.5 }),
        }),
        Box::new(Sphere {
            center: vec3(0, 1, 0),
            radius: -0.95,
            material: Arc::new(Dielectric { ref_idx: 1.5 }),
        }),
    ];

    BVHNode::new(v, 0.0, 1.0, &None)
}
