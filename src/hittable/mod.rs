use crate::hitrecord::HitRecord;
use crate::ray::Ray;

pub trait Hittable {
    fn hit(self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}
