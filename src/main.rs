mod camera;
mod hitrecord;
mod hittable;
mod ray;
mod sphere;
mod utils;
mod vec3;

use std::io::Write;

use crate::camera::Camera;
use crate::hittable::HittableList;
use crate::sphere::Sphere;
use crate::utils::_utils::{random, ray_color, write_pixel};
use crate::utils::types::{Color, Point};

fn main() {
    // image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMG_WIDTH: i64 = 400;
    const IMG_HEIGHT: i64 = ((IMG_WIDTH as f64) / ASPECT_RATIO) as i64;
    const SAMPLES_PER_PIXEL: i64 = 100;
    const MAX_DEPTH: i64 = 50;

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
            for _s in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random()) / ((IMG_WIDTH - 1) as f64);
                let v = (j as f64 + random()) / ((IMG_HEIGHT - 1) as f64);
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }

            write_pixel(&pixel_color, SAMPLES_PER_PIXEL);
        }
    }
}
