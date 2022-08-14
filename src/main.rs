mod camera;
mod hitrecord;
mod hittable;
mod ray;
mod sphere;
mod utils;
mod vec3;

use crate::camera::Camera;
use crate::hittable::HittableList;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::utils::_utils::{random, ray_color, write_pixel};
use crate::utils::types::{Color, Point};
use crate::vec3::Vec3;

fn main() {
    // image
    const ASPECT_RATIO: f32 = 16.0 / 9.0;
    const IMG_WIDTH: u16 = 400;
    const IMG_HEIGHT: u16 = ((IMG_WIDTH as f32) / ASPECT_RATIO) as u16;
    const SAMPLES_PER_PIXEL: i32 = 100;

    // world
    let mut world: HittableList<Sphere> = HittableList::<Sphere> {
        objects: Vec::<Sphere>::new(),
    };

    world.add(Sphere::from(Point::from(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::from(Point::from(0.0, -100.5, -1.0), 100.0));

    // camera
    let cam = Camera::new();

    // render
    println!("P3\n{} {}\n255", IMG_WIDTH, IMG_HEIGHT);

    for j in (0..=IMG_HEIGHT - 1).rev() {
        for i in 0..IMG_WIDTH {
            let mut pixel_color = Color::new();
            for s in 0..SAMPLES_PER_PIXEL {
                let u = (i as f32 + random()) / ((IMG_WIDTH - 1) as f32);
                let v = (j as f32 + random()) / ((IMG_HEIGHT - 1) as f32);
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world);
            }

            write_pixel(&pixel_color, SAMPLES_PER_PIXEL);
        }
    }
}
