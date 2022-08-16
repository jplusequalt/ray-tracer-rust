use crate::ray::Ray;
use crate::utils::types::Point;
use crate::vec3::Vec3;

pub struct Camera {
    pub origin: Point,
    pub lower_left_corner: Point,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        const ASPECT_RATIO: f32 = 16.0 / 9.0;
        const VIEWPOINT_HEIGHT: f32 = 2.0;
        const VIEWPOINT_WIDTH: f32 = ASPECT_RATIO * VIEWPOINT_HEIGHT;
        const FOCAL_LENGTH: f32 = 1.0;

        let origin = Point::new();
        let horizontal = Vec3::from(VIEWPOINT_WIDTH, 0.0, 0.0);
        let vertical = Vec3::from(0.0, VIEWPOINT_HEIGHT, 0.0);
        let lower_left_corner =
            &origin - &(&horizontal / 2.0) - &vertical / 2.0 - Vec3::from(0.0, 0.0, FOCAL_LENGTH);

        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::from(
            &self.origin,
            &(&self.lower_left_corner + &(&self.horizontal * u) + &self.vertical * v
                - &self.origin),
        )
    }
}
