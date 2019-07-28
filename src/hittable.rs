use crate::vec3::{Vec3, sqr, random_in_unit_sphere};
use crate::ray::Ray;
use std::vec::Vec;
use std::boxed::Box;


pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)>;
}


pub struct Lambertian {
    albedo: Vec3
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let target = hit.p + hit.normal + random_in_unit_sphere();
        let scattered = Ray::new(hit.p, target - hit.p);
        let attenuation = self.albedo;
        Some((scattered, attenuation))
    }
}


struct Metal {
    albedo: Vec3
}

impl Material for Metal {
    
}


pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    material: Box<Material>
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

pub struct Sphere {
    center: Vec3,
    radius: f32
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere {
            center,
            radius
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin() - &self.center;
        let a = r.direction().dot(r.direction());
        let b = oc.dot(r.direction());
        let c = oc.dot(&oc) - sqr(self.radius);
        let discriminant = sqr(b) - a * c;
        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let point = r.point_at(temp);
                return Some(HitRecord {
                    t: temp,
                    p: point,
                    normal: (point - self.center) / self.radius
                });
            }

            let temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let point = r.point_at(temp);
                return Some(HitRecord {
                    t: temp,
                    p: point,
                    normal: (point - self.center) / self.radius
                });
            }
        }

        None
    }
}

pub struct HittableList {
    pub items: Vec<Box<Hittable>>
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut result = None;
        let mut closest = t_max;
        for item in &self.items {
            let current = item.hit(r, t_min, closest);
            match &current {
                Some(current_hit) => {
                    closest = current_hit.t;
                    result = current;
                },
                _ => ()
            }
        }

        result
    }
}

