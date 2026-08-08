use crate::{interval::Interval, point::Point, ray::Ray};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoundingBox {
    pub(crate) x: Interval,
    pub(crate) y: Interval,
    pub(crate) z: Interval,
}

impl BoundingBox {
    pub fn new(a: Point, b: Point) -> Self {
        let x = if a.x <= b.x {
            Interval::new(a.x, b.x)
        } else {
            Interval::new(b.x, a.x)
        };
        let y = if a.y <= b.y {
            Interval::new(a.y, b.y)
        } else {
            Interval::new(b.y, a.y)
        };
        let z = if a.z <= b.z {
            Interval::new(a.z, b.z)
        } else {
            Interval::new(b.z, a.z)
        };

        Self { x, y, z }
    }

    pub fn empty() -> Self {
        Self {
            x: Interval::new(f32::INFINITY, f32::NEG_INFINITY),
            y: Interval::new(f32::INFINITY, f32::NEG_INFINITY),
            z: Interval::new(f32::INFINITY, f32::NEG_INFINITY),
        }
    }

    pub fn new_from_bounding_boxes(a: &BoundingBox, b: &BoundingBox) -> Self {
        let x = Interval::new_from_intervals(&a.x, &b.x);
        let y = Interval::new_from_intervals(&a.y, &b.y);
        let z = Interval::new_from_intervals(&a.z, &b.z);
        Self { x, y, z }
    }

    pub fn intersects(&self, ray: &Ray, interval: &Interval) -> bool {
        let origin = ray.origin();
        let inverse_direction = ray.inverse_direction();
        let mut ray_interval = *interval;

        self.intersects_axis(&self.x, origin.x, inverse_direction.x, &mut ray_interval)
            && self.intersects_axis(&self.y, origin.y, inverse_direction.y, &mut ray_interval)
            && self.intersects_axis(&self.z, origin.z, inverse_direction.z, &mut ray_interval)
    }

    #[inline(always)]
    fn intersects_axis(
        &self,
        axis_interval: &Interval,
        origin: f32,
        inverse_direction: f32,
        ray_interval: &mut Interval,
    ) -> bool {
        let mut t0 = (axis_interval.min - origin) * inverse_direction;
        let mut t1 = (axis_interval.max - origin) * inverse_direction;

        if t0 > t1 {
            std::mem::swap(&mut t0, &mut t1);
        }
        if t0 > ray_interval.min {
            ray_interval.min = t0;
        }
        if t1 < ray_interval.max {
            ray_interval.max = t1;
        }

        ray_interval.max > ray_interval.min
    }
}

#[cfg(test)]
mod tests {
    use crate::direction::Direction;

    use super::*;

    fn unit_cube() -> BoundingBox {
        BoundingBox::new(Point::new(0.0, 0.0, 0.0), Point::new(1.0, 1.0, 1.0))
    }

    fn ray_from(origin: Point, direction: Direction) -> Ray {
        Ray::new(origin, direction)
    }

    fn open_interval() -> Interval {
        Interval::new(0.001, f32::INFINITY)
    }

    #[test]
    fn empty_box_empty_on_all_axis() {
        let bbox = BoundingBox::empty();

        assert!(bbox.x.max < bbox.x.min);
        assert!(bbox.y.max < bbox.y.min);
        assert!(bbox.z.max < bbox.z.min);
    }

    #[test]
    fn merge_with_empty_returns_other_box() {
        let box1 = BoundingBox::empty();
        let box2 = BoundingBox::new(Point::new(0.0, 0.0, 0.0), Point::new(1.0, 1.0, 1.0));

        assert_eq!(BoundingBox::new_from_bounding_boxes(&box1, &box2), box2);
    }

    #[test]
    fn new_accepts_corners_in_either_order() {
        let a = Point::new(0.0, 0.0, 0.0);
        let b = Point::new(1.0, 1.0, 1.0);

        let forward = BoundingBox::new(a, b);
        let reverse = BoundingBox::new(b, a);

        let test_ray = ray_from(Point::new(0.5, 0.5, -1.0), Direction::new(0.0, 0.0, 1.0));

        assert_eq!(
            forward.intersects(&test_ray, &open_interval()),
            reverse.intersects(&test_ray, &open_interval())
        );
        assert!(forward.intersects(&test_ray, &open_interval()));
    }

    #[test]
    fn intersects_ray_through_box_center() {
        let bbox = unit_cube();
        let ray = ray_from(Point::new(0.5, 0.5, -1.0), Direction::new(0.0, 0.0, 1.0));

        assert!(bbox.intersects(&ray, &open_interval()));
    }

    #[test]
    fn intersects_ray_misses_box_outside_slabs() {
        let bbox = unit_cube();
        let ray = ray_from(Point::new(5.0, 0.5, 0.5), Direction::new(1.0, 0.0, 0.0));

        assert!(!bbox.intersects(&ray, &open_interval()));
    }

    #[test]
    fn intersects_ray_segment_before_box() {
        let bbox = unit_cube();
        let ray = ray_from(Point::new(0.5, 0.5, -1.0), Direction::new(0.0, 0.0, 1.0));
        let interval = Interval::new(0.001, 0.5);

        assert!(!bbox.intersects(&ray, &interval));
    }

    #[test]
    fn intersects_ray_segment_after_box() {
        let bbox = unit_cube();
        let ray = ray_from(Point::new(0.5, 0.5, -1.0), Direction::new(0.0, 0.0, 1.0));
        let interval = Interval::new(3.0, 100.0);

        assert!(!bbox.intersects(&ray, &interval));
    }

    #[test]
    fn intersects_ray_segment_inside_box() {
        let bbox = unit_cube();
        let ray = ray_from(Point::new(0.5, 0.5, -1.0), Direction::new(0.0, 0.0, 1.0));
        let interval = Interval::new(1.5, 1.8);

        assert!(bbox.intersects(&ray, &interval));
    }

    #[test]
    fn intersects_ray_with_negative_direction() {
        let bbox = unit_cube();
        let ray = ray_from(Point::new(2.0, 0.5, 0.5), Direction::new(-1.0, 0.0, 0.0));

        assert!(bbox.intersects(&ray, &open_interval()));
    }

    #[test]
    fn intersects_ray_parallel_inside_axis_slab() {
        let bbox = unit_cube();
        let ray = ray_from(Point::new(0.5, 0.5, -1.0), Direction::new(0.0, 0.0, 1.0));

        assert!(bbox.intersects(&ray, &open_interval()));
    }

    #[test]
    fn intersects_ray_parallel_outside_axis_slab() {
        let bbox = unit_cube();
        let ray = ray_from(Point::new(-1.0, 0.5, 0.5), Direction::new(0.0, 0.0, 1.0));

        assert!(!bbox.intersects(&ray, &open_interval()));
    }
}
