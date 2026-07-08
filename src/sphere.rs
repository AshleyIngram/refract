use crate::ray::Ray;
use crate::vector3::Point;

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Point, radius: f32) -> Self {
        Self { center, radius }
    }

    pub fn hit(&self, ray: &Ray) -> Option<f32> {
        let direction_to_sphere = self.center - ray.origin;

        // Determine whether there's an intersection using the quadratic formula - (A * t^2 + B * t + C = 0)
        let a = ray.direction.dot(&ray.direction);
        let b = -2.0 * ray.direction.dot(&direction_to_sphere);
        let c = direction_to_sphere.dot(&direction_to_sphere) - (self.radius * self.radius);

        let discriminant = (b * b) - (4.0 * a * c);

        if discriminant < 0.0 {
            Option::None
        } else {
            Option::Some((-b - discriminant.sqrt()) / (2.0 * a))
        }
    }
}
