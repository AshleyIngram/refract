use std::{cmp::Ordering, sync::Arc};

use crate::{
    bounding_box::BoundingBox,
    hittable::{HitResult, Hittable},
    interval::Interval,
    ray::Ray,
};

pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bounding_box: BoundingBox,
}

impl BvhNode {
    pub fn new(objects: &mut [Arc<dyn Hittable>]) -> Arc<Self> {
        Self::new_with_span(objects, 0, objects.len())
    }

    fn new_with_span(objects: &mut [Arc<dyn Hittable>], start: usize, end: usize) -> Arc<Self> {
        assert!(start < end);

        let span = end - start;

        match span {
            1 => Arc::new(Self {
                left: objects[start].clone(),
                right: objects[start].clone(),
                bounding_box: objects[start].bounding_box(),
            }),
            2 => Arc::new(Self {
                left: objects[start].clone(),
                right: objects[start + 1].clone(),
                bounding_box: BoundingBox::new_from_bounding_boxes(
                    &objects[start].bounding_box(),
                    &objects[start + 1].bounding_box(),
                ),
            }),
            _ => {
                let bbox = objects[start..end]
                    .iter()
                    .fold(BoundingBox::empty(), |acc, object| {
                        BoundingBox::new_from_bounding_boxes(&acc, &object.bounding_box())
                    });
                let axis = Self::longest_axis(&bbox);
                objects[start..end].sort_by(|a, b| Self::box_compare(a, b, axis));
                let mid = start + span / 2;

                let left = Self::new_with_span(objects, start, mid);
                let right = Self::new_with_span(objects, mid, end);
                let bounding_box = BoundingBox::new_from_bounding_boxes(
                    &left.bounding_box(),
                    &right.bounding_box(),
                );

                Arc::new(Self {
                    left: left,
                    right: right,
                    bounding_box,
                })
            }
        }
    }

    fn longest_axis(bbox: &BoundingBox) -> usize {
        [bbox.x.size(), bbox.y.size(), bbox.z.size()]
            .iter()
            .enumerate()
            .max_by(|(_k1, v1), (_k2, v2)| v1.partial_cmp(v2).unwrap())
            .unwrap()
            .0
    }

    fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis_index: usize) -> Ordering {
        let select_axis = |bbox: &BoundingBox| match axis_index {
            0 => bbox.x.min,
            1 => bbox.y.min,
            2 => bbox.z.min,
            _ => unreachable!(),
        };

        let a_axis = select_axis(&a.bounding_box());
        let b_axis = select_axis(&b.bounding_box());

        a_axis.partial_cmp(&b_axis).unwrap()
    }
}

impl Hittable for BvhNode {
    fn bounding_box(&self) -> BoundingBox {
        self.bounding_box
    }

    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitResult> {
        if !self.bounding_box.intersects(ray, interval) {
            None
        } else {
            let hit_left = self.left.hit(ray, interval);
            let hit_right = self.right.hit(
                ray,
                &Interval::new(
                    interval.min,
                    hit_left.as_ref().map(|h| h.t).unwrap_or(interval.max),
                ),
            );

            hit_right.or(hit_left)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        color::Color,
        direction::Direction,
        material::{Matte, ReflectionType},
        point::Point,
        sphere::Sphere,
    };

    use super::*;

    #[test]
    fn new_single_object_is_the_root() {
        let object: Arc<dyn Hittable> = sphere_at(0.0, 0.0, 0.0, 1.0);
        let sphere_bbox = object.bounding_box();

        let bvh_node = BvhNode::new(&mut [object.clone()]);

        assert_eq!(bvh_node.bounding_box(), sphere_bbox);
        assert!(Arc::ptr_eq(&bvh_node.left, &object));
        assert!(Arc::ptr_eq(&bvh_node.right, &object));
    }

    #[test]
    fn new_three_objects_correct_tree_construction() {
        let sphere1 = sphere_at(-4.0, 0.0, 0.0, 1.0);
        let sphere2 = sphere_at(0.0, 0.0, 0.0, 1.0);
        let sphere3 = sphere_at(4.0, 0.0, 0.0, 1.0);

        let bvh_node = BvhNode::new(&mut [sphere1.clone(), sphere2.clone(), sphere3.clone()]);

        assert_eq!(bvh_node.left.bounding_box(), sphere1.bounding_box());
        assert_eq!(
            bvh_node.right.bounding_box(),
            BoundingBox::new_from_bounding_boxes(&sphere2.bounding_box(), &sphere3.bounding_box())
        );
    }

    #[test]
    fn hit_returns_closest_object() {
        let near = sphere_at(0.0, 0.0, -1.0, 0.5);
        let far = sphere_at(0.0, 0.0, -3.0, 0.5);
        let bvh_node = BvhNode::new(&mut [near, far]);
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Direction::new(0.0, 0.0, -1.0));
        let interval = Interval::new(0.0, f32::INFINITY);

        let hit_result_option = bvh_node.hit(&ray, &interval);

        assert!(hit_result_option.is_some());
        let hit_result = hit_result_option.unwrap();
        assert_eq!(hit_result.t, 0.5);
        assert_eq!(hit_result.point.z, -0.5);
    }

    #[test]
    fn hit_misses_everything_none() {
        let near = sphere_at(0.0, 0.0, -1.0, 0.5);
        let far = sphere_at(0.0, 0.0, -3.0, 0.5);
        let bvh_node = BvhNode::new(&mut [near, far]);
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Direction::new(0.0, 0.0, 1.0));
        let interval = Interval::new(0.0, f32::INFINITY);

        let hit_result_option = bvh_node.hit(&ray, &interval);

        assert!(hit_result_option.is_none());
    }

    fn sphere_at(x: f32, y: f32, z: f32, radius: f32) -> Arc<dyn Hittable> {
        Arc::new(Sphere::new_stationary(
            Point::new(x, y, z),
            radius,
            Arc::new(Matte::new(
                Color::new(1.0, 1.0, 1.0),
                ReflectionType::Diffuse,
            )),
        ))
    }
}
