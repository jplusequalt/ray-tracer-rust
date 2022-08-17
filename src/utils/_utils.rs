use super::types::Color;
use crate::hitrecord::HitRecord;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::{self, Rng};

// constants
pub const INFINITY: f64 = f64::INFINITY;

pub fn write_pixel(pixel_color: &Color, samples_per_pixel: i64) {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    // divide the color by the number of pixels
    let scale = 1.0 / (samples_per_pixel as f64);
    r = f64::sqrt(scale * r);
    g = f64::sqrt(scale * g);
    b = f64::sqrt(scale * b);

    println!(
        "{} {} {}",
        (256.0 * r.clamp(0.0, 0.999)) as i64,
        (256.0 * g.clamp(0.0, 0.999)) as i64,
        (256.0 * b.clamp(0.0, 0.999)) as i64
    );
}

pub fn ray_color<T: Hittable>(r: &Ray, world: &T, depth: i64) -> Color {
    let mut rec = HitRecord::new();

    if depth <= 0 {
        return Color::new();
    }

    if world.hit(r, 0.001, INFINITY, &mut rec) {
        let target = rec.p + rec.normal + Vec3::random_in_unit_sphere();
        return ray_color(&Ray::from(&rec.p, &(target - rec.p)), world, depth - 1) * 0.5;
    }

    let unit_direction = Vec3::unit_vector(&r.direction);
    let t = (unit_direction.y + 1.0) * 0.5;

    Color::from(1.0, 1.0, 1.0) * (1.0 - t) + Color::from(0.5, 0.7, 1.0) * t
}

pub fn random() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn random_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random()
}
