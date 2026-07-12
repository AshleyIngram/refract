use std::io::{Write, stdout};

use crate::{color::Color, interval::Interval};

pub trait Canvas {
    fn set_pixel(&mut self, x: u32, y: u32, color: Color) -> Result<(), std::io::Error>;
}

pub struct PpmCanvas {
    is_initialized: bool,
    width: i32,
    height: i32,
    writer: Box<dyn Write>,
}

impl PpmCanvas {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            is_initialized: false,
            writer: Box::new(stdout()),
        }
    }
}

impl Canvas for PpmCanvas {
    fn set_pixel(&mut self, _x: u32, _y: u32, color: Color) -> Result<(), std::io::Error> {
        if !self.is_initialized {
            write!(self.writer, "P3\n{} {}\n255\n", self.width, self.height)?;
            self.is_initialized = true;
        }

        let intensity = Interval::new(0.0, 0.999);
        write!(
            self.writer,
            "{} {} {}\n",
            (256.0 * intensity.clamp(color.r)) as i32,
            (256.0 * intensity.clamp(color.g)) as i32,
            (256.0 * intensity.clamp(color.b)) as i32
        )
    }
}
