use crate::material::Material;
use crate::ray::Ray;
use crate::utils::types::Color;
use crate::vec3::Vec3;
use crate::hitrecord::HitRecord;

#[derive(Clone, Copy)]
pub struct Metal {
    pub albedo: Color
}

impl Material for Metal {
    fn scatter(&self, r: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = Vec3::reflect(&Vec3::unit_vector(&r.direction), &rec.normal);

        *scattered = Ray::from(&rec.p, &reflected);
        *attenuation = self.albedo;

        Vec3::dot(&scattered.direction, &rec.normal) > 0f64
    }
}