pub mod checker_texture;
pub mod image_texture;
pub mod perlin_noise_texture;
pub mod solid_color_texture;

use crate::{color::Color, point::Point};

pub trait Texture: Send + Sync {
    fn value(&self, u: f32, v: f32, p: Point) -> Color;
}
