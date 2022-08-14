use super::types::{Color, Point};
use crate::hitrecord::HitRecord;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

// constants
pub const INFINITY: f32 = f32::INFINITY;

pub fn write_pixel(pixel_color: &Color) {
    let ir = (255.999 * pixel_color.x) as u16;
    let ig = (255.999 * pixel_color.y) as u16;
    let ib = (255.999 * pixel_color.z) as u16;

    println!("{} {} {}", ir, ig, ib);
}

pub fn ray_color<T: Hittable>(r: &Ray, world: &T) -> Color {
    let mut rec = HitRecord::new();

    if world.hit(r, 0.0, INFINITY, &mut rec) {
        return (rec.normal + Color::from(1.0, 1.0, 1.0)) * 0.5;
    }

    let unit_direction = Vec3::unit_vector(&r.direction);
    let t = (unit_direction.y + 1.0) * 0.5;

    Color::from(1.0, 1.0, 1.0) * (1.0 - t) + Color::from(0.5, 0.7, 1.0) * t
}
