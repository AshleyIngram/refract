use std::io::Write;

#[derive(Debug, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub fn write_ppm(&self, writer: &mut impl Write) -> Result<(), std::io::Error> {
        write!(
            writer,
            "{} {} {}\n",
            (255.9 * self.r) as i32,
            (255.9 * self.g) as i32,
            (255.9 * self.b) as i32
        )
    }
}
