use std::{mem::take, sync::Arc};

use crate::{
    bounding_box::BoundingBox,
    hittable::{HitResult, Hittable},
    interval::Interval,
    ray::Ray,
};

pub struct Scene {
    objects: Vec<Arc<dyn Hittable>>,
    bounding_box: BoundingBox,
}

pub struct SceneBuilder {
    objects: Vec<Arc<dyn Hittable>>,
    bounding_box: BoundingBox,
}

impl SceneBuilder {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bounding_box: BoundingBox::empty(),
        }
    }

    pub fn add_object(&mut self, object: impl Hittable + 'static) -> &mut Self {
        let object_bounding_box = object.bounding_box();
        self.objects.push(Arc::new(object));

        self.bounding_box =
            BoundingBox::new_from_bounding_boxes(&self.bounding_box, &object_bounding_box);
        self
    }

    pub fn build(&mut self) -> Scene {
        Scene {
            objects: take(&mut self.objects),
            bounding_box: self.bounding_box,
        }
    }
}

impl Hittable for Scene {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitResult> {
        let mut hit_record: Option<HitResult> = None;

        for object in &self.objects {
            let current_interval = Interval::new(
                interval.min,
                hit_record.as_ref().map(|h| h.t).unwrap_or(interval.max),
            );

            if let Some(new_hit) = object.hit(ray, &current_interval) {
                hit_record = Some(new_hit);
            }
        }

        hit_record
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
        point::Point,
        sphere::Sphere,
    };

    use super::*;

    #[test]
    fn scene_hit_empty_scene_none() {
        let scene = SceneBuilder::new().build();
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Direction::new(0.0, 0.0, -1.0));
        let interval = Interval::new(0.0, f32::INFINITY);

        let hit_result = scene.hit(&ray, &interval);

        assert!(hit_result.is_none());
    }

    #[test]
    fn scene_hit_multiple_returns_closest() {
        let near = Sphere::new_stationary(
            Point::new(0.0, 0.0, -1.0),
            0.5,
            Arc::new(Matte::new(
                Color::new(1.0, 1.0, 1.0),
                ReflectionType::Diffuse,
            )),
        );
        let far = Sphere::new_stationary(
            Point::new(0.0, 0.0, -3.0),
            0.5,
            Arc::new(Matte::new(
                Color::new(1.0, 1.0, 1.0),
                ReflectionType::Diffuse,
            )),
        );
        let scene = SceneBuilder::new().add_object(near).add_object(far).build();
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Direction::new(0.0, 0.0, -1.0));
        let interval = Interval::new(0.0, f32::INFINITY);

        let hit_result_option = scene.hit(&ray, &interval);

        assert!(hit_result_option.is_some());
        let hit_result = hit_result_option.unwrap();
        assert_eq!(hit_result.t, 0.5);
        assert_eq!(hit_result.point.z, -0.5);
    }

    #[test]
    fn scene_hit_two_spheres_misses_returns_none() {
        let near = Sphere::new_stationary(
            Point::new(0.0, 0.0, -1.0),
            0.5,
            Arc::new(Matte::new(
                Color::new(1.0, 1.0, 1.0),
                ReflectionType::Diffuse,
            )),
        );
        let far = Sphere::new_stationary(
            Point::new(0.0, 0.0, -3.0),
            0.5,
            Arc::new(Matte::new(
                Color::new(1.0, 1.0, 1.0),
                ReflectionType::Diffuse,
            )),
        );
        let scene = SceneBuilder::new().add_object(near).add_object(far).build();
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Direction::new(0.0, 0.0, 1.0));
        let interval = Interval::new(0.0, f32::INFINITY);

        let hit_result_option = scene.hit(&ray, &interval);

        assert!(hit_result_option.is_none());
    }

    #[test]
    fn scene_bounding_box_unions_object_boxes() {
        let near = Sphere::new_stationary(
            Point::new(0.0, 0.0, -1.0),
            0.5,
            Arc::new(Matte::new(
                Color::new(1.0, 1.0, 1.0),
                ReflectionType::Diffuse,
            )),
        );
        let far = Sphere::new_stationary(
            Point::new(0.0, 0.0, -3.0),
            0.5,
            Arc::new(Matte::new(
                Color::new(1.0, 1.0, 1.0),
                ReflectionType::Diffuse,
            )),
        );
        let scene = SceneBuilder::new().add_object(near).add_object(far).build();

        assert_eq!(scene.bounding_box(), BoundingBox::new(Point::new(-0.5, -0.5, -3.5), Point::new(0.5, 0.5, -0.5)));
    }
}
