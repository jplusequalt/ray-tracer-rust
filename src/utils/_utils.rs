use super::types::{Color, Point};
use crate::hitrecord::HitRecord;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::vec3::Vec3;
use rand::{self, Rng};

// constants
pub const INFINITY: f32 = f32::INFINITY;

pub fn write_pixel(pixel_color: &Color, samples_per_pixel: i32) {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    // divide the color by the number of pixels
    let scale = 1.0 / (samples_per_pixel as f32);
    r *= scale;
    g *= scale;
    b *= scale;

    println!(
        "{} {} {}",
        (256.0 * clamp(r, 0.0, 0.999)) as u16,
        (256.0 * clamp(g, 0.0, 0.999)) as u16,
        (256.0 * clamp(b, 0.0, 0.999)) as u16
    );
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

pub fn random() -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen()
}

pub fn random_range(min: f32, max: f32) -> f32 {
    min + (max - min) * random()
}

pub fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    } else if x > max {
        return max;
    }

    x
}
