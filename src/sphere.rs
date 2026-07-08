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

        let a = ray.direction.len_squared();
        let h = ray.direction.dot(&direction_to_sphere);
        let c = direction_to_sphere.len_squared() - (self.radius * self.radius);

        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            Option::None
        } else {
            Option::Some((h - discriminant.sqrt()) / a)
        }
    }
}
