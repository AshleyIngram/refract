use refract::camera::Camera;
use refract::point::Point;
use refract::scene::SceneBuilder;
use refract::sphere::Sphere;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let width = 400;

    let camera = Camera::new(width, aspect_ratio);

    let scene = SceneBuilder::new()
        .add_object(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5))
        .add_object(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0))
        .build();

    camera.render(&scene)
}
