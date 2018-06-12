use vec3::{Ray, Vec3};
use std::f64::consts::PI;
pub struct Camera {
    pub origin: Vec3,
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lens_radius: f64,
}

impl Camera {
    // vfov is top to bottom in degrees
    pub fn new(look_from: Vec3, look_at: Vec3, v_up: Vec3, vfov: f64, aspect: f64, aperture: f64, focus_dist: f64) -> Camera {
        let lens_radius = aperture / 2.0;
        let theta = vfov * PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let origin = look_from;
        let w = (look_from - look_at).unit_vector();
        let u = v_up.cross(w).unit_vector();
        let v = w.cross(u);

        let lower_left_corner =
            origin -
            half_width * focus_dist * u -
            half_height * focus_dist * v -
            focus_dist * w;
        let horizontal = 2.0 * half_width * focus_dist * u;
        let vertical = 2.0 * half_height * focus_dist * v;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            lens_radius,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = u * rd.x() + v * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
        )
    }
}
