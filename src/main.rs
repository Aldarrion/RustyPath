mod vec3;
mod ray;
mod hittable;
mod camera;

extern crate rand;

use rand::Rng;
use camera::Camera;
use hittable::{Sphere, MovingSphere, Hittable, HittableList, Lambertian, Metal, Dielectric};
use vec3::{Vec3};
use ray::Ray;
use std::rc::Rc;

fn color(r: &Ray, world: &dyn Hittable, depth: i32) -> Vec3 {
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

fn random_scene() -> Box<dyn Hittable> {
    let mut scene = HittableList { items: vec![] };
    scene.items.push(Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Rc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5).to_linear())))));

    let mut rng = rand::thread_rng();
    for a in -5..5 {
        let a = 2.0 * a as f32;
        for b in -5..5 {
            let b = 2.0 * b as f32;
            let choose_mat = rng.gen::<f32>();
            let center = Vec3::new(a * 0.9 * rng.gen::<f32>(), 0.2, b + 0.9 * rng.gen::<f32>());
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    scene.items.push(Box::new(MovingSphere::new(
                        center,
                        center + Vec3::new(0.0, 0.5, 0.0),
                        0.0,
                        1.0,
                        0.2,
                        Rc::new(
                            Lambertian::new(Vec3::new(rng.gen::<f32>() * rng.gen::<f32>(), rng.gen::<f32>() * rng.gen::<f32>(), rng.gen::<f32>() * rng.gen::<f32>()).to_linear())))
                        ),
                    );
                } else if choose_mat < 0.95 {
                    scene.items.push(Box::new(Sphere::new(center, 0.2, Rc::new(
                        Metal::new(Vec3::new(0.5 * (1.0 + rng.gen::<f32>()), 0.5 * (1.0 + rng.gen::<f32>()), 0.5 * (1.0 + rng.gen::<f32>())).to_linear(), 0.5 * rng.gen::<f32>()))))
                    );
                } else {
                    scene.items.push(Box::new(Sphere::new(center, 0.2, Rc::new(Dielectric::new(1.5)))));
                }
            }
        }
    }

    scene.items.push(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Rc::new(Dielectric::new(1.5)))));
    scene.items.push(Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, Rc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1).to_linear())))));
    scene.items.push(Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, Rc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5).to_linear(), 0.0)))));

    Box::new(scene)
}

fn main() {
    let nx = 400;
    let ny = 200;
    let ns = 100;

    println!("P3\n{} {}\n255", nx, ny);

    let look_from = Vec3::new(10.0, 2.0, 3.0);
    let look_to = Vec3::new(4.0, 1.0, 1.0);
    let focus_dist = (look_from - look_to).length();

    let camera = Camera::new(&look_from, &look_to, &Vec3::up(), 20.0, nx as f32 / ny as f32, 0.2, focus_dist, 1.0);
    /*let world = HittableList {items: vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Rc::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3).to_linear())))),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0).to_linear())))),
        Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2).to_linear(), 0.1)))),
        Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, Rc::new(Dielectric::new(1.5)))),
        //Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.45, Rc::new(Dielectric::new(1.5)))),
    ]};*/

    let world = random_scene();

    let mut rng = rand::thread_rng();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Vec3::zero();
            
            for _ in 0..ns {
                let u = (i as f32 + rng.gen::<f32>()) / nx as f32;
                let v = (j as f32 + rng.gen::<f32>()) / ny as f32;
                let r = camera.get_ray(u, v, 0.0);
                col += &color(&r, &*world, 0);
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
