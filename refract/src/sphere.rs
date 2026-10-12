use std::sync::Arc;

use crate::bounding_box::BoundingBox;
use crate::direction::Direction;
use crate::hittable::{HitResult, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::point::Point;
use crate::ray::Ray;

pub struct Sphere {
    center: Ray,
    radius: f32,
    material: Arc<dyn Material>,
    bounding_box: BoundingBox,
}

impl Sphere {
    pub fn new_stationary(center: Point, radius: f32, material: Arc<dyn Material>) -> Self {
        let radius_vector = Direction::new(radius, radius, radius);
        let bounding_box = BoundingBox::new(center - radius_vector, center + radius_vector);
        Self {
            center: Ray::new(center, Direction::new(0.0, 0.0, 0.0)),
            radius,
            material,
            bounding_box,
        }
    }

    pub fn new_moving(
        from_center: Point,
        to_center: Point,
        radius: f32,
        material: Arc<dyn Material>,
    ) -> Self {
        let center = Ray::new(from_center, to_center - from_center);
        let radius_vector = Direction::new(radius, radius, radius);
        let bounding_box_1 = BoundingBox::new(
            center.at(0.0) - radius_vector,
            center.at(0.0) + radius_vector,
        );
        let bounding_box_2 = BoundingBox::new(
            center.at(1.0) - radius_vector,
            center.at(1.0) + radius_vector,
        );
        let bounding_box = BoundingBox::new_from_bounding_boxes(&bounding_box_1, &bounding_box_2);
        Self {
            center,
            radius,
            material,
            bounding_box,
        }
    }

    fn get_sphere_uv(point: Point) -> (f32, f32) {
        const PI: f32 = std::f32::consts::PI;

        let theta = (-point.y).acos();
        let phi = (-point.z).atan2(point.x) + PI;

        let u = phi / (2.0 * PI);
        let v = theta / PI;

        (u, v)
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitResult> {
        let current_center = self.center.at(ray.time());
        let direction_to_sphere = current_center - ray.origin();

        let a = ray.direction().len_squared();
        let h = ray.direction().dot(direction_to_sphere);
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
            let normal = ((point - current_center) / self.radius).normalize();
            let (u, v) = Self::get_sphere_uv(Point::new(normal.x, normal.y, normal.z));
            HitResult::new(ray, point, t, u, v, normal, Arc::clone(&self.material))
        })
    }

    fn bounding_box(&self) -> BoundingBox {
        self.bounding_box
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        color::Color,
        direction::Direction,
        material::{Matte, ReflectionType},
    };

    use super::*;

    fn moving_sphere_along_z() -> Sphere {
        Sphere::new_moving(
            Point::new(0.0, 0.0, -1.0),
            Point::new(0.0, 0.0, -3.0),
            0.5,
            Arc::new(Matte::new(
                Color::new(1.0, 1.0, 1.0),
                ReflectionType::Diffuse,
            )),
        )
    }

    #[test]
    fn sphere_hit_from_outside_first_intersection() {
        let sphere = Sphere::new_stationary(
            Point::new(0.0, 0.0, -1.0),
            0.5,
            Arc::new(Matte::new(
                Color::new(1.0, 1.0, 1.0),
                ReflectionType::Diffuse,
            )),
        );
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Direction::new(0.0, 0.0, -1.0));
        let interval = Interval::new(0.001, f32::INFINITY);

        let hit_result = sphere.hit(&ray, &interval);

        assert!(hit_result.is_some());
        let result = hit_result.unwrap();
        assert_eq!(result.t, 0.5);
        assert_eq!(result.point.z, -0.5);
        assert!(result.front_face);
        assert!(result.normal.z > 0.0);
    }

    #[test]
    fn sphere_hit_from_inside_second_intersection() {
        let sphere = Sphere::new_stationary(
            Point::new(0.0, 0.0, -1.0),
            0.5,
            Arc::new(Matte::new(
                Color::new(1.0, 1.0, 1.0),
                ReflectionType::Diffuse,
            )),
        );
        let ray = Ray::new(Point::new(0.0, 0.0, -0.9), Direction::new(0.0, 0.0, -1.0));
        let interval = Interval::new(0.001, f32::INFINITY);

        let hit_result = sphere.hit(&ray, &interval);

        assert!(hit_result.is_some());
        let result = hit_result.unwrap();
        assert_eq!(result.t, 0.6);
        assert_eq!(result.point.z, -1.5);
        assert!(!result.front_face);
        assert!(result.normal.z > 0.0);
    }

    #[test]
    fn sphere_hit_misses_none() {
        let sphere = Sphere::new_stationary(
            Point::new(0.0, 0.0, -1.0),
            0.5,
            Arc::new(Matte::new(
                Color::new(1.0, 1.0, 1.0),
                ReflectionType::Diffuse,
            )),
        );
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Direction::new(0.0, 0.0, 1.0));
        let interval = Interval::new(0.001, f32::INFINITY);

        let hit_result = sphere.hit(&ray, &interval);

        assert!(hit_result.is_none());
    }

    #[test]
    fn moving_sphere_hit_depends_on_ray_time() {
        let sphere = moving_sphere_along_z();
        let direction = Direction::new(0.0, 0.0, -1.0);
        let interval = Interval::new(0.001, f32::INFINITY);

        let hit_at_start = sphere.hit(
            &Ray::new_at_time(Point::new(0.0, 0.0, 0.0), direction, 0.0),
            &interval,
        );
        let hit_at_end = sphere.hit(
            &Ray::new_at_time(Point::new(0.0, 0.0, 0.0), direction, 1.0),
            &interval,
        );

        assert_eq!(hit_at_start.unwrap().t, 0.5);
        assert_eq!(hit_at_end.unwrap().t, 2.5);
    }

    #[test]
    fn stationary_sphere_hit_is_unchanged_across_ray_times() {
        let sphere = Sphere::new_stationary(
            Point::new(0.0, 0.0, -1.0),
            0.5,
            Arc::new(Matte::new(
                Color::new(1.0, 1.0, 1.0),
                ReflectionType::Diffuse,
            )),
        );
        let direction = Direction::new(0.0, 0.0, -1.0);
        let interval = Interval::new(0.001, f32::INFINITY);

        let hit_at_start = sphere
            .hit(
                &Ray::new_at_time(Point::new(0.0, 0.0, 0.0), direction, 0.0),
                &interval,
            )
            .unwrap();
        let hit_at_end = sphere
            .hit(
                &Ray::new_at_time(Point::new(0.0, 0.0, 0.0), direction, 1.0),
                &interval,
            )
            .unwrap();

        assert_eq!(hit_at_start.t, hit_at_end.t);
        assert_eq!(hit_at_start.point, hit_at_end.point);
    }

    #[test]
    fn moving_sphere_hit_at_half_time_is_midpoint() {
        let sphere = moving_sphere_along_z();
        let direction = Direction::new(0.0, 0.0, -1.0);
        let interval = Interval::new(0.001, f32::INFINITY);
        let origin = Point::new(0.0, 0.0, 0.0);

        let hit_at_start = sphere
            .hit(&Ray::new_at_time(origin, direction, 0.0), &interval)
            .unwrap();
        let hit_at_half = sphere
            .hit(&Ray::new_at_time(origin, direction, 0.5), &interval)
            .unwrap();
        let hit_at_end = sphere
            .hit(&Ray::new_at_time(origin, direction, 1.0), &interval)
            .unwrap();

        assert_eq!(hit_at_half.t, (hit_at_start.t + hit_at_end.t) / 2.0);
        assert_eq!(
            hit_at_half.point.z,
            (hit_at_start.point.z + hit_at_end.point.z) / 2.0
        );
    }

    #[test]
    fn stationary_sphere_bounding_box_is_correct() {
        let sphere = Sphere::new_stationary(
            Point::new(0.0, 0.0, -1.0),
            0.5,
            Arc::new(Matte::new(
                Color::new(1.0, 1.0, 1.0),
                ReflectionType::Diffuse,
            )),
        );

        assert_eq!(
            sphere.bounding_box(),
            BoundingBox::new(Point::new(-0.5, -0.5, -1.5), Point::new(0.5, 0.5, -0.5))
        );
    }

    #[test]
    fn moving_sphere_bounding_box_is_correct() {
        let sphere = moving_sphere_along_z();

        assert_eq!(
            sphere.bounding_box(),
            BoundingBox::new(Point::new(-0.5, -0.5, -3.5), Point::new(0.5, 0.5, -0.5))
        );
    }

    #[test]
    fn get_sphere_uv_cardinal_directions_correct() {
        struct UvCase {
            point: Point,
            expected_x: f32,
            expected_y: f32,
        }

        let cases: [UvCase; 6] = [
            UvCase { point: Point::new(1.0, 0.0, 0.0), expected_x: 0.5, expected_y: 0.5 },
            UvCase { point: Point::new(-1.0, 0.0, 0.0), expected_x: 0.0, expected_y: 0.5 },
            UvCase { point: Point::new(0.0, 1.0, 0.0), expected_x: 0.5, expected_y: 1.0 },
            UvCase { point: Point::new(0.0, -1.0, 0.0), expected_x: 0.5, expected_y: 0.0 },
            UvCase { point: Point::new(0.0, 0.0, 1.0), expected_x: 0.25, expected_y: 0.5 },
            UvCase { point: Point::new(0.0, 0.0, -1.0), expected_x: 0.75, expected_y: 0.5 },
        ];

        for test_case in cases {
            let actual = Sphere::get_sphere_uv(test_case.point);

            assert!((actual.0 - test_case.expected_x).abs() < f32::EPSILON);
            assert!((actual.1 - test_case.expected_y).abs() < f32::EPSILON);
        }
    }
}
