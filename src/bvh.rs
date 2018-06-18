use vec3::{Vec3, Ray, vec3};

pub fn ffmin(a: f64, b: f64) -> f64 {
    if a < b {
        a
    } else {
        b
    }
}

pub fn ffmax(a: f64, b: f64) -> f64 {
    if a > b {
        a
    } else {
        b
    }
}

pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
    let small = vec3(
        ffmin(box0.min().x(), box1.min().x()),
        ffmin(box0.min().y(), box1.min().y()),
        ffmin(box0.min().z(), box1.min().z())
    );
    let big = vec3(
        ffmax(box0.min().x(), box1.min().x()),
        ffmax(box0.min().y(), box1.min().y()),
        ffmax(box0.min().z(), box1.min().z())
    );

    AABB { min: small, max: big }
}

#[derive(Clone, Copy, Debug)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn min(&self) -> Vec3 {
        self.min
    }

    pub fn max(&self) -> Vec3 {
        self.max
    }

    pub fn hit(&self, r: &Ray, tmin: f64, tmax: f64) -> bool {
        for a in 0..3 {
            let t0 = ffmin(
                self.min[a] - r.origin()[a] / r.direction()[a],
                self.max[a] - r.origin()[a] / r.direction()[a]
            );
            let t1 = ffmax(
                self.min[a] - r.origin()[a] / r.direction()[a],
                self.max[a] - r.origin()[a] / r.direction()[a]
            );

            let tmin = ffmax(t0, tmin);
            let tmax = ffmin(t1, tmax);

            if tmax <= tmin {
                return false;
            }
        }

        true
    }
}
