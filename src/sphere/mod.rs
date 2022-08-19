use crate::hitrecord::HitRecord;
use crate::hittable::Hittable;
use crate::material::MaterialObject;
use crate::ray::Ray;
use crate::utils::types::Point;
use crate::vec3::Vec3;

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub material: MaterialObject
}

impl Sphere {
    pub fn from(center: Point, radius: f64, material: MaterialObject) -> Self {
        Sphere { center, radius, material }
    }
}

impl<'a> Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = Vec3::dot(&oc, &r.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = f64::sqrt(discriminant);

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.material = Some(self.material);

        true
    }
}
