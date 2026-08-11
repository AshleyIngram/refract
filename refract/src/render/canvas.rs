use std::io::Write;

use crate::{color::Color, interval::Interval, pixel_buffer::PixelBuffer};

/// A thread-safe destination for finished pixels.
///
/// `Camera::render` calls `set_pixel` from rayon worker threads as each pixel
/// completes, so implementations must be cheap and non-blocking.
pub trait PixelSink: Sync {
    fn set_pixel(&self, x: u32, y: u32, color: Color);

    fn is_cancelled(&self) -> bool {
        false
    }
}

/// Writes a rendered buffer as a plain-text PPM (P3) image.
pub fn write_ppm(buffer: &PixelBuffer, writer: &mut impl Write) -> std::io::Result<()> {
    write!(writer, "P3\n{} {}\n255\n", buffer.width(), buffer.height())?;

    for pixel in buffer.snapshot_rgba().chunks_exact(4) {
        writeln!(writer, "{} {} {}", pixel[0], pixel[1], pixel[2])?;
    }

    Ok(())
}

/// Converts a linear color to gamma-corrected 8-bit RGB.
pub(crate) fn color_to_rgb8(color: Color) -> [u8; 3] {
    let intensity = Interval::new(0.0, 0.999);
    let gamma_corrected = linear_to_gamma(color);

    [
        (256.0 * intensity.clamp(gamma_corrected.r)) as u8,
        (256.0 * intensity.clamp(gamma_corrected.g)) as u8,
        (256.0 * intensity.clamp(gamma_corrected.b)) as u8,
    ]
}

fn linear_to_gamma(color: Color) -> Color {
    Color::new(
        if color.r > 0.0 {
            color.r.sqrt()
        } else {
            color.r
        },
        if color.g > 0.0 {
            color.g.sqrt()
        } else {
            color.g
        },
        if color.b > 0.0 {
            color.b.sqrt()
        } else {
            color.b
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_to_rgb8_quarter_intensity_gamma_corrects_to_half() {
        let color = Color::new(0.25, 0.25, 0.25);

        let rgb = color_to_rgb8(color);

        assert_eq!(rgb, [128, 128, 128]);
    }

    #[test]
    fn color_to_rgb8_above_one_clamps_to_255() {
        let color = Color::new(2.0, 2.0, 2.0);

        let rgb = color_to_rgb8(color);

        assert_eq!(rgb, [255, 255, 255]);
    }

    #[test]
    fn write_ppm_two_by_one_buffer_writes_header_and_pixels() {
        let buffer = PixelBuffer::new(2, 1);
        buffer.set_pixel(0, 0, Color::new(1.0, 1.0, 1.0));
        buffer.set_pixel(1, 0, Color::new(0.0, 0.0, 0.0));
        let mut output = Vec::new();

        write_ppm(&buffer, &mut output).unwrap();

        let text = String::from_utf8(output).unwrap();
        assert_eq!(text, "P3\n2 1\n255\n255 255 255\n0 0 0\n");
    }
}
