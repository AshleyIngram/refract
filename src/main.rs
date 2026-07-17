use std::sync::Arc;

use refract::camera::Camera;
use refract::canvas::PpmCanvas;
use refract::color::Color;
use refract::direction::Direction;
use refract::material::{Dielectric, Matte, Metal, ReflectionType};
use refract::point::Point;
use refract::scene::SceneBuilder;
use refract::sphere::Sphere;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let field_of_view = 90.0;
    let camera_center = Point::new(-2.0, 2.0, 1.0);
    let look_at = Point::new(0.0, 0.0, -1.0);
    let camera_up_direction = Direction::new(0.0, 1.0, 0.0);

    let camera = Camera::new(
        width,
        aspect_ratio,
        field_of_view,
        camera_center,
        look_at,
        camera_up_direction,
    );
    let mut canvas = PpmCanvas::new(camera.width, camera.height);

    let scene = SceneBuilder::new()
        .add_object(Sphere::new(
            Point::new(0.0, 0.0, -1.2),
            0.5,
            Arc::new(Matte::new(
                Color::new(0.1, 0.2, 0.5),
                ReflectionType::Lambertian,
            )),
        ))
        .add_object(Sphere::new(
            Point::new(0.0, -100.5, -1.0),
            100.0,
            Arc::new(Matte::new(
                Color::new(0.8, 0.8, 0.0),
                ReflectionType::Lambertian,
            )),
        ))
        .add_object(Sphere::new(
            Point::new(-1.0, 0.0, -1.0),
            0.5,
            Arc::new(Dielectric::new(1.5)),
        ))
        .add_object(Sphere::new(
            Point::new(-1.0, 0.0, -1.0),
            0.4,
            Arc::new(Dielectric::new(1.0 / 1.5)),
        ))
        .add_object(Sphere::new(
            Point::new(1.0, 0.0, -1.0),
            0.5,
            Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0)),
        ))
        .build();

    camera.render(&scene, &mut canvas);
}
