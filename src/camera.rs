use std::io::stdout;

use crate::{
    color::Color, direction::Direction, hittable::Hittable, interval::Interval, point::Point,
    ray::Ray, scene::Scene,
};

pub struct Camera {
    width: i32,
    height: i32,
    camera_center: Point,
    pixel_delta_u: Direction,
    pixel_delta_v: Direction,
    origin: Point,
}

impl Camera {
    pub fn new(width: i32, aspect_ratio: f64) -> Self {
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
        let origin = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            width,
            height,
            camera_center,
            pixel_delta_u,
            pixel_delta_v,
            origin,
        }
    }

    pub fn ray_color(&self, ray: &Ray, scene: &Scene) -> Color {
        let interval = Interval::new(0.001, f32::INFINITY);
        let hit_result = scene.hit(ray, &interval);

        match hit_result {
            None => {
                let unit_direction = ray.direction.normalize();
                let a = 0.5 * (unit_direction.y + 1.0);
                (1.0 - a) * Color::new(1.0, 1.0, 1.0) + (a * Color::new(0.5, 0.7, 1.0))
            }
            Some(h) => 0.5 * Color::new(h.normal.x + 1.0, h.normal.y + 1.0, h.normal.z + 1.0),
        }
    }

    pub fn render(&self, scene: &Scene) {
        println!("P3\n{} {}\n255", self.width, self.height);

        for i in 0..self.height {
            for j in 0..self.width {
                let pixel_center =
                    self.origin + (j as f32 * self.pixel_delta_u) + (i as f32 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.camera_center;
                let ray = Ray::new(self.camera_center, ray_direction);

                let color = self.ray_color(&ray, &scene);
                color.write_ppm(&mut stdout()).unwrap();
            }
        }
    }
}
