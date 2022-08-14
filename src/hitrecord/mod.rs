use crate::utils::types::Point;
use crate::vec3::Vec3;

pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    pub t: f32,
    front_face: bool,
}
