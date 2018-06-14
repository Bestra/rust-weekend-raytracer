use vec3::{Ray, Vec3, vec3};
use material::{Dielectric, Lambertian, Material, Metal};
use std::sync::Arc;
use rand::prelude::*;

#[derive(Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Arc<Material>,
}

pub trait Hittable: Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Arc<Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Arc<Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().dot(r.direction());
        let b = oc.dot(r.direction());
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let temp_minus = (-b - (b * b - a * c).sqrt()) / a;
            if temp_minus < t_max && temp_minus > t_min {
                let t = temp_minus;
                let p = r.point_at_parameter(t);
                let normal = (p - self.center) / self.radius;

                return Some(HitRecord {
                    t,
                    p,
                    normal,
                    material: Arc::clone(&self.material),
                });
            }

            let temp_plus = (-b + (b * b - a * c).sqrt()) / a;
            if temp_plus < t_max && temp_plus > t_min {
                let t = temp_plus;
                let p = r.point_at_parameter(temp_plus);
                let normal = (p - self.center) / self.radius;

                return Some(HitRecord {
                    t,
                    p,
                    normal,
                    material: Arc::clone(&self.material),
                });
            }
            None
        } else {
            None
        }
    }

}

pub struct MovingSphere {
    pub center0: Vec3,
    pub center1: Vec3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub material: Arc<Material>,
}

impl MovingSphere {
    pub fn new(center0: Vec3, center1: Vec3, time0: f64, time1: f64, radius: f64, material: Arc<Material>) -> MovingSphere {
        MovingSphere {
            center0,
            center1,
            time0,
            time1,
            radius,
            material,
        }
    }

    pub fn center(&self, time: f64) -> Vec3 {
        self.center0 + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center(r.time());
        let a = r.direction().dot(r.direction());
        let b = oc.dot(r.direction());
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;
        if discriminant > 0.0 {
            let temp_minus = (-b - (b * b - a * c).sqrt()) / a;
            if temp_minus < t_max && temp_minus > t_min {
                let t = temp_minus;
                let p = r.point_at_parameter(t);
                let normal = (p - self.center(r.time())) / self.radius;

                return Some(HitRecord {
                    t,
                    p,
                    normal,
                    material: Arc::clone(&self.material),
                });
            }

            let temp_plus = (-b + (b * b - a * c).sqrt()) / a;
            if temp_plus < t_max && temp_plus > t_min {
                let t = temp_plus;
                let p = r.point_at_parameter(temp_plus);
                let normal = (p - self.center(r.time())) / self.radius;

                return Some(HitRecord {
                    t,
                    p,
                    normal,
                    material: Arc::clone(&self.material),
                });
            }
            None
        } else {
            None
        }
    }

}

pub struct HittableList {
    pub list: Vec<Box<Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit = None;
        let mut closest_so_far = t_max;

        for item in &self.list {
            match item.hit(&r.clone(), t_min, closest_so_far) {
                Some(temp_rec) => {
                    closest_so_far = temp_rec.t;
                    hit = Some(temp_rec)
                }

                None => (),
            }
        }

        hit
    }
}


pub fn random_scene() -> HittableList {
    let mut list: Vec<Box<Hittable>> = vec![
        Box::new(Sphere {
            center: vec3(0, -1000, 0),
            radius: 1000.0,
            material: Arc::new(Lambertian {
                albedo: vec3(0.5, 0.5, 0.5),
            }),
        }),
    ];

    let mut rng = thread_rng();
    for a in -10..10 {
        for b in -10..10 {
            let choose_mat: f64 = rng.gen();
            let v1: f64 = rng.gen();
            let v2: f64 = rng.gen();
            let center = vec3(a as f64 + 0.9 * v1, 0.2, b as f64 + 0.9 * v2);

            if (center - vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                match choose_mat {
                    x  if x < 0.8 => { //diffuse
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
                            })))
                        );
                    }

                    x if x < 0.95 => { // metal
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
                            })))
                        );
                    }

                    _ => { // glass
                        list.push(Box::new(Sphere::new(
                            center,
                            0.2,
                            Arc::new(Dielectric {
                                ref_idx: 1.5,
                            })))
                        );
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

    HittableList {
        list
    }
}
