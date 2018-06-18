use bvh::{surrounding_box, AABB};
use material::{Material};
use rand::prelude::*;
use std::cmp::Ordering;
use std::sync::Arc;
use std::fmt::Debug;
use vec3::{vec3, Ray, Vec3};

#[derive(Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Arc<Material>,
}

pub trait Hittable: Sync + Debug {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB>;
    fn box_clone(&self) -> Box<Hittable>;
}

impl Clone for Box<Hittable> {
    fn clone(&self) -> Box<Hittable> {
        self.box_clone()
    }
}

#[derive(Clone, Debug)]
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

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(AABB {
            min: self.center - vec3(self.radius, self.radius, self.radius),
            max: self.center + vec3(self.radius, self.radius, self.radius),
        })
    }

    fn box_clone(&self) -> Box<Hittable> {
        Box::new((*self).clone())
    }
}

#[derive(Clone, Debug)]
pub struct MovingSphere {
    pub center0: Vec3,
    pub center1: Vec3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub material: Arc<Material>,
}

impl MovingSphere {
    pub fn new(
        center0: Vec3,
        center1: Vec3,
        time0: f64,
        time1: f64,
        radius: f64,
        material: Arc<Material>,
    ) -> MovingSphere {
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
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
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

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        let b0 = AABB {
            min: self.center0 - vec3(self.radius, self.radius, self.radius),
            max: self.center0 + vec3(self.radius, self.radius, self.radius),
        };

        let b1 = AABB {
            min: self.center1 - vec3(self.radius, self.radius, self.radius),
            max: self.center1 + vec3(self.radius, self.radius, self.radius),
        };

        Some(surrounding_box(&b0, &b1))
    }

    fn box_clone(&self) -> Box<Hittable> {
        Box::new((*self).clone())
    }
}

#[derive(Clone, Debug)]
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

    fn bounding_box(&self, t0: f64, t1: f64) -> Option<AABB> {
        if self.list.len() < 1 {
            return None;
        }

        let mut tmp_box;

        tmp_box = self.list[0].bounding_box(t0, t1)?;

        for i in 1..self.list.len() {
            match self.list[i].bounding_box(t0, t1) {
                Some(abox) => {
                    tmp_box = surrounding_box(&abox, &tmp_box);
                }
                None => return None,
            }
        }

        Some(tmp_box)
    }

    fn box_clone(&self) -> Box<Hittable> {
        Box::new((*self).clone())
    }
}


pub fn box_x_compare(a: &Box<Hittable>, b: &Box<Hittable>) -> Ordering {
    match (a.bounding_box(0.0, 0.0), b.bounding_box(0.0, 0.0)) {
        (Some(l), Some(r)) => {
            if l.min().x() - r.min().x() < 0.0 {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
        _ => panic!("no bounding box found for either l or r"),
    }
}

pub enum Axis {
    X,
    Y,
    Z
}

pub fn box_y_compare(a: &Box<Hittable>, b: &Box<Hittable>) -> Ordering {
    match (a.bounding_box(0.0, 0.0), b.bounding_box(0.0, 0.0)) {
        (Some(l), Some(r)) => {
            if l.min().y() - r.min().y() < 0.0 {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
        _ => panic!("no bounding box found for either l or r"),
    }
}

pub fn box_z_compare(a: &Box<Hittable>, b: &Box<Hittable>) -> Ordering {
    match (a.bounding_box(0.0, 0.0), b.bounding_box(0.0, 0.0)) {
        (Some(l), Some(r)) => {
            if l.min().z() - r.min().z() < 0.0 {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
        _ => panic!("no bounding box found for either l or r"),
    }
}

pub fn box_compare(a: &Box<Hittable>, b: &Box<Hittable>, axis: Axis) -> Ordering {
    match (a.bounding_box(0.0, 0.0), b.bounding_box(0.0, 0.0)) {
        (Some(l), Some(r)) => {
            let result = match axis {
                Axis::X => l.min().x() - r.min().x(),
                Axis::Y => l.min().y() - r.min().y(),
                Axis::Z => l.min().z() - r.min().z()
            };
            if result < 0.0 {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        }
        _ => panic!("no bounding box found for either l or r"),
    }
}

#[derive(Clone, Debug)]
pub struct BVHNode {
    left: Box<Hittable>,
    right: Box<Hittable>,
    bounding_box: AABB,
}

impl BVHNode {
    pub fn new(mut hitable: Vec<Box<Hittable>>, time0: f64, time1: f64) -> BVHNode {
        let mut rng = thread_rng();
        let axis = rng.gen_range(0, 2);

        match axis {
            0 => {
                hitable.sort_by(box_x_compare);
            }
            1 => {
                hitable.sort_by(box_y_compare);
            }
            2 => {
                hitable.sort_by(box_z_compare);
            }
            _ => panic!("this should never happen"),
        }

        let left: Box<Hittable>;
        let right: Box<Hittable>;
        let len = hitable.len();
        match len {
            0 => panic!("empty hittable list"),
            1 => {
                left = hitable[0].clone();
                right = hitable[0].clone();
            }
            2 => {
                left = hitable[0].clone();
                right = hitable[1].clone();
            }
            _ => {
                let r = hitable.split_off(len / 2);
                left = Box::new(BVHNode::new(hitable, time0, time1));
                right = Box::new(BVHNode::new(r, time0, time1));
            }
        }

        let bounding_box = match (
            left.bounding_box(time0, time1),
            right.bounding_box(time0, time1),
        ) {
            (Some(l_box), Some(r_box)) => surrounding_box(&l_box, &r_box),
            _ => panic!("no bounding box found for either l or r"),
        };

        BVHNode {
            left: left,
            right: right,
            bounding_box,
        }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if self.bounding_box.hit(r, t_min, t_max) {
            let hit_left = self.left.hit(r, t_min, t_max);
            let hit_right = self.right.hit(r, t_min, t_max);
            match (hit_left, hit_right) {
                (Some(left_rec), Some(right_rec)) => {
                    if left_rec.t < right_rec.t {
                        Some(left_rec)
                    } else {
                        Some(right_rec)
                    }
                }
                (Some(left_rec), None) => Some(left_rec),
                (None, Some(right_rec)) => Some(right_rec),
                (None, None) => None,
            }
        } else {
            return None;
        }
    }

    fn bounding_box(&self, _t0: f64, _t1: f64) -> Option<AABB> {
        Some(self.bounding_box)
    }

    fn box_clone(&self) -> Box<Hittable> {
        Box::new((*self).clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use material::Lambertian;
    #[test]
    fn bvh_node_with_one_item() {
        let v: Vec<Box<Hittable>> = vec![
            Box::new(Sphere {
                center: vec3(0, 0, 0),
                radius: 1.0,
                material: Arc::new(Lambertian {
                    albedo: vec3(0.8, 0.8, 0.0),
                }),
            }),
        ];

        let n = BVHNode::new(v, 0.0, 1.0);
        let b = n.bounding_box(0.0, 1.0).unwrap();
        println!("{:?}", b);

        assert_eq!(b.max(), Vec3::new([1.0, 1.0, 1.0]));
    }
}