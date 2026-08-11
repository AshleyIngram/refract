use crate::{color::Color, point::Point, surfaces::texture::texture::Texture};

pub struct SolidColorTexture {
    color: Color,
}

impl SolidColorTexture {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Texture for SolidColorTexture {
    fn value(&self, _u: f32, _v: f32, _p: Point) -> Color {
        self.color
    }
}
