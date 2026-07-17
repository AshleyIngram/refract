use std::f32::consts::PI;
use std::sync::Arc;

use refract::camera::Camera;
use refract::canvas::PpmCanvas;
use refract::color::Color;
use refract::material::{Matte, ReflectionType};
use refract::point::Point;
use refract::scene::SceneBuilder;
use refract::sphere::Sphere;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let field_of_view = 90.0;

    let camera = Camera::new(width, aspect_ratio, field_of_view);
    let mut canvas = PpmCanvas::new(camera.width, camera.height);

    let r = (PI / 4.0).cos();

    let scene = SceneBuilder::new()
        .add_object(Sphere::new(
            Point::new(-r, 0.0, -1.0),
            r,
            Arc::new(Matte::new(
                Color::new(0.0, 0.0, 1.0),
                ReflectionType::Lambertian,
            )),
        ))
        .add_object(Sphere::new(
            Point::new(r, 0.0, -1.0),
            r,
            Arc::new(Matte::new(
                Color::new(1.0, 0.0, 0.0),
                ReflectionType::Lambertian,
            )),
        ))
        .build();

    camera.render(&scene, &mut canvas);
}
