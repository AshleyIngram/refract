use std::io::{Write, stdout};

use crate::color::Color;

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

        write!(
            self.writer,
            "{} {} {}\n",
            (255.9 * color.r) as i32,
            (255.9 * color.g) as i32,
            (255.9 * color.b) as i32
        )
    }
}
