use rand::{RngExt, rngs::SmallRng};

use crate::{
    canvas::Canvas, color::Color, direction::Direction, hittable::Hittable, interval::Interval,
    point::Point, ray::Ray, scene::Scene,
};

pub struct Camera {
    pub width: i32,
    pub height: i32,
    camera_center: Point,
    pixel_delta_u: Direction,
    pixel_delta_v: Direction,
    origin: Point,
    samples_per_pixel: i32,
    pixel_samples_scale: f32,
    random_generator: SmallRng,
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

        let samples_per_pixel = 10;
        let pixel_samples_scale = 1.0 / samples_per_pixel as f32;
        let random_generator: SmallRng = rand::make_rng();

        Self {
            width,
            height,
            camera_center,
            pixel_delta_u,
            pixel_delta_v,
            origin,
            samples_per_pixel,
            pixel_samples_scale,
            random_generator,
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

    pub fn render(&mut self, scene: &Scene, canvas: &mut impl Canvas) {
        for i in 0..self.height {
            for j in 0..self.width {
                let mut color = Color::new(0.0, 0.0, 0.0);

                for _sample in 0..self.samples_per_pixel {
                    let ray = self.get_ray(j, i);
                    color += self.ray_color(&ray, &scene);
                }

                canvas
                    .set_pixel(j as u32, i as u32, color * self.pixel_samples_scale)
                    .unwrap();
            }
        }
    }

    fn get_ray(&mut self, u: i32, v: i32) -> Ray {
        let u_offset = self.random_generator.random_range(-0.5..0.5);
        let v_offset = self.random_generator.random_range(-0.5..0.5);
        let pixel_sample = self.origin
            + ((u as f32 + u_offset) * self.pixel_delta_u)
            + ((v as f32 + v_offset) * self.pixel_delta_v);

        let ray_direction = pixel_sample - self.camera_center;

        Ray::new(self.camera_center, ray_direction)
    }
}
