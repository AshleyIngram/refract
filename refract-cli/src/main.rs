use std::io::stdout;

use refract::camera::{Camera, RenderSettings};
use refract::canvas::write_ppm;
use refract::material::ReflectionType;
use refract::pixel_buffer::PixelBuffer;
use refract_scenes::Book1Scene;

fn main() {
    let camera = Camera::new(RenderSettings {
        samples_per_pixel: 10,
        ..RenderSettings::default()
    });
    let buffer = PixelBuffer::new(camera.width as u32, camera.height as u32);
    let scene = Book1Scene::build(ReflectionType::Lambertian);

    camera.render(&scene, &buffer);

    write_ppm(&buffer, &mut stdout()).expect("failed to write PPM to stdout");
}
