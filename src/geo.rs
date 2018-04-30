use vec3::{Vec3, Ray};

pub struct HitRecord {
    pub t: f64,
    pub p: Vec3,
    pub normal: Vec3,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}


pub struct Sphere {
    pub center: Vec3,
    pub radius: f64
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

                return Some(HitRecord {t, p, normal})
            }

            let temp_plus = (-b + (b * b - a * c).sqrt()) / a;
            if temp_plus < t_max && temp_plus > t_min {
                let t = temp_plus;
                let p = r.point_at_parameter(temp_plus);
                let normal = (p - self.center) / self.radius;

                return Some(HitRecord {t, p, normal})
            }
            None
        } else {
            None
        }
    }

    // fn test_hit(self, t: f64, r: Ray, t_min: f64, t_max: f64) -> Option<HitRecord>
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

                None => ()
            }
        }

        hit
    }
}
