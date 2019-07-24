mod vec3;
mod ray;
mod hittable;

use hittable::{Sphere, Hittable, HittableList};
use vec3::Vec3;
use ray::Ray;

fn color(r: &Ray, world: &Hittable) -> Vec3 {
    if let Some(result) = world.hit(r, 0.0, std::f32::MAX) {
        0.5 * (Vec3::one() + &result.normal)
    } else {
        let unit_dir = r.direction().normalized();
        let t = 0.5 * (unit_dir.y() + 1.0);
        (1.0 - t) * Vec3::one() + t * Vec3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    let nx = 200;
    let ny = 100;

    println!("P3\n{} {}\n255", nx, ny);

    let bot_left = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::zero();

    let world = HittableList {items: vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0))
    ]};

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;

            let r = Ray::new(origin, bot_left + u * horizontal + v * vertical);
            let col = color(&r, &world);

            let ir = (255.99 * col.r()) as i32;
            let ig = (255.99 * col.g()) as i32;
            let ib = (255.99 * col.b()) as i32;
            
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
