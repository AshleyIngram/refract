use std::{
    io::Write,
    ops::{Add, Mul},
};

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

impl Add<Color> for Color {
    type Output = Self;

    fn add(self, other: Color) -> Self {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
        }
    }
}

impl Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            r: self * other.r,
            g: self * other.g,
            b: self * other.b,
        }
    }
}
