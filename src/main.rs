mod vec3;
mod ray;
mod sphere;
mod hit;
mod camera;

use vec3::Vec3;
use sphere::Sphere;
use camera::Camera;

fn ray_color(ray: &ray::Ray, sphere: &Sphere) -> Vec3 {
    if let Some(hit) = sphere.hit(ray, 0.001, f64::INFINITY) {
        return (hit.normal + Vec3(1.0, 1.0, 1.0)) * 0.5;
    }

    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.1 + 1.0);
    Vec3(1.0, 1.0, 1.0) * (1.0 - t) + Vec3(0.5, 0.7, 1.0) * t
}

fn main() {
    let width = 200;
    let height = 100;
    let camera = Camera::new();
    let sphere = Sphere {
        center: Vec3(0.0, 0.0, -1.0),
        radius: 0.5,
    };

    println!("P3\n{} {}\n255", width, height);
    for j in (0..height).rev() {
        for i in 0..width {
            let u = i as f64 / width as f64;
            let v = j as f64 / height as f64;
            let r = camera.get_ray(u, v);
            let color = ray_color(&r, &sphere);
            let ir = (255.99 * color.0) as i32;
            let ig = (255.99 * color.1) as i32;
            let ib = (255.99 * color.2) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
