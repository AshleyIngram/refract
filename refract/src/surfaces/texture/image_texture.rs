use std::path::Path;

use load_image::export::imgref::ImgVec;
use load_image::export::rgb::RGBA8;

use crate::{color::Color, point::Point, surfaces::texture::texture::Texture};

pub struct ImageTexture {
    bitmap: Option<ImgVec<RGBA8>>,
}

impl ImageTexture {
    pub fn new(texture_path: impl AsRef<Path>) -> Self {
        match load_image::load_path(texture_path) {
            Ok(image) => {
                let (bitmap, _meta) = image.into_rgba();
                Self {
                    bitmap: Some(bitmap),
                }
            }
            Err(error) => {
                log::error!("Error loading image: {error}");
                Self { bitmap: None }
            }
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f32, v: f32, _p: Point) -> Color {
        let Some(bitmap) = &self.bitmap else {
            return Color::new(0.0, 1.0, 1.0);
        };

        if bitmap.width() == 0 || bitmap.height() == 0 {
            return Color::new(0.0, 1.0, 1.0);
        }

        let clamped_u = u.clamp(0.0, 1.0);
        let flipped_v = 1.0 - v.clamp(0.0, 1.0);

        let i = (clamped_u * bitmap.width() as f32) as usize;
        let j = (flipped_v * bitmap.height() as f32) as usize;
        let i = i.min(bitmap.width() - 1);
        let j = j.min(bitmap.height() - 1);

        let pixel = bitmap[(i, j)];
        let color_scale = 1.0 / 255.0;

        Color::new(
            color_scale * pixel.r as f32,
            color_scale * pixel.g as f32,
            color_scale * pixel.b as f32,
        )
    }
}