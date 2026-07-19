use std::{mem::take, sync::Arc};

use crate::{
    color::Color,
    hittable::{HitResult, Hittable},
    interval::Interval,
    material::{Dielectric, Material, Matte, Metal, ReflectionType},
    point::Point,
    ray::Ray,
    rng::random_range,
    sphere::Sphere,
};

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

    pub fn add_object(&mut self, object: impl Hittable + 'static) -> &mut Self {
        self.objects.push(Arc::new(object));
        self
    }

    pub fn build(&mut self) -> Scene {
        Scene {
            objects: take(&mut self.objects),
        }
    }
}

/// Builds the "Ray Tracing in One Weekend" book cover scene: a large ground
/// sphere, a grid of small randomized spheres, and three hero spheres.
pub fn demo_scene(reflection_type: ReflectionType) -> Scene {
    let mut scene_builder = SceneBuilder::new();

    let ground = Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Matte::new(Color::new(0.5, 0.5, 0.5), reflection_type)),
    );
    scene_builder.add_object(ground);

    for a in -11..11 {
        for b in -11..11 {
            scene_builder.add_object(make_small_sphere(a, b, reflection_type));
        }
    }

    scene_builder.add_object(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    scene_builder.add_object(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        Arc::new(Matte::new(Color::new(0.4, 0.2, 0.1), reflection_type)),
    ));
    scene_builder.add_object(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)),
    ));

    scene_builder.build()
}

fn make_small_sphere(a: i32, b: i32, reflection_type: ReflectionType) -> Sphere {
    let center = Point::new(
        a as f32 + 0.9 * random_range(0.0..1.0),
        0.2,
        b as f32 + 0.9 * random_range(0.0..1.0),
    );
    let material: Arc<dyn Material> = match random_range(0.0..1.0) {
        m if m < 0.8 => {
            let albedo = Color::random() * Color::random();
            Arc::new(Matte::new(albedo, reflection_type))
        }
        m if m < 0.95 => {
            let albedo = Color::random();
            let fuzz = random_range(0.0..0.5);
            Arc::new(Metal::new(albedo, fuzz))
        }
        _ => Arc::new(Dielectric::new(1.5)),
    };

    Sphere::new(center, 0.2, material)
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
