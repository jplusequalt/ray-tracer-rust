use super::types::Color;
use crate::hitrecord::HitRecord;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::{self, Rng};

// constants
pub const INFINITY: f64 = f64::INFINITY;

pub fn random() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn random_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random()
}
