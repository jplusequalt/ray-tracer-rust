use crate::material::Material;
use crate::ray::Ray;
use crate::utils::types::Color;
use crate::vec3::Vec3;
use crate::hitrecord::HitRecord;

#[derive(Clone, Copy)]
pub struct Lambertian {
    pub albedo: Color
}

impl Material for Lambertian {
    fn scatter(&self, r: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::from(&rec.p, &scatter_direction);
        *attenuation = self.albedo;

        true
    }
}