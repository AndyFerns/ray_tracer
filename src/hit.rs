use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
}
