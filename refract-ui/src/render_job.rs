use std::sync::{Arc, OnceLock};
use std::thread;
use std::time::{Duration, Instant};

use refract::camera::{Camera, RenderSettings};
use refract::color::Color;
use refract::direction::Direction;
use refract::material::{Dielectric, Material, Matte, Metal, ReflectionType};
use refract::pixel_buffer::PixelBuffer;
use refract::point::Point;
use refract::rng::random_range;
use refract::scene::{Scene, SceneBuilder};
use refract::sphere::Sphere;

pub const ASPECT_RATIO: f64 = 16.0 / 9.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RenderConfig {
    pub width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub field_of_view: f32,
    pub look_from: Point,
    pub look_at: Point,
    pub defocus_angle: f32,
    pub focus_distance: f32,
    pub reflection_type: ReflectionType,
}

impl Default for RenderConfig {
    fn default() -> Self {
        let settings = RenderSettings::default();

        Self {
            width: 1200,
            samples_per_pixel: settings.samples_per_pixel,
            max_depth: settings.max_depth,
            field_of_view: 20.0,
            look_from: Point::new(13.0, 2.0, 3.0),
            look_at: Point::new(0.0, 0.0, 0.0),
            defocus_angle: 0.6,
            focus_distance: 10.0,
            reflection_type: ReflectionType::Lambertian,
        }
    }
}

impl RenderConfig {
    fn build_camera(&self) -> Camera {
        Camera::new(RenderSettings {
            width: self.width,
            aspect_ratio: ASPECT_RATIO,
            vertical_field_of_view: self.field_of_view,
            camera_center: self.look_from,
            look_at: self.look_at,
            camera_up_direction: Direction::new(0.0, 1.0, 0.0),
            defocus_angle: self.defocus_angle,
            focus_distance: self.focus_distance,
            samples_per_pixel: self.samples_per_pixel,
            max_depth: self.max_depth,
        })
    }
}

/// A render running on a background thread, writing into a shared
/// `PixelBuffer` that the UI thread snapshots without blocking the workers.
pub struct RenderJob {
    buffer: Arc<PixelBuffer>,
    started_at: Instant,
    final_duration: Arc<OnceLock<Duration>>,
}

impl RenderJob {
    pub fn spawn(config: RenderConfig) -> Self {
        let camera = config.build_camera();
        let buffer = Arc::new(PixelBuffer::new(camera.width as u32, camera.height as u32));
        let started_at = Instant::now();
        let final_duration = Arc::new(OnceLock::new());

        let worker_buffer = Arc::clone(&buffer);
        let worker_duration = Arc::clone(&final_duration);
        thread::spawn(move || {
            let scene = build_scene(config.reflection_type);
            camera.render(&scene, worker_buffer.as_ref());
            let _ = worker_duration.set(started_at.elapsed());
        });

        Self {
            buffer,
            started_at,
            final_duration,
        }
    }

    pub fn buffer(&self) -> &PixelBuffer {
        &self.buffer
    }

    pub fn progress(&self) -> f32 {
        self.buffer.progress()
    }

    pub fn is_complete(&self) -> bool {
        self.buffer.is_complete()
    }

    /// Time spent rendering: still ticking while in flight, frozen once done.
    pub fn elapsed(&self) -> Duration {
        self.final_duration
            .get()
            .copied()
            .unwrap_or_else(|| self.started_at.elapsed())
    }

    pub fn cancel(&self) {
        self.buffer.cancel();
    }
}

impl Drop for RenderJob {
    fn drop(&mut self) {
        // The detached worker notices the flag at the next pixel and exits.
        self.cancel();
    }
}

fn build_scene(reflection_type: ReflectionType) -> Scene {
    let mut scene_builder = SceneBuilder::new();

    let ground = Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Matte::new(Color::new(0.5, 0.5, 0.5), reflection_type)),
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

    scene_builder.build()
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
