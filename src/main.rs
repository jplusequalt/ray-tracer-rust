mod ray;
mod utils;
mod vec3;

use crate::ray::Ray;
use crate::utils::_utils::write_pixel;
use crate::utils::types::{Color, Point};
use crate::vec3::Vec3;

fn hit_sphere(center: &Point, radius: f32, r: &Ray) -> f32 {
    let oc = &r.origin - center;
    let a = Vec3::dot(&r.direction, &r.direction);
    let b = 2.0 * Vec3::dot(&oc, &r.direction);
    let c = Vec3::dot(&oc, &oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-b - f32::sqrt(discriminant)) / (2.0 * a);
    }
}

fn ray_color(r: &Ray) -> Color {
    let t = hit_sphere(&Point::from(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let normal = Vec3::unit_vector(&(r.at(t) - Vec3::from(0.0, 0.0, -1.0)));
        return Color::from(normal.x + 1.0, normal.y + 1.0, normal.z + 1.0) * 0.5;
    }

    let unit_direction = Vec3::unit_vector(&r.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    Color::from(1.0, 1.0, 1.0) * (1.0 - t) + Color::from(0.5, 0.7, 1.0) * t
}

fn main() {
    // image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMG_WIDTH: u16 = 400;
    const IMG_HEIGHT: u16 = ((IMG_WIDTH as f32) / ASPECT_RATIO) as u16;

    // camera

    const VIEWPOINT_HEIGHT: f32 = 2.0;
    const VIEWPOINT_WIDTH: f32 = ASPECT_RATIO * VIEWPOINT_HEIGHT;
    const FOCAL_LENGTH: f32 = 1.0;

    let origin = Point::new();
    let horizontal = Vec3::from(VIEWPOINT_WIDTH, 0.0, 0.0);
    let vertical = Vec3::from(0.0, VIEWPOINT_HEIGHT, 0.0);
    let lower_left_corner =
        &origin - &(&horizontal / 2.0) - &vertical / 2.0 - Vec3::from(0.0, 0.0, FOCAL_LENGTH);

    // render

    println!("P3\n{} {}\n255", IMG_WIDTH, IMG_HEIGHT);

    for j in (0..=IMG_HEIGHT - 1).rev() {
        for i in 0..IMG_WIDTH {
            let u = (i as f32) / ((IMG_WIDTH - 1) as f32);
            let v = (j as f32) / ((IMG_HEIGHT - 1) as f32);

            let r = Ray::from(
                &origin,
                &(&lower_left_corner + &(&horizontal * u) + (&vertical * v) - &origin),
            );

            let c = ray_color(&r);

            write_pixel(&c);
        }
    }
}
