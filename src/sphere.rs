use crate::hittable::{HitResult, Hittable};
use crate::interval::Interval;
use crate::point::Point;
use crate::ray::Ray;

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Point, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitResult> {
        let direction_to_sphere = self.center - ray.origin;

        let a = ray.direction.len_squared();
        let h = ray.direction.dot(direction_to_sphere);
        let c = direction_to_sphere.len_squared() - (self.radius * self.radius);

        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let discriminant_root = discriminant.sqrt();
        // root1 is the closest intersection to the camera. This is generally the only intersection needed.
        let root1 = (h - discriminant_root) / a;
        let root = if interval.surrounds(root1) {
            Some(root1)
        } else {
            // If the camera is inside the sphere, or we're doing ray-interval slicing (e.g. spheres in spheres),
            // we need to check the other root (where the ray "exits" the sphere).
            let root2 = (h + discriminant_root) / a;
            if interval.surrounds(root2) {
                Some(root2)
            } else {
                None
            }
        };

        root.map(|t| {
            let point = ray.at(t);
            let normal = ((point - self.center) / self.radius).normalize();
            HitResult::new(ray, point, t, normal)
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::direction::Direction;

    use super::*;

    #[test]
    fn sphere_hit_from_outside_first_intersection() {
        let sphere = Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5);
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Direction::new(0.0, 0.0, -1.0));
        let interval = Interval::new(0.001, f32::INFINITY);

        let hit_result = sphere.hit(&ray, &interval);

        assert!(hit_result.is_some());
        let result = hit_result.unwrap();
        assert_eq!(result.t, 0.5);
        assert_eq!(result.point.z, -0.5);
        assert_eq!(result.front_face, true);
        assert_eq!(result.normal.z > 0.0, true);
    }

    #[test]
    fn sphere_hit_from_inside_second_intersection() {
        let sphere = Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5);
        let ray = Ray::new(Point::new(0.0, 0.0, -0.9), Direction::new(0.0, 0.0, -1.0));
        let interval = Interval::new(0.001, f32::INFINITY);

        let hit_result = sphere.hit(&ray, &interval);

        assert!(hit_result.is_some());
        let result = hit_result.unwrap();
        assert_eq!(result.t, 0.6);
        assert_eq!(result.point.z, -1.5);
        assert_eq!(result.front_face, false);
        assert_eq!(result.normal.z > 0.0, true);
    }

    #[test]
    fn sphere_hit_misses_none() {
        let sphere = Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5);
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Direction::new(0.0, 0.0, 1.0));
        let interval = Interval::new(0.001, f32::INFINITY);

        let hit_result = sphere.hit(&ray, &interval);

        assert!(hit_result.is_none());
    }
}
