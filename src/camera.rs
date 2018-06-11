use vec3::{Ray, Vec3, vec3};
use std::f64::consts::PI;
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(vfov: f64, aspect: f64) -> Camera {
        let theta = vfov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let lower_left_corner = vec3(-half_width, -half_height, -1.0);
        let horizontal = vec3(2.0 * half_width, 0.0, 0.0);
        let vertical = vec3(0.0, 2.0 * half_height, 0.0);
        let origin = vec3(0.0, 0.0, 0.0);

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
