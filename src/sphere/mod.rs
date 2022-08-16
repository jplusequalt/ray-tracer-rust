use crate::hitrecord::HitRecord;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::utils::types::Point;
use crate::vec3::Vec3;

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
}

impl Sphere {
    pub fn from(center: Point, radius: f32) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = &r.origin - &self.center;
        let a = r.direction.length_squared();
        let half_b = Vec3::dot(&oc, &r.direction);
        let c = &oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = f32::sqrt(discriminant);

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (&rec.p - &self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);

        return true;
    }
}
