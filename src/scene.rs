use std::sync::Arc;

use crate::{hittable::HitResult, hittable::Hittable, interval::Interval, ray::Ray};

pub struct Scene {
    objects: Vec<Arc<dyn Hittable>>,
}

pub struct SceneBuilder {
    objects: Vec<Arc<dyn Hittable>>,
}

impl SceneBuilder {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add_object(mut self, object: impl Hittable + 'static) -> Self {
        self.objects.push(Arc::new(object));
        self
    }

    pub fn build(self) -> Scene {
        Scene {
            objects: self.objects,
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
        let near = Sphere::new(
            Point::new(0.0, 0.0, -1.0),
            0.5,
            Arc::new(Matte::new(
                Color::new(1.0, 1.0, 1.0),
                ReflectionType::Diffuse,
            )),
        );
        let far = Sphere::new(
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
        let near = Sphere::new(
            Point::new(0.0, 0.0, -1.0),
            0.5,
            Arc::new(Matte::new(
                Color::new(1.0, 1.0, 1.0),
                ReflectionType::Diffuse,
            )),
        );
        let far = Sphere::new(
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
}
