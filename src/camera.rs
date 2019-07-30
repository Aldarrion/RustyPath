use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct Camera {
    origin: Vec3,
    lower_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    pub fn new(look_from: &Vec3, look_at: &Vec3, up: &Vec3, fov: f32, aspect: f32) -> Camera {
        let theta = fov.to_radians();
        let half_height = (theta / 2.0).atan();
        let half_width = aspect * half_height;
        let w = (look_from - look_at).normalized();
        let u = up.cross(&w);
        let v = w.cross(&u);
        Camera {
            lower_left: look_from - &(half_width * u) - half_height * v - w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
            origin: *look_from
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin, self.lower_left + u * self.horizontal + v * self.vertical - self.origin)
    }
}
