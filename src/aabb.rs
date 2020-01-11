use crate::vec3::Vec3;
use crate::ray::Ray;
use std::mem;
use std::f32;

pub struct AABB {
    min: Vec3,
    max: Vec3,
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> AABB {
        AABB {
            min,
            max
        }
    }

    pub fn new_empty() -> AABB {
        AABB {
            min: Vec3::new_fill(f32::MAX),
            max: Vec3::new_fill(f32::MIN)
        }
    }

    pub fn min(&self) -> &Vec3 {
        &self.min
    }

    pub fn max(&self) -> &Vec3 {
        &self.max
    }

    pub fn hit(&self, r: &Ray, tmin: f32, tmax: f32) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / r.direction().v()[a];
            let mut t0 = (self.min.v()[a] - r.origin().v()[a]) * inv_d;
            let mut t1 = (self.max.v()[a] - r.origin().v()[a]) * inv_d;
            if inv_d < 0.0 {
                mem::swap(&mut t0, &mut t1);
            }
            let tmin = if t0 > tmin { t0 } else { tmin };
            let tmax = if t1 < tmax { t1 } else { tmax };
            if tmax <= tmin {
                return false
            }
        }
        true
    }

    pub fn add(&mut self, other: &AABB) {
        for i in 0..3 {
            self.min.set(i, self.min.v()[i].min(other.min.v()[i]));
            self.max.set(i, self.max.v()[i].max(other.max.v()[i]));
        }
    }
}
