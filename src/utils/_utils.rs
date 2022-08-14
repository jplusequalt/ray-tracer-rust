use super::types::{Color, Point};
use crate::ray::Ray;
use crate::sphere::Sphere;

pub fn write_pixel(pixel_color: &Color) {
    let ir = (255.999 * pixel_color.x) as u16;
    let ig = (255.999 * pixel_color.y) as u16;
    let ib = (255.999 * pixel_color.z) as u16;

    println!("{} {} {}", ir, ig, ib);
}

pub fn ray_color(r: &Ray) -> Color {
    let s = Sphere::from(Point::from(0.0, 0.0, -1.0), 0.5);
    todo!();
}
