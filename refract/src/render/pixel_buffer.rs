use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};

use crate::{
    canvas::{PixelSink, color_to_rgb8},
    color::Color,
};

/// A lock-free image buffer that rayon workers write into as pixels finish,
/// while another thread (e.g. a UI) concurrently snapshots it for display.
///
/// Pixels are stored as packed gamma-corrected RGBA using relaxed atomics, so
/// writers never block and readers see a consistent-enough view for preview.
pub struct PixelBuffer {
    width: u32,
    height: u32,
    pixels: Vec<AtomicU32>,
    completed: AtomicUsize,
    cancelled: AtomicBool,
}

impl PixelBuffer {
    pub fn new(width: u32, height: u32) -> Self {
        let pixel_count = (width * height) as usize;

        Self {
            width,
            height,
            pixels: (0..pixel_count).map(|_| AtomicU32::new(0)).collect(),
            completed: AtomicUsize::new(0),
            cancelled: AtomicBool::new(false),
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    /// Fraction of pixels completed so far, in `0.0..=1.0`.
    pub fn progress(&self) -> f32 {
        if self.pixels.is_empty() {
            return 1.0;
        }

        self.completed.load(Ordering::Relaxed) as f32 / self.pixels.len() as f32
    }

    pub fn is_complete(&self) -> bool {
        self.completed.load(Ordering::Relaxed) == self.pixels.len()
    }

    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Relaxed);
    }

    /// Copies the current pixels into an RGBA byte array (row-major order).
    /// Unfinished pixels are transparent black.
    pub fn snapshot_rgba(&self) -> Vec<u8> {
        self.pixels
            .iter()
            .flat_map(|pixel| pixel.load(Ordering::Relaxed).to_be_bytes())
            .collect()
    }
}

impl PixelSink for PixelBuffer {
    fn set_pixel(&self, x: u32, y: u32, color: Color) {
        let index = (y * self.width + x) as usize;
        let [r, g, b] = color_to_rgb8(color);
        let packed = u32::from_be_bytes([r, g, b, u8::MAX]);

        self.pixels[index].store(packed, Ordering::Relaxed);
        self.completed.fetch_add(1, Ordering::Relaxed);
    }

    fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_pixel_stores_gamma_corrected_rgba() {
        let buffer = PixelBuffer::new(2, 2);

        buffer.set_pixel(1, 0, Color::new(0.25, 1.0, 0.0));

        let rgba = buffer.snapshot_rgba();
        assert_eq!(&rgba[4..8], &[128, 255, 0, 255]);
    }

    #[test]
    fn snapshot_rgba_unfinished_pixels_are_transparent_black() {
        let buffer = PixelBuffer::new(1, 2);

        buffer.set_pixel(0, 1, Color::new(1.0, 1.0, 1.0));

        let rgba = buffer.snapshot_rgba();
        assert_eq!(&rgba[0..4], &[0, 0, 0, 0]);
    }

    #[test]
    fn progress_no_pixels_complete_returns_zero() {
        let buffer = PixelBuffer::new(2, 2);

        let progress = buffer.progress();

        assert_eq!(progress, 0.0);
    }

    #[test]
    fn progress_all_pixels_complete_returns_one() {
        let buffer = PixelBuffer::new(2, 1);

        buffer.set_pixel(0, 0, Color::new(0.0, 0.0, 0.0));
        buffer.set_pixel(1, 0, Color::new(0.0, 0.0, 0.0));

        assert_eq!(buffer.progress(), 1.0);
        assert!(buffer.is_complete());
    }

    #[test]
    fn cancel_marks_sink_as_cancelled() {
        let buffer = PixelBuffer::new(1, 1);

        buffer.cancel();

        assert!(buffer.is_cancelled());
    }
}
