use crate::vec3::{Vec3, sqr, random_in_unit_sphere, schlick, clamp};
use crate::ray::Ray;
use crate::aabb::AABB;
use std::vec::Vec;
use rand::Rng;
use std::sync::Arc;
//use std::boxed::Box;
//use std::rc::Rc;
use std::cmp;


pub trait Material : Send + Sync {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)>;
}


pub struct Lambertian {
    albedo: Vec3
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian {
            albedo
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let target = hit.p + hit.normal + random_in_unit_sphere();
        let scattered = Ray::new(hit.p, target - hit.p, ray.time());
        let attenuation = self.albedo;
        Some((scattered, attenuation))
    }
}


pub struct Metal {
    albedo: Vec3,
    roughness: f32
}

impl Metal {
    pub fn new(albedo: Vec3, roughness: f32) -> Metal {
        Metal {
            albedo,
            roughness
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let scattered = Ray::new(hit.p, ray.direction().normalized().reflect(&hit.normal) + self.roughness * random_in_unit_sphere(), ray.time());
        if scattered.direction().dot(&hit.normal) > 0.0 {
            Some((
                scattered,
                self.albedo
            ))
        } else {
            None
        }
    }
}


pub struct Dielectric {
    ref_idx: f32
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Dielectric {
        Dielectric {
            ref_idx
        }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let (outward_normal, ni_over_nt, cosine) = if ray.direction().dot(&hit.normal) > 0.0 {
            (-hit.normal, 
            self.ref_idx,
            self.ref_idx * ray.direction().dot(&hit.normal) / ray.direction().length())
        } else {
            (hit.normal, 
            1.0 / self.ref_idx,
            -ray.direction().dot(&hit.normal) / ray.direction().length())
        };

        let attuneation = Vec3::new(1.0, 1.0, 1.0);
        if let Some(refracted) = ray.direction().refract(&outward_normal, ni_over_nt) {
            if rand::thread_rng().gen::<f32>() >= schlick(cosine, self.ref_idx) {
                return Some((Ray::new(hit.p, refracted, ray.time()), attuneation));
            }
        }

        Some((Ray::new(hit.p, ray.direction().reflect(&hit.normal), ray.time()), attuneation))
    }
}


pub struct HitRecord {
    pub material: Arc<dyn Material>,
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32
}

pub trait Hittable : Sync + Send {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn bouding_box(&self, t0: f32, t1: f32) -> Option<AABB>;
}

pub struct Sphere {
    center: Vec3,
    material: Arc<dyn Material>,
    radius: f32
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Arc<dyn Material>) -> Sphere {
        Sphere {
            center,
            material,
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
                    material: self.material.clone(),
                    p: point,
                    normal: (point - self.center) / self.radius,
                    t: temp
                });
            }

            let temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let point = r.point_at(temp);
                return Some(HitRecord {
                    material: self.material.clone(),
                    p: point,
                    normal: (point - self.center) / self.radius,
                    t: temp
                });
            }
        }

        None
    }

    fn bouding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(AABB::new(
            self.center - Vec3::new_fill(self.radius),
            self.center + Vec3::new_fill(self.radius),
        ))
    }
}

pub struct MovingSphere {
    position_start: Vec3,
    position_end: Vec3,
    time_start: f32,
    time_end: f32,
    radius: f32,
    material: Arc<dyn Material>,
}

impl MovingSphere {
    pub fn new(position_start: Vec3, position_end: Vec3, time_start: f32, time_end: f32, radius: f32, material: Arc<dyn Material>) -> MovingSphere {
        MovingSphere {
            position_start,
            position_end,
            time_start,
            time_end,
            radius,
            material
        }
    }

    fn center(&self, time: f32) -> Vec3 {
        let t = (time - self.time_start) / (self.time_end - self.time_start);
        let t = clamp(t, 0.0, 1.0);
        (1.0 - t) * self.position_start + t * self.position_end
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin() - &self.center(r.time());
        let a = r.direction().dot(r.direction());
        let b = oc.dot(r.direction());
        let c = oc.dot(&oc) - sqr(self.radius);
        let discriminant = sqr(b) - a * c;
        if discriminant > 0.0 {
            let temp = (-b - discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let point = r.point_at(temp);
                return Some(HitRecord {
                    material: self.material.clone(),
                    p: point,
                    normal: (point - self.center(r.time())) / self.radius,
                    t: temp
                });
            }

            let temp = (-b + discriminant.sqrt()) / a;
            if temp < t_max && temp > t_min {
                let point = r.point_at(temp);
                return Some(HitRecord {
                    material: self.material.clone(),
                    p: point,
                    normal: (point - self.center(r.time())) / self.radius,
                    t: temp
                });
            }
        }

        None
    }

