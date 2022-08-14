mod hitrecord;
mod hittable;
mod ray;
mod sphere;
mod utils;
mod vec3;

use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::utils::_utils::{ray_color, write_pixel};
use crate::utils::types::{Color, Point};
use crate::vec3::Vec3;

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
