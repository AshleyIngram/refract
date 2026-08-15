use std::sync::Arc;

use refract::camera::RenderSettings;
use refract::surfaces::texture::image_texture::ImageTexture;
use refract::{
    material::{Matte, ReflectionType},
    point::Point,
    scene::{Scene, SceneBuilder},
    sphere::Sphere,
};

use crate::ScenePreset;

pub struct TexturesScene;

impl ScenePreset for TexturesScene {
    fn build(&self, reflection_type: ReflectionType) -> Scene {
        let mut scene_builder = SceneBuilder::new();

        let texture = ImageTexture::new("assets/textures/earthmap.jpg");
        let material = Arc::new(Matte::with_texture(Arc::new(texture), reflection_type));
        let sphere = Sphere::new_stationary(Point::new(0.0, 0.0, 0.0), 2.0, material.clone());

        scene_builder.add_object(sphere);

        scene_builder.build()
    }

    fn default_render_settings(&self) -> RenderSettings {
        RenderSettings {
            width: 400,
            samples_per_pixel: 100,
            max_depth: 50,
            vertical_field_of_view: 20.0,
            camera_center: Point::new(0.0, 0.0, 12.0),
            look_at: Point::new(0.0, 0.0, 0.0),
            defocus_angle: 0.0,
            ..RenderSettings::default()
        }
    }
}
