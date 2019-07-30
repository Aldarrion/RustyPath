mod vec3;
mod ray;
mod hittable;
mod camera;

extern crate rand;

use rand::Rng;
use camera::Camera;
use hittable::{Sphere, Hittable, HittableList, Lambertian, Metal, Dielectric};
use vec3::{Vec3};
use ray::Ray;
use std::rc::Rc;

fn color(r: &Ray, world: &Hittable, depth: i32) -> Vec3 {
    // 0.001 to avoid self-intersections
    if let Some(result) = world.hit(r, 0.001, std::f32::MAX) {
        match &result.material.scatter(r, &result) {
            Some((scatter_result, attenuation)) if depth < 50 => {
                 attenuation * &color(&scatter_result, world, depth + 1)
            }
            _ => Vec3::zero()
        }
    } else {
        let unit_dir = r.direction().normalized();
        let t = 0.5 * (unit_dir.y() + 1.0);
        (1.0 - t) * Vec3::one() + t * Vec3::new(0.5, 0.7, 1.0).to_linear()
    }
}

fn main() {
    let nx = 800;
    let ny = 400;
    let ns = 1000;

    println!("P3\n{} {}\n255", nx, ny);

    let camera = Camera::new_default();
    let world = HittableList {items: vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Rc::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3).to_linear())))),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0).to_linear())))),
        Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2).to_linear(), 0.1)))),
        Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, Rc::new(Dielectric::new(1.5)))),
        //Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.45, Rc::new(Dielectric::new(1.5)))),
    ]};

    let mut rng = rand::thread_rng();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::zero();
            
            for _ in 0..ns {
                let u = (i as f32 + rng.gen::<f32>()) / nx as f32;
                let v = (j as f32 + rng.gen::<f32>()) / ny as f32;
                let r = camera.get_ray(u, v);
                col += &color(&r, &world, 0);
            }
            col /= ns as f32;
            col = col.to_srgb();

            let ir = (255.99 * col.r()) as i32;
            let ig = (255.99 * col.g()) as i32;
            let ib = (255.99 * col.b()) as i32;
            
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
