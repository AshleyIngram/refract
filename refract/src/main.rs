use std::sync::Arc;

use refract::camera::{Camera, RenderSettings};
use refract::canvas::PpmCanvas;
use refract::color::Color;
use refract::material::{Dielectric, Material, Matte, Metal, ReflectionType};
use refract::point::Point;
use refract::rng::random_range;
use refract::scene::SceneBuilder;
use refract::sphere::Sphere;

fn main() {
    let camera = Camera::new(RenderSettings::default());
    let mut canvas = PpmCanvas::new(camera.width, camera.height);

    let mut scene_builder = SceneBuilder::new();

    let ground = Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Matte::new(
            Color::new(0.5, 0.5, 0.5),
            ReflectionType::Lambertian,
        )),
    );

    scene_builder.add_object(ground);

    for a in -11..11 {
        for b in -11..11 {
            scene_builder.add_object(make_small_sphere(a, b));
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
        Arc::new(Matte::new(
            Color::new(0.4, 0.2, 0.1),
            ReflectionType::Lambertian,
        )),
    ));
    scene_builder.add_object(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)),
    ));

    let scene = scene_builder.build();
    camera.render(&scene, &mut canvas);
}

fn make_small_sphere(a: i32, b: i32) -> Sphere {
    let center = Point::new(
        a as f32 + 0.9 * random_range(0.0..1.0),
        0.2,
        b as f32 + 0.9 * random_range(0.0..1.0),
    );
    let material: Arc<dyn Material> = match random_range(0.0..1.0) {
        m if m < 0.8 => {
            let albedo = Color::random() * Color::random();
            Arc::new(Matte::new(albedo, ReflectionType::Lambertian))
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
