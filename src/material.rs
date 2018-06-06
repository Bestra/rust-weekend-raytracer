use vec3::{Ray, Vec3, reflect, refract, dot};
use geo::HitRecord;

pub struct MaterialReflection {
    pub scattered: Ray,
    pub attenuation: Vec3,
    pub hit: bool
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
        let scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());
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
        let mut reflected = reflect(r_in.direction(), rec.normal);
        let attenuation = Vec3::new([1.0, 1.0, 0.0]);

        let (outward_normal, ni_over_nt) = if dot(r_in.direction(), rec.normal) > 0.0 {
            (-rec.normal, self.ref_idx)

        } else {
            (rec.normal, 1.0 / self.ref_idx)
        };

        match refract(r_in.direction(), outward_normal, ni_over_nt) {
            Some(refracted) => {
                Some(MaterialReflection { scattered: Ray::new(rec.p, refracted), attenuation: attenuation, hit: true })
            }
            None => {
                Some(MaterialReflection { scattered: Ray::new(rec.p, reflected), attenuation: attenuation, hit: false })
            }
        }
    }
}
