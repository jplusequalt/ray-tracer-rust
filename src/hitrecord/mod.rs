use crate::lambertian::Lambertian;
use crate::material::MaterialObject;
use crate::ray::Ray;
use crate::utils::types::Point;
use crate::vec3::Vec3;
use crate::utils::types::Color;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Option<MaterialObject>
}

impl HitRecord {
    pub fn new() -> Self {
        Self {
            p: Point::new(),
            normal: Vec3::new(),
            t: 0.0,
            front_face: false,
            material: None
        }
    }

    fn from(p: Point, normal: Vec3, t: f64, front_face: bool, material: MaterialObject) -> Self {
        HitRecord {
            p,
            normal,
            t,
            front_face,
            material: Some(material)
        }
    }

    #[inline]
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(&r.direction, &outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
