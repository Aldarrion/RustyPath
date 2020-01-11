use crate::vec3::Vec3;
use crate::ray::Ray;
use std::mem;

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
}
