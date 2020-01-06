extern crate rand;

use rand::Rng;
use crate::vec3::{Vec3, random_in_unit_circle};
use crate::ray::Ray;

pub struct Camera {
    origin: Vec3,
    lower_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3, // right
    v: Vec3, // up
    //w: Vec3, // -forward
    lens_radius: f32,
    shutter_time: f32,
}

impl Camera {
    pub fn new(look_from: &Vec3, look_at: &Vec3, up: &Vec3, fov: f32, aspect: f32, aperture: f32, focus_dist: f32, shutter_time: f32) -> Camera {
        let theta = fov.to_radians();
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = (look_from - look_at).normalized();
        let u = up.cross(&w).normalized();
        let v = w.cross(&u);
        Camera {
            lower_left: look_from - &(half_width * focus_dist * u) - half_height * focus_dist * v - focus_dist * w,
            horizontal: 2.0 * half_width * focus_dist * u,
            vertical: 2.0 * half_height * focus_dist * v,
            origin: *look_from,
            u,
            v,
            //w,
            lens_radius: aperture / 2.0,
            shutter_time
        }
    }

    pub fn get_ray(&self, s: f32, t: f32, time_start: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_circle();
        let offset = self.u * rd.x() + self.v * rd.y();
        let mut rng = rand::thread_rng();
        let ray_time = time_start + rng.gen::<f32>() * self.shutter_time;
        Ray::new(self.origin + &offset, self.lower_left + s * self.horizontal + t * self.vertical - self.origin - offset, ray_time)
    }
}
