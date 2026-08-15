use std::sync::Arc;

use refract::{
    color::Color,
    material::{Dielectric, Material, Matte, Metal, ReflectionType},
    point::Point,
    rng::random_range,
    scene::{Scene, SceneBuilder},
    sphere::Sphere,
};

use crate::ScenePreset;

pub struct Book1Scene;

impl ScenePreset for Book1Scene {
    fn build(&self, reflection_type: ReflectionType) -> Scene {
        let mut scene_builder = SceneBuilder::new();

        let ground = Sphere::new_stationary(
            Point::new(0.0, -1000.0, 0.0),
            1000.0,
            Arc::new(Matte::new(Color::new(0.5, 0.5, 0.5), reflection_type)),
        );

        scene_builder.add_object(ground);

        for a in -11..11 {
            for b in -11..11 {
                if let Some(sphere) = make_small_sphere(a, b, reflection_type) {
                    scene_builder.add_object(sphere);
                }
            }
        }

        scene_builder.add_object(Sphere::new_stationary(
            Point::new(0.0, 1.0, 0.0),
            1.0,
            Arc::new(Dielectric::new(1.5)),
        ));
        scene_builder.add_object(Sphere::new_stationary(
            Point::new(-4.0, 1.0, 0.0),
            1.0,
            Arc::new(Matte::new(Color::new(0.4, 0.2, 0.1), reflection_type)),
        ));
        scene_builder.add_object(Sphere::new_stationary(
            Point::new(4.0, 1.0, 0.0),
            1.0,
            Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)),
        ));

        scene_builder.build()
    }
}

fn make_small_sphere(a: i32, b: i32, reflection_type: ReflectionType) -> Option<Sphere> {
    let center = Point::new(
        a as f32 + 0.9 * random_range(0.0..1.0),
        0.2,
        b as f32 + 0.9 * random_range(0.0..1.0),
    );

    if (center - Point::new(4.0, 0.2, 0.0)).len() <= 0.9 {
        return None;
    }

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

    Some(Sphere::new_stationary(center, 0.2, material))
}
