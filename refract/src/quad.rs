use std::sync::Arc;

use crate::{
    bounding_box::BoundingBox,
    direction::{Direction, UnitDirection},
    hittable::{HitResult, Hittable},
    interval::Interval,
    material::Material,
    point::Point,
    ray::Ray,
};

pub struct Quad {
    origin: Point,
    edge_a: Direction,
    edge_b: Direction,
    material: Arc<dyn Material>,
    bounding_box: BoundingBox,
    normal: UnitDirection,
    plane_offset: f32,
    scale_vector: Direction,
}

impl Quad {
    pub fn new(
        origin: Point,
        edge_a: Direction,
        edge_b: Direction,
        material: Arc<dyn Material>,
    ) -> Self {
        let bounding_box = BoundingBox::new_from_bounding_boxes(
            &BoundingBox::new(origin, origin + edge_a + edge_b),
            &BoundingBox::new(origin + edge_a, origin + edge_b),
        );

        let n = edge_a.cross(edge_b);
        let normal = n.normalize();
        let origin_vector = Direction::new(origin.x, origin.y, origin.z);
        let plane_offset = normal.dot(origin_vector);
        let scale_vector = n / n.len_squared();

        Self {
            origin,
            edge_a,
            edge_b,
            material,
            bounding_box,
            normal,
            plane_offset,
            scale_vector,
        }
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitResult> {
        let denominator = ray.direction().dot(*self.normal);

        if denominator.abs() < f32::EPSILON {
            return None;
        }

        let ray_origin_vector = Direction::new(ray.origin().x, ray.origin().y, ray.origin().z);
        let t = (self.plane_offset - ray_origin_vector.dot(*self.normal)) / denominator;

        if !interval.contains(t) {
            return None;
        }

        let intersection = ray.at(t);
        let hit_offset = intersection - self.origin;
        let alpha = self.scale_vector.dot(hit_offset.cross(self.edge_b));
        let beta = self.scale_vector.dot(self.edge_a.cross(hit_offset));

        if alpha < 0.0 || alpha > 1.0 || beta < 0.0 || beta > 1.0 {
            return None;
        }

        Some(HitResult::new(
            ray,
            intersection,
            t,
            alpha,
            beta,
            self.normal,
            self.material.clone(),
        ))
    }

    fn bounding_box(&self) -> BoundingBox {
        self.bounding_box
    }
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use crate::{
        color::Color,
        direction::Direction,
        material::{Matte, ReflectionType},
        point::Point,
    };

    use super::*;

    static TEST_QUAD: LazyLock<Quad> = LazyLock::new(|| {
        Quad::new(
            Point::new(-2.0, -2.0, 0.0),
            Direction::new(4.0, 0.0, 0.0),
            Direction::new(0.0, 4.0, 0.0),
            Arc::new(Matte::new(
                Color::new(1.0, 1.0, 1.0),
                ReflectionType::Diffuse,
            )),
        )
    });

    #[test]
    fn quad_hit_has_intersection() {
        let ray = Ray::new(Point::new(0.0, 0.0, 5.0), Direction::new(0.0, 0.0, -1.0));
        let interval = Interval::new(0.0, f32::INFINITY);

        let hit_result = TEST_QUAD.hit(&ray, &interval);

        assert!(hit_result.is_some());
        let hit_result = hit_result.unwrap();
        assert_eq!(hit_result.t, 5.0);
        assert_eq!(hit_result.u, 0.5);
        assert_eq!(hit_result.v, 0.5);
    }

    #[test]
    fn quad_hit_misses_when_ray_misses_plane() {
        let ray = Ray::new(Point::new(0.0, 0.0, 5.0), Direction::new(10.0, 0.0, 0.0));
        let interval = Interval::new(0.0, f32::INFINITY);

        let hit_result = TEST_QUAD.hit(&ray, &interval);

        assert!(hit_result.is_none());
    }

    #[test]
    fn quad_hit_misses_when_ray_misses_quad() {
        let ray = Ray::new(Point::new(50.0, 50.0, 5.0), Direction::new(10.0, 0.0, -1.0));
        let interval = Interval::new(0.0, f32::INFINITY);

        let hit_result = TEST_QUAD.hit(&ray, &interval);

        assert!(hit_result.is_none());
    }
}
