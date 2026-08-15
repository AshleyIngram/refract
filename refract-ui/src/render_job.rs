use std::sync::{Arc, OnceLock};
use std::thread;
use std::time::{Duration, Instant};

use refract::camera::{Camera, RenderSettings};
use refract::canvas::PixelSink;
use refract::direction::Direction;
use refract::material::ReflectionType;
use refract::pixel_buffer::PixelBuffer;
use refract::point::Point;

pub use refract_scenes::SceneKind;

pub const ASPECT_RATIO: f64 = 16.0 / 9.0;
pub fn derived_height(width: i32) -> i32 {
    ((width as f64 / ASPECT_RATIO) as i32).max(1)
}

/// UI render settings. Scene presets may define width and aspect ratio, but
/// [`build_camera`] always uses [`ASPECT_RATIO`] regardless of the preset's
/// `aspect_ratio` field.
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
    pub scene: SceneKind,
}

impl Default for RenderConfig {
    fn default() -> Self {
        let scene = SceneKind::default();
        let settings = scene.default_render_settings();

        Self {
            width: settings.width,
            samples_per_pixel: settings.samples_per_pixel,
            max_depth: settings.max_depth,
            field_of_view: settings.vertical_field_of_view,
            look_from: settings.camera_center,
            look_at: settings.look_at,
            defocus_angle: settings.defocus_angle,
            focus_distance: settings.focus_distance,
            reflection_type: ReflectionType::Lambertian,
            scene,
        }
    }
}

impl RenderConfig {
    fn apply_camera_from(&mut self, settings: &RenderSettings) {
        self.field_of_view = settings.vertical_field_of_view;
        self.look_from = settings.camera_center;
        self.look_at = settings.look_at;
        self.defocus_angle = settings.defocus_angle;
        self.focus_distance = settings.focus_distance;
    }

    pub fn apply_scene_camera_defaults(&mut self) {
        let settings = self.scene.default_render_settings();
        self.apply_camera_from(&settings);
    }

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
        let scene_kind = config.scene;
        let reflection_type = config.reflection_type;
        thread::spawn(move || {
            let scene = scene_kind.build(reflection_type);
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
