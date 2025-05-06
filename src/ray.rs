#![allow(dead_code)]

use crate::color::Color;
use crate::point3::Point3;
use crate::vector3::Vector3;

#[derive(Default)]
pub struct Ray {
    origin: Point3,
    direction: Vector3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vector3) -> Self {
        Self { origin, direction }
    }

    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vector3 {
        &self.direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }

    pub fn hit_sphere(&self, center: &Point3, radius: f64) -> bool {
        let oc = *center - *self.origin();
        let a = self.direction().dot(self.direction());
        let b = -2.0 * self.direction().dot(&oc);
        let c = oc.dot(&oc) - radius * radius;
        let discriminant = b * b - 4.0 * a * c;
        discriminant >= 0.0
    }

    pub fn ray_color(&self) -> Color {
        if self.hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5) {
            return Color::new(1.0, 0.0, 0.0);
        }

        let unit_direction = self.direction.unit_vector();
        let alpha = 0.5 * (unit_direction.y() + 1.0);
        let white = Color::new(1.0, 1.0, 1.0);
        let blue = Color::new(0.5, 0.7, 1.0);
        Color::lerp(white, white, alpha)
    }
}
