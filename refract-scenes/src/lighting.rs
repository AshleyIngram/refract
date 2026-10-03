use std::sync::Arc;

use refract::camera::RenderSettings;
use refract::color::Color;
use refract::direction::Direction;
use refract::material::DiffuseLight;
use refract::quad::Quad;
use refract::surfaces::texture::perlin_noise_texture::PerlinNoiseTexture;
use refract::surfaces::texture::solid_color_texture::SolidColorTexture;
use refract::{
    material::{Matte, ReflectionType},
    point::Point,
    scene::{Scene, SceneBuilder},
    sphere::Sphere,
};

use crate::ScenePreset;

pub struct LightingScene;

impl ScenePreset for LightingScene {
    fn build(&self, reflection_type: ReflectionType) -> Scene {
        let mut scene_builder = SceneBuilder::new();

        let perlin_noise_texture = PerlinNoiseTexture::new(4.0);
        let perlin_noise_material = Arc::new(Matte::with_texture(
            Arc::new(perlin_noise_texture),
            reflection_type,
        ));

        let light_material = Arc::new(DiffuseLight::new(Arc::new(SolidColorTexture::new(
            Color::new(4.0, 4.0, 4.0),
        ))));

        scene_builder
            .add_object(Sphere::new_stationary(
                Point::new(0.0, -1000.0, 0.0),
                1000.0,
                perlin_noise_material.clone(),
            ))
            .add_object(Sphere::new_stationary(
                Point::new(0.0, 2.0, 0.0),
                2.0,
                perlin_noise_material.clone(),
            ))
            .add_object(Sphere::new_stationary(
                Point::new(0.0, 7.0, 0.0),
                2.0,
                light_material.clone(),
            ))
            .add_object(Quad::new(
                Point::new(3.0, 1.0, -2.0),
                Direction::new(2.0, 0.0, 0.0),
                Direction::new(0.0, 2.0, 0.0),
                light_material.clone(),
            ));

        scene_builder.build()
    }

    fn default_render_settings(&self) -> RenderSettings {
        RenderSettings {
            width: 400,
            samples_per_pixel: 100,
            camera_center: Point::new(26.0, 3.0, 6.0),
            look_at: Point::new(0.0, 2.0, 0.0),
            defocus_angle: 0.0,
            ..RenderSettings::default()
        }
    }
}
