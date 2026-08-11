use crate::{color::Color, point::Point};

pub trait Texture: Send + Sync {
    fn value(&self, u: f32, v: f32, p: Point) -> Color;
}
