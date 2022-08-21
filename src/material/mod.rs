use crate::dialetric::Dialetric;
use crate::lambertian::Lambertian;
use crate::metal::Metal;
use crate::ray::Ray;
use crate::utils::types::Color;
use crate::hitrecord::HitRecord;

pub trait Material {
    fn scatter(&self, r: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

#[derive(Clone, Copy)]
pub enum MaterialObject {
    Lambertian(Lambertian),
    Metal(Metal),
    Dialetric(Dialetric)
}

impl Material for MaterialObject {
    fn scatter(&self, r: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        match self {
            MaterialObject::Lambertian(l) => l.scatter(r, rec, attenuation, scattered),
            MaterialObject::Metal(m) => m.scatter(r, rec, attenuation, scattered),
            MaterialObject::Dialetric(d) => d.scatter(r, rec, attenuation, scattered)
        }
    }
}