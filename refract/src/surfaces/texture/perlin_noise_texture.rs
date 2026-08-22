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

        let shuffled_x = Self::shuffled_indices(&mut rng);
        let shuffled_y = Self::shuffled_indices(&mut rng);
        let shuffled_z = Self::shuffled_indices(&mut rng);

        Self {
            random_floats,
            shuffled_x,
            shuffled_y,
            shuffled_z,
        }
    }

    fn shuffled_indices(rng: &mut impl rand::Rng) -> [usize; POINT_COUNT] {
        let mut indices = array::from_fn(|i| i);
        indices.shuffle(rng);
        indices
    }

    const CORNERS: [(i32, i32, i32); 8] = [
        (0, 0, 0),
        (1, 0, 0),
        (0, 1, 0),
        (1, 1, 0),
        (0, 0, 1),
        (1, 0, 1),
        (0, 1, 1),
        (1, 1, 1),
    ];

    fn perlin_noise(&self, p: Point) -> f32 {
        let u = p.x - p.x.floor();
        let v = p.y - p.y.floor();
        let w = p.z - p.z.floor();

        let cell_x = p.x.floor() as i32;
        let cell_y = p.y.floor() as i32;
        let cell_z = p.z.floor() as i32;

        let corners =
            Self::CORNERS.map(|(dx, dy, dz)| self.hash(cell_x + dx, cell_y + dy, cell_z + dz));

        Self::trilinear(corners, u, v, w)
    }

    fn hash(&self, x: i32, y: i32, z: i32) -> f32 {
        let idx = self.shuffled_x[(x & 255) as usize]
            ^ self.shuffled_y[(y & 255) as usize]
            ^ self.shuffled_z[(z & 255) as usize];
        self.random_floats[idx]
    }

    fn lerp(a: f32, b: f32, t: f32) -> f32 {
        a + t * (b - a)
    }

    fn trilinear(corners: [f32; 8], u: f32, v: f32, w: f32) -> f32 {
        Self::lerp(
            Self::lerp(
                Self::lerp(corners[0], corners[1], u),
                Self::lerp(corners[2], corners[3], u),
                v,
            ),
            Self::lerp(
                Self::lerp(corners[4], corners[5], u),
                Self::lerp(corners[6], corners[7], u),
                v,
            ),
            w,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn identity_texture() -> PerlinNoiseTexture {
        PerlinNoiseTexture {
            random_floats: array::from_fn(|i| i as f32),
            shuffled_x: array::from_fn(|i| i),
            shuffled_y: array::from_fn(|i| i),
            shuffled_z: array::from_fn(|i| i),
        }
    }

    #[test]
    fn lerp_endpoints_and_midpoint() {
        assert_eq!(PerlinNoiseTexture::lerp(0.0, 10.0, 0.0), 0.0);
        assert_eq!(PerlinNoiseTexture::lerp(0.0, 10.0, 1.0), 10.0);
        assert_eq!(PerlinNoiseTexture::lerp(0.0, 10.0, 0.5), 5.0);
    }

    #[test]
    fn trilinear_returns_corner_values_at_lattice_points() {
        let corners = [10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0];

        assert_eq!(PerlinNoiseTexture::trilinear(corners, 0.0, 0.0, 0.0), 10.0);
        assert_eq!(PerlinNoiseTexture::trilinear(corners, 1.0, 0.0, 0.0), 20.0);
        assert_eq!(PerlinNoiseTexture::trilinear(corners, 0.0, 1.0, 0.0), 30.0);
        assert_eq!(PerlinNoiseTexture::trilinear(corners, 1.0, 1.0, 0.0), 40.0);
        assert_eq!(PerlinNoiseTexture::trilinear(corners, 0.0, 0.0, 1.0), 50.0);
        assert_eq!(PerlinNoiseTexture::trilinear(corners, 1.0, 0.0, 1.0), 60.0);
        assert_eq!(PerlinNoiseTexture::trilinear(corners, 0.0, 1.0, 1.0), 70.0);
        assert_eq!(PerlinNoiseTexture::trilinear(corners, 1.0, 1.0, 1.0), 80.0);
    }

    #[test]
    fn trilinear_at_cell_center_averages_corners() {
        let corners = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];

        assert_eq!(PerlinNoiseTexture::trilinear(corners, 0.5, 0.5, 0.5), 3.5);
    }

    #[test]
    fn hash_wraps_negative_coordinates() {
        let texture = identity_texture();

        assert_eq!(texture.hash(-1, 0, 0), texture.hash(255, 0, 0));
        assert_eq!(texture.hash(0, -1, 0), texture.hash(0, 255, 0));
        assert_eq!(texture.hash(0, 0, -1), texture.hash(0, 0, 255));
    }

    #[test]
    fn hash_uses_xor_of_wrapped_permutations() {
        let texture = identity_texture();

        assert_eq!(texture.hash(3, 5, 7), 1.0);
        assert_eq!(texture.hash(-1, 0, 0), 255.0);
    }

    #[test]
    fn perlin_noise_at_integer_point_matches_first_corner_hash() {
        let texture = identity_texture();

        assert_eq!(
            texture.perlin_noise(Point::new(3.0, 0.0, 0.0)),
            texture.hash(3, 0, 0)
        );
        assert_eq!(
            texture.perlin_noise(Point::new(-1.0, 0.0, 0.0)),
            texture.hash(-1, 0, 0)
        );
    }
}
