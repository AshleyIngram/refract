use refract::camera::{Camera, RenderSettings};
use refract::canvas::PpmCanvas;
use refract::direction::Direction;
use refract::material::ReflectionType;
use refract::point::Point;
use refract::scene::demo_scene;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let width = 1200;
    let field_of_view = 20.0;
    let camera_center = Point::new(13.0, 2.0, 3.0);
    let look_at = Point::new(0.0, 0.0, 0.0);
    let camera_up_direction = Direction::new(0.0, 1.0, 0.0);
    let defocus_angle = 0.6;
    let focus_distance = 10.0;

    let camera = Camera::new(
        width,
        aspect_ratio,
        field_of_view,
        camera_center,
        look_at,
        camera_up_direction,
        defocus_angle,
        focus_distance,
        RenderSettings::default(),
    );
    let mut canvas = PpmCanvas::new(camera.width, camera.height);

    let scene = demo_scene(ReflectionType::Lambertian);
    camera.render(&scene, &mut canvas);
}
