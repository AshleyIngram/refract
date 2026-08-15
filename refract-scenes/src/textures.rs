use std::sync::Arc;

use refract::{
    checker_texture::CheckerTexture,
    color::Color,
    material::{Matte, ReflectionType},
    point::Point,
    scene::{Scene, SceneBuilder},
    sphere::Sphere,
};

pub struct TexturesScene;

impl TexturesScene {
    pub fn build(reflection_type: ReflectionType) -> Scene {
        let mut scene_builder = SceneBuilder::new();

        let texture =
            CheckerTexture::from_colors(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9));
        let material = Arc::new(Matte::with_texture(Arc::new(texture), reflection_type));

        let sphere1 = Sphere::new_stationary(Point::new(0.0, -10.0, 0.0), 10.0, material.clone());
        let sphere2 = Sphere::new_stationary(Point::new(0.0, 10.0, 0.0), 10.0, material.clone());

        scene_builder.add_object(sphere1);
        scene_builder.add_object(sphere2);

        scene_builder.build()
    }
}
