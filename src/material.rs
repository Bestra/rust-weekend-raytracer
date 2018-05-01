use vec3::{Ray, Vec3, reflect, dot};
use geo::HitRecord;

pub struct MaterialReflection {
    pub scattered: Ray,
    pub attenuation: Vec3,
}

pub trait Material {
    fn scatter(&self, r_in: Ray, rec: HitRecord) -> Option<MaterialReflection>;
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: Ray, rec: HitRecord) -> Option<MaterialReflection> {
        let target = rec.p + rec.normal+ Vec3::random_in_unit_sphere();
        let scattered = Ray::new(rec.p, target - rec.p);
        Some(MaterialReflection { scattered, attenuation: self.albedo })
    }
}

pub struct Metal {
    pub albedo: Vec3,
}

impl Material for Metal {
    fn scatter(&self, r_in: Ray, rec: HitRecord) -> Option<MaterialReflection> {
        let reflected = reflect(r_in.direction().unit_vector(), rec.normal);
        let scattered = Ray::new(rec.p, reflected);
        if dot(scattered.direction(), rec.normal) > 0.0 {
            Some(MaterialReflection { scattered: scattered.clone(), attenuation: self.albedo })
        } else {
            None
        }
    }
}
