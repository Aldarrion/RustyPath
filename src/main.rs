use rayon::prelude::*;
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
use std::sync::Arc;

fn color(r: &Ray, world: Arc<dyn Hittable>, depth: i32) -> Vec3 {
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

fn random_scene() -> Arc<dyn Hittable> {
    let mut scene = HittableList { items: vec![] };
    scene.items.push(Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Arc::new(Lambertian::new(Vec3::new(0.5, 0.5, 0.5).to_linear())))));

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
                        0.3,
                        0.2,
                        Arc::new(
                            Lambertian::new(Vec3::new(rng.gen::<f32>() * rng.gen::<f32>(), rng.gen::<f32>() * rng.gen::<f32>(), rng.gen::<f32>() * rng.gen::<f32>()).to_linear())))
                        ),
                    );
                } else if choose_mat < 0.95 {
                    scene.items.push(Box::new(Sphere::new(center, 0.2, Arc::new(
                        Metal::new(Vec3::new(0.5 * (1.0 + rng.gen::<f32>()), 0.5 * (1.0 + rng.gen::<f32>()), 0.5 * (1.0 + rng.gen::<f32>())).to_linear(), 0.5 * rng.gen::<f32>()))))
                    );
                } else {
                    scene.items.push(Box::new(Sphere::new(center, 0.2, Arc::new(Dielectric::new(1.5)))));
                }
            }
        }
    }

    scene.items.push(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Arc::new(Dielectric::new(1.5)))));
    scene.items.push(Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, Arc::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1).to_linear())))));
    scene.items.push(Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, Arc::new(Metal::new(Vec3::new(0.7, 0.6, 0.5).to_linear(), 0.0)))));

    Arc::new(scene)
}

fn main() {
    const NX: usize = 800;
    const NY: usize = 400;
    let ns = 200;

    println!("P3\n{} {}\n255", NX, NY);

    let look_from = Vec3::new(10.0, 2.0, 3.0);
    let look_to = Vec3::new(4.0, 1.0, 1.0);
    let focus_dist = (look_from - look_to).length();

    let camera = Camera::new(&look_from, &look_to, &Vec3::up(), 20.0, NX as f32 / NY as f32, 0.2, focus_dist, 1.0);
    /*let world = HittableList {items: vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, Rc::new(Lambertian::new(Vec3::new(0.8, 0.3, 0.3).to_linear())))),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0).to_linear())))),
        Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2).to_linear(), 0.1)))),
        Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, Rc::new(Dielectric::new(1.5)))),
        //Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.45, Rc::new(Dielectric::new(1.5)))),
    ]};*/

    let world = random_scene();

    let mut image = vec![(0, 0, 0); NX * NY];
    
    image
    .par_iter_mut()
    .enumerate()
    .for_each(|(i, (r, g, b))|{
        let mut rng = rand::thread_rng();
        let x = i % NX;
        let y = NY - (i / NX);
        
        let mut col = Vec3::zero();
        
        for _ in 0..ns {
            let u = (x as f32 + rng.gen::<f32>()) / NX as f32;
            let v = (y as f32 + rng.gen::<f32>()) / NY as f32;
            let r = camera.get_ray(u, v, 0.0);
            col += &color(&r, world.clone(), 0);
        }
        col /= ns as f32;
        col = col.to_srgb();
        *r = (255.99 * col.r()) as i32;
        *g = (255.99 * col.g()) as i32;
        *b = (255.99 * col.b()) as i32;
    });

    for (r, g, b) in image.iter() {
        println!("{} {} {}", r, g, b);
    }
}
