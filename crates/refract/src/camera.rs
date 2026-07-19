use crate::{
    canvas::Canvas,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RenderSettings {
    samples_per_pixel: i32,
    max_depth: i32,
}

impl RenderSettings {
    pub fn new(samples_per_pixel: i32, max_depth: i32) -> Self {
        assert!(samples_per_pixel > 0, "samples_per_pixel must be positive");
        assert!(max_depth > 0, "max_depth must be positive");

        Self {
            samples_per_pixel,
            max_depth,
        }
    }

    pub fn samples_per_pixel(&self) -> i32 {
        self.samples_per_pixel
    }

    pub fn max_depth(&self) -> i32 {
        self.max_depth
    }

    fn pixel_samples_scale(&self) -> f32 {
        1.0 / self.samples_per_pixel as f32
    }
}

impl Default for RenderSettings {
    fn default() -> Self {
        Self::new(500, 50)
    }
}

pub struct Camera {
    pub width: i32,
    pub height: i32,
    settings: RenderSettings,
    camera_center: Point,
    pixel_delta_u: Direction,
    pixel_delta_v: Direction,
    origin: Point,
    defocus_angle: f32,
    defocus_disk_u: Direction,
    defocus_disk_v: Direction,
}

impl Camera {
    pub fn new(
        width: i32,
        aspect_ratio: f64,
        vertical_field_of_view: f32,
        camera_center: Point,
        look_at: Point,
        camera_up_direction: Direction,
        defocus_angle: f32,
        focus_distance: f32,
        settings: RenderSettings,
    ) -> Self {
        let height = ((width as f64 / aspect_ratio) as i32).max(1);
        let theta = vertical_field_of_view.to_radians();
        let h = f32::tan(theta / 2.0);
        let viewport_height = 2.0 * h * focus_distance;
        let viewport_width = viewport_height * (width as f64 / height as f64) as f32;

        let w = (camera_center - look_at).normalize();
        let u = camera_up_direction.cross(*w).normalize();
        let v = w.cross(*u).normalize();

        let viewport_u = viewport_width * *u;
        let viewport_v = viewport_height * *-v;

        let pixel_delta_u = viewport_u / (width as f32);
        let pixel_delta_v = viewport_v / (height as f32);

        let viewport_upper_left =
            camera_center - focus_distance * *w - viewport_u / 2.0 - viewport_v / 2.0;
        let origin = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius = focus_distance * f32::tan((defocus_angle / 2.0).to_radians());
        let defocus_disk_u = defocus_radius * *u;
        let defocus_disk_v = defocus_radius * *v;

        Self {
            width,
            height,
            settings,
            camera_center,
            pixel_delta_u,
            pixel_delta_v,
            origin,
            defocus_angle,
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

    pub fn render(&self, scene: &Scene, canvas: &mut impl Canvas) {
        let pixels = (0..self.height)
            .flat_map(|y| (0..self.width).map(move |x| (x, y)))
            .collect::<Vec<_>>();

        let colors = pixels.par_iter().map(|(x, y)| {
            let mut color = Color::new(0.0, 0.0, 0.0);

            for _sample in 0..self.settings.samples_per_pixel() {
                let ray = self.get_ray(*x, *y);
                color += self.ray_color(&ray, self.settings.max_depth(), scene);
            }

            color * self.settings.pixel_samples_scale()
        }).collect::<Vec<_>>();

        for i in 0..colors.len() {
            let y = i / self.width as usize;
            let x = i % self.width as usize;
            let _ = canvas.set_pixel(x as u32, y as u32, colors[i]);
        }
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
    use super::*;

    #[test]
    fn render_settings_new_zero_samples_panics() {
        let result = std::panic::catch_unwind(|| RenderSettings::new(0, 50));

        assert!(result.is_err());
    }

    #[test]
    fn render_settings_new_zero_depth_panics() {
        let result = std::panic::catch_unwind(|| RenderSettings::new(500, 0));

        assert!(result.is_err());
    }

    #[test]
    fn render_settings_pixel_samples_scale_is_reciprocal_of_samples() {
        let settings = RenderSettings::new(4, 10);

        let scale = settings.pixel_samples_scale();

        assert_eq!(scale, 0.25);
    }

    #[test]
    fn render_settings_default_matches_previous_constants() {
        let settings = RenderSettings::default();

        assert_eq!(settings.samples_per_pixel(), 500);
        assert_eq!(settings.max_depth(), 50);
    }
}
