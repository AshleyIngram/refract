use std::sync::Arc;

use refract::camera::RenderSettings;
use refract::color::Color;
use refract::direction::Direction;
use refract::{
    material::{Matte, ReflectionType},
    point::Point,
    quad::Quad,
    scene::{Scene, SceneBuilder},
};

use crate::ScenePreset;

pub struct QuadsScene;

impl ScenePreset for QuadsScene {
    fn build(&self, reflection_type: ReflectionType) -> Scene {
        let mut scene_builder = SceneBuilder::new();

        let red_material = Arc::new(Matte::new(Color::new(1.0, 0.2, 0.2), reflection_type));
        let green_material = Arc::new(Matte::new(Color::new(0.2, 1.0, 0.2), reflection_type));
        let blue_material = Arc::new(Matte::new(Color::new(0.2, 0.2, 1.0), reflection_type));
        let orange_material = Arc::new(Matte::new(Color::new(1.0, 0.5, 0.0), reflection_type));
        let teal_material = Arc::new(Matte::new(Color::new(0.2, 0.8, 0.8), reflection_type));

        scene_builder
            .add_object(Quad::new(
                Point::new(-3.0, -2.0, 5.0),
                Direction::new(0.0, 0.0, -4.0),
                Direction::new(0.0, 4.0, 0.0),
                red_material,
            ))
            .add_object(Quad::new(
                Point::new(-2.0, -2.0, 0.0),
                Direction::new(4.0, 0.0, 0.0),
                Direction::new(0.0, 4.0, 0.0),
                green_material,
            ))
            .add_object(Quad::new(
                Point::new(3.0, -2.0, 1.0),
                Direction::new(0.0, 0.0, 4.0),
                Direction::new(0.0, 4.0, 0.0),
                blue_material,
            ))
            .add_object(Quad::new(
                Point::new(-2.0, 3.0, 1.0),
                Direction::new(4.0, 0.0, 0.0),
                Direction::new(0.0, 0.0, 4.0),
                orange_material,
            ))
            .add_object(Quad::new(
                Point::new(-2.0, -3.0, 5.0),
                Direction::new(4.0, 0.0, 0.0),
                Direction::new(0.0, 0.0, -4.0),
                teal_material,
            ));

        scene_builder.build()
    }

    fn default_render_settings(&self) -> RenderSettings {
        RenderSettings {
            width: 400,
            samples_per_pixel: 100,
            max_depth: 50,
            vertical_field_of_view: 80.0,
            camera_center: Point::new(0.0, 0.0, 9.0),
            look_at: Point::new(0.0, 0.0, 0.0),
            defocus_angle: 0.0,
            aspect_ratio: 1.0,
            background_color: Color::new(0.7, 0.8, 1.0),
            ..RenderSettings::default()
        }
    }
}
