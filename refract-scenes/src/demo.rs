use std::sync::Arc;

use refract::{
    checker_texture::CheckerTexture,
    color::Color,
    direction::Direction,
    material::{Dielectric, Material, Matte, Metal, ReflectionType},
    point::Point,
    rng::random_range,
    scene::{Scene, SceneBuilder},
    sphere::Sphere,
};

pub struct DemoScene;

impl DemoScene {
    pub fn build(reflection_type: ReflectionType) -> Scene {
        let mut scene_builder = SceneBuilder::new();

        let checker_texture =
            CheckerTexture::from_colors(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));
        let ground = Sphere::new_stationary(
            Point::new(0.0, -1000.0, 0.0),
            1000.0,
            Arc::new(Matte::with_texture(
                Arc::new(checker_texture),
                reflection_type,
            )),
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

    let random_number = random_range(0.0..1.0);
    let material: Arc<dyn Material> = match random_number {
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

    if random_number < 0.8 {
        let center_to = center + Direction::new(0.0, random_range(0.0..0.5), 0.0);
        Some(Sphere::new_moving(center, center_to, 0.2, material))
    } else {
        Some(Sphere::new_stationary(center, 0.2, material))
    }
}
