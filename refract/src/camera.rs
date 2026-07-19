use crate::{
    canvas::PixelSink,
    color::Color,
    direction::Direction,
    hittable::Hittable,
    interval::Interval,
    point::Point,
    ray::Ray,
    rng::{self, random_range},
    scene::Scene,
};

use rayon::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct RenderSettings {
    pub width: i32,
    pub aspect_ratio: f64,
    pub vertical_field_of_view: f32,
    pub camera_center: Point,
    pub look_at: Point,
    pub camera_up_direction: Direction,
    pub defocus_angle: f32,
    pub focus_distance: f32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
}

impl Default for RenderSettings {
    fn default() -> Self {
        Self {
            width: 1200,
            aspect_ratio: 16.0 / 9.0,
            vertical_field_of_view: 20.0,
            camera_center: Point::new(13.0, 2.0, 3.0),
            look_at: Point::new(0.0, 0.0, 0.0),
            camera_up_direction: Direction::new(0.0, 1.0, 0.0),
            defocus_angle: 0.6,
            focus_distance: 10.0,
            samples_per_pixel: 500,
            max_depth: 50,
        }
    }
}

pub struct Camera {
    pub width: i32,
    pub height: i32,
    samples_per_pixel: i32,
    max_depth: i32,
    camera_center: Point,
    pixel_delta_u: Direction,
    pixel_delta_v: Direction,
    origin: Point,
    defocus_angle: f32,
    defocus_disk_u: Direction,
    defocus_disk_v: Direction,
}

impl Camera {
    pub fn new(settings: RenderSettings) -> Self {
        let width = settings.width;
        let height = ((width as f64 / settings.aspect_ratio) as i32).max(1);
        let theta = settings.vertical_field_of_view.to_radians();
        let h = f32::tan(theta / 2.0);
        let viewport_height = 2.0 * h * settings.focus_distance;
        let viewport_width = viewport_height * (width as f64 / height as f64) as f32;

        let w = (settings.camera_center - settings.look_at).normalize();
        let u = settings.camera_up_direction.cross(*w).normalize();
        let v = w.cross(*u).normalize();

        let viewport_u = viewport_width * *u;
        let viewport_v = viewport_height * *-v;

        let pixel_delta_u = viewport_u / (width as f32);
        let pixel_delta_v = viewport_v / (height as f32);

        let viewport_upper_left = settings.camera_center
            - settings.focus_distance * *w
            - viewport_u / 2.0
            - viewport_v / 2.0;
        let origin = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius =
            settings.focus_distance * f32::tan((settings.defocus_angle / 2.0).to_radians());
        let defocus_disk_u = defocus_radius * *u;
        let defocus_disk_v = defocus_radius * *v;

        Self {
            width,
            height,
            samples_per_pixel: settings.samples_per_pixel,
            max_depth: settings.max_depth,
            camera_center: settings.camera_center,
            pixel_delta_u,
            pixel_delta_v,
            origin,
            defocus_angle: settings.defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
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

    /// Renders the scene, delivering each pixel to the sink as it completes.
    ///
    /// Pixels are computed in parallel and arrive in arbitrary order. Returns
    /// early (leaving remaining pixels untouched) if the sink reports
    /// cancellation.
    pub fn render(&self, scene: &Scene, sink: &impl PixelSink) {
        let pixels = (0..self.height)
            .flat_map(|y| (0..self.width).map(move |x| (x, y)))
            .collect::<Vec<_>>();

        pixels.par_iter().for_each(|&(x, y)| {
            if sink.is_cancelled() {
                return;
            }

            sink.set_pixel(x as u32, y as u32, self.render_pixel(x, y, scene));
        });
    }

    fn render_pixel(&self, x: i32, y: i32, scene: &Scene) -> Color {
        let pixel_samples_scale = 1.0 / self.samples_per_pixel as f32;
        let mut color = Color::new(0.0, 0.0, 0.0);

        for _sample in 0..self.samples_per_pixel {
            let ray = self.get_ray(x, y);
            color += self.ray_color(&ray, self.max_depth, scene);
        }

        color * pixel_samples_scale
    }

    fn get_ray(&self, u: i32, v: i32) -> Ray {
        let u_offset = rng::random_range(-0.5..0.5);
        let v_offset = rng::random_range(-0.5..0.5);
        let pixel_sample = self.origin
            + ((u as f32 + u_offset) * self.pixel_delta_u)
            + ((v as f32 + v_offset) * self.pixel_delta_v);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.camera_center
        } else {
            self.random_point_in_unit_disk()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(ray_origin, ray_direction)
    }

    fn random_point_in_unit_disk(&self) -> Point {
        let p = loop {
            // Use direction for now, because it has a len_squared function.
            // This could be moved to an "offset" type, but that would require lots of code duplication,
            // and it's an implementation detail rather than part of the API.
            let d = Direction::new(random_range(-1.0..1.0), random_range(-1.0..1.0), 0.0);

            if d.len_squared() < 1.0 {
                break Point::new(d.x, d.y, d.z);
            }
        };

        self.camera_center + (p.x * self.defocus_disk_u) + (p.y * self.defocus_disk_v)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{
        material::{Matte, ReflectionType},
        pixel_buffer::PixelBuffer,
        scene::SceneBuilder,
        sphere::Sphere,
    };

    use super::*;

    fn tiny_camera() -> Camera {
        Camera::new(RenderSettings {
            width: 4,
            aspect_ratio: 1.0,
            vertical_field_of_view: 20.0,
            camera_center: Point::new(13.0, 2.0, 3.0),
            look_at: Point::new(0.0, 0.0, 0.0),
            camera_up_direction: Direction::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_distance: 10.0,
            samples_per_pixel: 500,
            max_depth: 50,
        })
    }

    fn test_scene() -> Scene {
        SceneBuilder::new()
            .add_object(Sphere::new(
                Point::new(0.0, 0.0, 0.0),
                1.0,
                Arc::new(Matte::new(
                    Color::new(0.5, 0.5, 0.5),
                    ReflectionType::Lambertian,
                )),
            ))
            .build()
    }

    #[test]
    fn render_uncancelled_sink_completes_every_pixel() {
        let camera = tiny_camera();
        let scene = test_scene();
        let buffer = PixelBuffer::new(camera.width as u32, camera.height as u32);

        camera.render(&scene, &buffer);

        assert!(buffer.is_complete());
    }

    #[test]
    fn render_cancelled_sink_completes_no_pixels() {
        let camera = tiny_camera();
        let scene = test_scene();
        let buffer = PixelBuffer::new(camera.width as u32, camera.height as u32);
        buffer.cancel();

        camera.render(&scene, &buffer);

        assert_eq!(buffer.progress(), 0.0);
    }

    #[test]
    fn render_settings_default_matches_previous_constants() {
        let settings = RenderSettings::default();

        assert_eq!(settings.samples_per_pixel, 500);
        assert_eq!(settings.max_depth, 50);
    }
}
