use crate::{vec3::Vec3, ray::Ray, hit::HitRecord};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
}

impl Sphere {
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(ray.direction);
        let b = oc.dot(ray.direction);
        let c = oc.dot(oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let root = (-b - discriminant.sqrt()) / a;
            if root < t_max && root > t_min {
                let point = ray.at(root);
                let normal = (point - self.center) / self.radius;
                return Some(HitRecord { point, normal });
            }
        }
        None
    }
}
