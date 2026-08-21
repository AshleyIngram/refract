use std::array;

use crate::{color::Color, point::Point, rng::random_range, surfaces::texture::texture::Texture};
use rand::seq::SliceRandom;

const POINT_COUNT: usize = 256;

pub struct PerlinNoiseTexture {
    random_floats: [f32; POINT_COUNT],
    shuffled_x: [usize; POINT_COUNT],
    shuffled_y: [usize; POINT_COUNT],
    shuffled_z: [usize; POINT_COUNT],
}

impl Texture for PerlinNoiseTexture {
    fn value(&self, _u: f32, _v: f32, p: Point) -> Color {
        Color::new(1.0, 1.0, 1.0) * self.perlin_noise(p)
    }
}

impl PerlinNoiseTexture {
    pub fn new() -> Self {
        let random_floats = array::from_fn(|_| random_range(0.0..1.0));

        let mut rng = rand::rng();

        let mut shuffled_x = array::from_fn(|i| i);
        shuffled_x.shuffle(&mut rng);
        let mut shuffled_y = array::from_fn(|i| i);
        shuffled_y.shuffle(&mut rng);
        let mut shuffled_z = array::from_fn(|i| i);
        shuffled_z.shuffle(&mut rng);

        Self {
            random_floats,
            shuffled_x,
            shuffled_y,
            shuffled_z,
        }
    }

    fn perlin_noise(&self, p: Point) -> f32 {
        let i = (4.0 * p.x) as i32 as usize & 255;
        let j = (4.0 * p.y) as i32 as usize & 255;
        let k = (4.0 * p.z) as i32 as usize & 255;

        self.random_floats[self.shuffled_x[i] ^ self.shuffled_y[j] ^ self.shuffled_z[k]]
    }
}
