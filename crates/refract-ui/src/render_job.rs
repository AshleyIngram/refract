use std::sync::{Arc, OnceLock};
use std::thread;
use std::time::{Duration, Instant};

use refract::camera::{Camera, RenderSettings};
use refract::canvas::PixelSink;
use refract::direction::Direction;
use refract::material::ReflectionType;
use refract::pixel_buffer::PixelBuffer;
use refract::point::Point;
use refract::scene::demo_scene;

pub const ASPECT_RATIO: f64 = 16.0 / 9.0;

/// Image height for a given width, matching how `Camera` derives it.
pub fn derived_height(width: i32) -> i32 {
    ((width as f64 / ASPECT_RATIO) as i32).max(1)
}

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
            samples_per_pixel: settings.samples_per_pixel(),
            max_depth: settings.max_depth(),
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
        Camera::new(
            self.width,
            ASPECT_RATIO,
            self.field_of_view,
            self.look_from,
            self.look_at,
            Direction::new(0.0, 1.0, 0.0),
            self.defocus_angle,
            self.focus_distance,
            RenderSettings::new(self.samples_per_pixel, self.max_depth),
        )
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
        let buffer = Arc::new(PixelBuffer::new(
            camera.width as u32,
            camera.height as u32,
        ));
        let started_at = Instant::now();
        let final_duration = Arc::new(OnceLock::new());

        let worker_buffer = Arc::clone(&buffer);
        let worker_duration = Arc::clone(&final_duration);
        thread::spawn(move || {
            let scene = demo_scene(config.reflection_type);
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

    pub fn is_cancelled(&self) -> bool {
        self.buffer.is_cancelled()
    }

    /// Whether the worker thread has exited, either by finishing every pixel
    /// or by observing cancellation.
    pub fn is_finished(&self) -> bool {
        self.final_duration.get().is_some()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn derived_height_standard_width_matches_aspect_ratio() {
        let height = derived_height(1200);

        assert_eq!(height, 675);
    }

    #[test]
    fn derived_height_tiny_width_clamps_to_one() {
        let height = derived_height(1);

        assert_eq!(height, 1);
    }
}
