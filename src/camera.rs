use crate::{
    canvas::Canvas, color::Color, direction::Direction, hittable::Hittable, interval::Interval,
    point::Point, ray::Ray, rng, scene::Scene,
};

pub struct Camera {
    pub width: i32,
    pub height: i32,
    camera_center: Point,
    pixel_delta_u: Direction,
    pixel_delta_v: Direction,
    origin: Point,
}

impl Camera {
    const MAX_DEPTH: i32 = 50;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const PIXEL_SAMPLES_SCALE: f32 = 1.0 / Self::SAMPLES_PER_PIXEL as f32;

    pub fn new(
        width: i32,
        aspect_ratio: f64,
        vertical_field_of_view: f32,
        camera_center: Point,
        look_at: Point,
        camera_up_direction: Direction,
    ) -> Self {
        let height = ((width as f64 / aspect_ratio) as i32).max(1);
        let focal_length = (look_at - camera_center).len();
        let theta = vertical_field_of_view.to_radians();
        let h = f32::tan(theta / 2.0);
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * (width as f64 / height as f64) as f32;

        let w = (camera_center - look_at).normalize();
        let u = camera_up_direction.cross(*w).normalize();
        let v = w.cross(*u).normalize();

        let viewport_u = viewport_width * *u;
        let viewport_v = viewport_height * *-v;

        let pixel_delta_u = viewport_u / (width as f32);
        let pixel_delta_v = viewport_v / (height as f32);

        let viewport_upper_left =
            camera_center - focal_length * *w - viewport_u / 2.0 - viewport_v / 2.0;
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

    pub fn ray_color(&self, ray: &Ray, depth: i32, scene: &Scene) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let interval = Interval::new(0.001, f32::INFINITY);
        let hit_result = scene.hit(ray, &interval);

        match hit_result {
            None => {
                let unit_direction = ray.direction.normalize();
                let a = 0.5 * (unit_direction.y + 1.0);
                (1.0 - a) * Color::new(1.0, 1.0, 1.0) + (a * Color::new(0.5, 0.7, 1.0))
            }
            Some(h) => {
                let scatter_result = h.material.scatter(ray, &h);
                match scatter_result {
                    Some(scatter_result) => {
                        scatter_result.attenuation
                            * self.ray_color(&scatter_result.scattered, depth - 1, scene)
                    }
                    None => Color::new(0.0, 0.0, 0.0),
                }
            }
        }
    }

    pub fn render(&self, scene: &Scene, canvas: &mut impl Canvas) {
        for i in 0..self.height {
            for j in 0..self.width {
                let mut color = Color::new(0.0, 0.0, 0.0);

                for _sample in 0..Self::SAMPLES_PER_PIXEL {
                    let ray = self.get_ray(j, i);
                    color += self.ray_color(&ray, Self::MAX_DEPTH, scene);
                }

                canvas
                    .set_pixel(j as u32, i as u32, color * Self::PIXEL_SAMPLES_SCALE)
                    .unwrap();
            }
        }
    }

    fn get_ray(&self, u: i32, v: i32) -> Ray {
        let u_offset = rng::random_range(-0.5..0.5);
        let v_offset = rng::random_range(-0.5..0.5);
        let pixel_sample = self.origin
            + ((u as f32 + u_offset) * self.pixel_delta_u)
            + ((v as f32 + v_offset) * self.pixel_delta_v);

        let ray_direction = pixel_sample - self.camera_center;

        Ray::new(self.camera_center, ray_direction)
    }
}
