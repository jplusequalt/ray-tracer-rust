mod camera;
mod hitrecord;
mod hittable;
mod ray;
mod sphere;
mod utils;
mod vec3;
mod lambertian;
mod material;
mod metal;

use material::Material;

use crate::camera::Camera;
use crate::hitrecord::HitRecord;
use crate::hittable::{Hittable, HittableList};
use crate::lambertian::Lambertian;
use crate::metal::Metal;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::utils::_utils::{random, INFINITY};
use crate::utils::types::{Color, Point};
use crate::vec3::Vec3;
use crate::material::MaterialObject;

fn write_pixel(pixel_color: &Color, samples_per_pixel: i64) {
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

fn ray_color<T: Hittable>(r: &Ray, world: &T, depth: i64) -> Color {
    let mut rec = HitRecord::new();

    if depth <= 0 {
        return Color::new();
    }

    if world.hit(r, 0.001, INFINITY, &mut rec) {
        let mut scattered = Ray::new();
        let mut attenuation = Color::new();
        // let target = rec.p + Vec3::random_in_hemispere(&rec.normal);
        // return ray_color(&Ray::from(&rec.p, &(target - rec.p)), world, depth - 1) * 0.5;

        if rec.material.unwrap().scatter(r, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }

        return Color::new();
    }

    let unit_direction = Vec3::unit_vector(&r.direction);
    let t = (unit_direction.y + 1.0) * 0.5;

    Color::from(1.0, 1.0, 1.0) * (1.0 - t) + Color::from(0.5, 0.7, 1.0) * t
}

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

    let material_ground = Lambertian {
        albedo: Color::from(0.8, 0.8, 0.0)
    };

    let material_center = Lambertian {
        albedo: Color::from(0.7, 0.3, 0.3)
    };

    let material_left = Metal {
        albedo: Color::from(0.8, 0.8, 0.8)
    };
    let material_right = Metal {
        albedo: Color::from(0.8, 0.6, 0.2)
    };

    world.add(Sphere::from(Point::from(0.0, -100.5, -1.0), 100.0, MaterialObject::Lambertian(material_ground)));
    world.add(Sphere::from(Point::from(0.0, 0.0, -1.0), 0.5, MaterialObject::Lambertian(material_center)));
    world.add(Sphere::from(Point::from(-1.0, 0.0, -1.0), 0.5, MaterialObject::Metal(material_left)));
    world.add(Sphere::from(Point::from(1.0, 0.0, -1.0), 0.5, MaterialObject::Metal(material_right)));

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
