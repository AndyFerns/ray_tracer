mod vec3;
mod ray;
mod sphere;
mod hit;
mod camera;

use vec3::Vec3;
use sphere::Sphere;
use camera::Camera;
use image::{RgbImage, Rgb};

fn ray_color(ray: &ray::Ray, sphere: &Sphere) -> Vec3 {
    if let Some(hit) = sphere.hit(ray, 0.001, f64::INFINITY) {
        return (hit.normal + Vec3(1.0, 1.0, 1.0)) * 0.5;
    }

    let unit_direction = ray.direction.normalize();
    let t = 0.5 * (unit_direction.1 + 1.0);
    Vec3(1.0, 1.0, 1.0) * (1.0 - t) + Vec3(0.5, 0.7, 1.0) * t
}

fn main() {
    let width = 800;
    let height = 400;
    let camera = Camera::new();
    let sphere = Sphere {
        center: Vec3(0.0, 0.0, -1.0),
        radius: 0.5,
    };

    let mut img = RgbImage::new(width, height);

    for j in 0..height {
        for i in 0..width {
            let u = i as f64 / width as f64;
            let v = (height - j - 1) as f64 / height as f64; // flip vertically
            let r = camera.get_ray(u, v);
            let color = ray_color(&r, &sphere);
            let pixel = Rgb([
                (255.99 * color.0) as u8,
                (255.99 * color.1) as u8,
                (255.99 * color.2) as u8,
            ]);
            img.put_pixel(i, j, pixel);
        }
    }

    img.save("output.png").unwrap();
    println!("Image saved to output.png");
}
