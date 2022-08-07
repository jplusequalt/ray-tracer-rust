use crate::utils::types::Point;
use crate::vec3::Vec3;

pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
}

impl Ray {
    fn new() -> Self {
        Self {
            origin: Point::new(),
            direction: Vec3::new(),
        }
    }

    fn from(origin: Point, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    fn at(&self, t: f32) -> Point {
        &self.origin + &(&self.direction * t)
    }
}

#[cfg(test)]
mod tests {
    use super::Ray;
    use crate::utils::types::Point;
    use crate::vec3::Vec3;

    #[test]
    fn test_at() {
        let ray = Ray {
            origin: Point::new(),
            direction: Vec3::from(1.0, 2.0, 3.0),
        };

        assert_eq!(ray.at(3.0), Point::from(3.0, 6.0, 9.0))
    }
}
