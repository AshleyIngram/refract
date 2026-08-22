use std::sync::Arc;

use crate::{
    color::Color,
    point::Point,
    surfaces::texture::{solid_color_texture::SolidColorTexture, texture::Texture},
};

pub struct CheckerTexture {
    inverted_scale: f32,
    odd: Arc<dyn Texture>,
    even: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f32, odd: Arc<dyn Texture>, even: Arc<dyn Texture>) -> Self {
        Self {
            inverted_scale: 1.0 / scale,
            odd,
            even,
        }
    }

    pub fn from_colors(scale: f32, odd: Color, even: Color) -> Self {
        Self::new(
            scale,
            Arc::new(SolidColorTexture::new(odd)),
            Arc::new(SolidColorTexture::new(even)),
        )
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f32, v: f32, p: Point) -> Color {
        let scaled_point = p * self.inverted_scale;

        let is_even = f32::floor(scaled_point.x + scaled_point.y + scaled_point.z) as i32 % 2 == 0;

        if is_even {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}
