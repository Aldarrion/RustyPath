mod vec3;
mod ray;

use vec3::Vec3;
use ray::Ray;

fn sqr(x: f32) -> f32 {
    x * x
}

fn hit_sphere(center: Vec3, radius: f32, r: &Ray) -> f32 {
    let oc = r.origin() - &center;
    let a = r.direction().dot(r.direction());
    let b = 2.0 * oc.dot(r.direction());
    let c = oc.dot(&oc) - sqr(radius);
    let discriminant = sqr(b) - 4.0 * a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

fn color(r: &Ray) -> Vec3 {
    let t = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let n = r.point_at(t) - Vec3::new(0.0, 0.0, -1.0);
        0.5 * (n + Vec3::one())
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

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;

            let r = Ray::new(origin, bot_left + u * horizontal + v * vertical);
            let col = color(&r);

            let ir = (255.99 * col.r()) as i32;
            let ig = (255.99 * col.g()) as i32;
            let ib = (255.99 * col.b()) as i32;
            
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
