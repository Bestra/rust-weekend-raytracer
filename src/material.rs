use vec3::{Ray, Vec3, reflect, refract, dot};
use geo::HitRecord;
use rand::prelude::*;

pub fn schlick(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    let r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

pub struct MaterialReflection {
    pub scattered: Ray,
    pub attenuation: Vec3,
    pub hit: bool
}

pub trait Material: Sync + Send {
    fn scatter(&self, r_in: Ray, rec: HitRecord) -> Option<MaterialReflection>;
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, r_in: Ray, rec: HitRecord) -> Option<MaterialReflection> {
        let target = rec.p + rec.normal + Vec3::random_in_unit_sphere();
        let scattered = Ray::new(rec.p, target - rec.p, r_in.time());
        Some(MaterialReflection { scattered, attenuation: self.albedo, hit: true })
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: HitRecord) -> Option<MaterialReflection> {
        let reflected = reflect(r_in.direction().unit_vector(), rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere(), r_in.time());
        if dot(scattered.direction(), rec.normal) > 0.0 {
            Some(MaterialReflection { scattered: scattered.clone(), attenuation: self.albedo, hit: true })
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub ref_idx: f64,
}

impl Material for Dielectric {
    fn scatter(&self, r_in: Ray, rec: HitRecord) -> Option<MaterialReflection> {
        let reflected = reflect(r_in.direction(), rec.normal);
        let attenuation = Vec3::new([1.0, 1.0, 1.0]);
        let rdotn = dot(r_in.direction(), rec.normal);

        let (outward_normal, ni_over_nt, cosine) = if rdotn > 0.0 {
            let cos = rdotn / r_in.direction().length();
            let cos = (1.0 - self.ref_idx * self.ref_idx * (1.0 - cos - cos)).sqrt();
            (-rec.normal, self.ref_idx, cos)

        } else {
            let cos = -rdotn / r_in.direction().length();
            (rec.normal, 1.0 / self.ref_idx, cos)
        };

        let (reflect_prob, refracted) = match refract(r_in.direction(), outward_normal, ni_over_nt) {
            Some(refracted_val) => {
                (schlick(cosine, self.ref_idx), refracted_val)
            }
            None => {
                (1.0, Vec3::random_in_unit_sphere()) // the refracted value is meaningless here
            }
        };

        let mut rng = thread_rng();
        let x: f64 = rng.gen();

        let scattered = if x < reflect_prob {
            Ray::new(rec.p, reflected, r_in.time())
        } else {
            Ray::new(rec.p, refracted, r_in.time())
        };

        Some(MaterialReflection { scattered: scattered.clone(), attenuation: attenuation, hit: true })

    }
}
