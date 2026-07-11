use std::io::stdout;

use crate::color::Color;
use crate::direction::Direction;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::point::Point;
use crate::ray::Ray;
use crate::sphere::Sphere;

pub mod color;
pub mod direction;
pub mod hittable;
pub mod interval;
pub mod point;
pub mod ray;
pub mod scene;
pub mod sphere;

fn ray_color(ray: &Ray, sphere: &Sphere) -> Color {
    let interval = Interval::new(0.001, f32::INFINITY);
    let hit_result = sphere.hit(ray, &interval);

    match hit_result {
        None => {
            let unit_direction = ray.direction.normalize();
            let a = 0.5 * (unit_direction.y + 1.0);
            (1.0 - a) * Color::new(1.0, 1.0, 1.0) + (a * Color::new(0.5, 0.7, 1.0))
        }
        Some(h) => 0.5 * Color::new(h.normal.x + 1.0, h.normal.y + 1.0, h.normal.z + 1.0),
    }
}

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let width = 400;
    let height = ((width as f64 / aspect_ratio) as i32).max(1);

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (width as f64 / height as f64) as f32;
    let camera_center = Point::new(0.0, 0.0, 0.0);

    let viewport_u = Direction::new(viewport_width, 0.0, 0.0);
    let viewport_v = Direction::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / (width as f32);
    let pixel_delta_v = viewport_v / (height as f32);

    let viewport_upper_left = camera_center
        - Direction::new(0.0, 0.0, focal_length)
        - viewport_u / 2.0
        - viewport_v / 2.0;
    let pixel00_location = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let sphere = Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5);

    println!("P3\n{} {}\n255", width, height);

    for i in 0..height {
        for j in 0..width {
            let pixel_center =
                pixel00_location + (j as f32 * pixel_delta_u) + (i as f32 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            let color = ray_color(&ray, &sphere);
            color.write_ppm(&mut stdout()).unwrap();
        }
    }
}