    fn bouding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        let c0 = self.center(t0);
        let mut b0 = AABB::new(
            c0 - Vec3::new_fill(self.radius),
            c0 + Vec3::new_fill(self.radius)
        );
        let c1 = self.center(t1);
        let b1 = AABB::new(
            c1 - Vec3::new_fill(self.radius),
            c1 + Vec3::new_fill(self.radius)
        );

        b0.add(&b1);
        Some(b0)
    }
}

pub struct HittableList {
    pub items: Vec<Arc<dyn Hittable>>
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

    fn bouding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        let mut result = AABB::new_empty();

        for item in self.items.iter() {
            if let Some(aabb) = item.bouding_box(t0, t1) {
                result.add(&aabb);
            } else {
                return None;
            }
        }

        Some(result)
    }
}

pub struct BVHNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: AABB,
}

trait Axis {
    fn value() -> usize;
}
struct X {}
impl Axis for X {
    fn value() -> usize {0}
}
struct Y {}
impl Axis for Y {
    fn value() -> usize {1}
}
struct Z {}
impl Axis for Z {
    fn value() -> usize {2}
}

fn axis_sort<TAxis: Axis>(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> cmp::Ordering {
    if let (Some(abb), Some(bbb)) = (a.bouding_box(0.0, 0.0), b.bouding_box(0.0, 0.0)) {
        if abb.min().v()[TAxis::value()] - bbb.min().v()[TAxis::value()] < 0.0 {
            return cmp::Ordering::Less;
        } else {
            return cmp::Ordering::Greater;
        }
    }

    panic!("No bounding box int BVHNode constructor");
}

impl BVHNode {
    pub fn new (objects: &mut [Arc<dyn Hittable>], t0: f32, t1: f32) -> BVHNode {
        let axis = rand::thread_rng().gen_range(0, 3);
        
        /*eprintln!("++++++++++++");

        for o in objects.iter() {
            if let Some(bb) = o.bouding_box(0.0, 0.0) {
                eprintln!("{:?}", bb.min());
            }
        }*/

        // Sort by random axis to split in two halves
        match axis {
            0 => objects.sort_by(axis_sort::<X>),
            1 => objects.sort_by(axis_sort::<Y>),
            2 => objects.sort_by(axis_sort::<Z>),
            _ => panic!("Invalid axis number"),
        };

        /*eprintln!("------------");

        for o in objects.iter() {
            if let Some(bb) = o.bouding_box(0.0, 0.0) {
                eprintln!("{:?}", bb.min());
            }
        }

        eprintln!("____________");*/

        // Split in two halves
        let (left, right) = if objects.len() == 2 {
            (objects[0].clone(), objects[1].clone())
        } else {
            // TODO(aldarrion): Is all this necessary? Could we exploit the division maybe?
            let first_half = objects.len() / 2;
            let second_half = objects.len() - objects.len() / 2;
            let l = if first_half == 1 {
                objects[0].clone()
            } else {
                Arc::new(BVHNode::new(&mut objects[0..first_half], t0, t1))
            };
            let r = if second_half == 1 {
                objects[1].clone()
            } else {
                Arc::new(BVHNode::new(&mut objects[first_half..], t0, t1))
            };
            (l, r)
        };

        // Compute bbox for this node
        let bbox = if let (Some(lbb), Some(rbb)) = (left.bouding_box(t0, t1), right.bouding_box(t0, t1)) {
            AABB::new_surrounding(lbb, &rbb)
        } else {
            panic!("No bounding box in BVHNode constructor");
        };

        BVHNode{
            left,
            right,
            bbox
        }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        if self.bbox.hit(r, t_min, t_max) {
            let left_hit = self.left.hit(r, t_min, t_max);
            let right_hit = self.right.hit(r, t_min, t_max);
            
            match (left_hit, right_hit) {
                (Some(left_rec), Some(right_rec)) => {
                    if left_rec.t < right_rec.t {
                        return Some(left_rec);
                    } else {
                        return Some(right_rec);
                    }
                },
                (Some(left_rec), None) => Some(left_rec),
                (None, Some(right_rec)) => Some(right_rec),
                (None, None) => None,
            }
        } else {
            None
        }
    }

    fn bouding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        Some(self.bbox)
    }
}

