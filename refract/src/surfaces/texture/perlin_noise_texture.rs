use std::array;

use crate::{
    color::Color,
    direction::{Direction, UnitDirection},
    point::Point,
    surfaces::texture::texture::Texture,
};
use rand::seq::SliceRandom;

const POINT_COUNT: usize = 256;

pub struct PerlinNoiseTexture {
    random_directions: [UnitDirection; POINT_COUNT],
    shuffled_x: [usize; POINT_COUNT],
    shuffled_y: [usize; POINT_COUNT],
    shuffled_z: [usize; POINT_COUNT],
    scale: f32,
}

impl Texture for PerlinNoiseTexture {
    fn value(&self, _u: f32, _v: f32, p: Point) -> Color {
        Color::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + self.perlin_noise(p * self.scale))
    }
}

impl PerlinNoiseTexture {
    pub fn new(scale: f32) -> Self {
        let random_directions =
            array::from_fn(|_| Direction::random_within_range(-1.0, 1.0).normalize());

        let mut rng = rand::rng();

        let shuffled_x = Self::shuffled_indices(&mut rng);
        let shuffled_y = Self::shuffled_indices(&mut rng);
        let shuffled_z = Self::shuffled_indices(&mut rng);

        Self {
            random_directions,
            shuffled_x,
            shuffled_y,
            shuffled_z,
            scale,
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
        let fx = p.x - p.x.floor();
        let fy = p.y - p.y.floor();
        let fz = p.z - p.z.floor();

        let u = Self::smoothstep(fx);
        let v = Self::smoothstep(fy);
        let w = Self::smoothstep(fz);

        let cell_x = p.x.floor() as i32;
        let cell_y = p.y.floor() as i32;
        let cell_z = p.z.floor() as i32;

        let corners = Self::CORNERS.map(|(dx, dy, dz)| {
            let idx = self.hash_index(cell_x + dx, cell_y + dy, cell_z + dz);
            let offset = Direction::new(fx - dx as f32, fy - dy as f32, fz - dz as f32);
            self.random_directions[idx].dot(offset)
        });

        Self::interpolate_corners(corners, u, v, w)
    }

    fn smoothstep(x: f32) -> f32 {
        x * x * (3.0 - 2.0 * x)
    }

    fn hash_index(&self, x: i32, y: i32, z: i32) -> usize {
        let idx = self.shuffled_x[(x & 255) as usize]
            ^ self.shuffled_y[(y & 255) as usize]
            ^ self.shuffled_z[(z & 255) as usize];
        idx
    }

    fn lerp(a: f32, b: f32, t: f32) -> f32 {
        a + t * (b - a)
    }

    fn interpolate_corners(corners: [f32; 8], u: f32, v: f32, w: f32) -> f32 {
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
            random_directions: array::from_fn(|_| Direction::new(1.0, 0.0, 0.0).normalize()),
            shuffled_x: array::from_fn(|i| i),
            shuffled_y: array::from_fn(|i| i),
            shuffled_z: array::from_fn(|i| i),
            scale: 1.0,
        }
    }

    #[test]
    fn lerp_endpoints_and_midpoint() {
        assert_eq!(PerlinNoiseTexture::lerp(0.0, 10.0, 0.0), 0.0);
        assert_eq!(PerlinNoiseTexture::lerp(0.0, 10.0, 1.0), 10.0);
        assert_eq!(PerlinNoiseTexture::lerp(0.0, 10.0, 0.5), 5.0);
    }

    #[test]
    fn interpolate_corners_returns_corner_values_at_lattice_points() {
        let corners = [10.0, 20.0, 30.0, 40.0, 50.0, 60.0, 70.0, 80.0];

        assert_eq!(
            PerlinNoiseTexture::interpolate_corners(corners, 0.0, 0.0, 0.0),
            10.0
        );
        assert_eq!(
            PerlinNoiseTexture::interpolate_corners(corners, 1.0, 0.0, 0.0),
            20.0
        );
        assert_eq!(
            PerlinNoiseTexture::interpolate_corners(corners, 0.0, 1.0, 0.0),
            30.0
        );
        assert_eq!(
            PerlinNoiseTexture::interpolate_corners(corners, 1.0, 1.0, 0.0),
            40.0
        );
        assert_eq!(
            PerlinNoiseTexture::interpolate_corners(corners, 0.0, 0.0, 1.0),
            50.0
        );
        assert_eq!(
            PerlinNoiseTexture::interpolate_corners(corners, 1.0, 0.0, 1.0),
            60.0
        );
        assert_eq!(
            PerlinNoiseTexture::interpolate_corners(corners, 0.0, 1.0, 1.0),
            70.0
        );
        assert_eq!(
            PerlinNoiseTexture::interpolate_corners(corners, 1.0, 1.0, 1.0),
            80.0
        );
    }

    #[test]
    fn interpolate_corners_at_cell_center_averages_corners() {
        let corners = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];

        assert_eq!(
            PerlinNoiseTexture::interpolate_corners(corners, 0.5, 0.5, 0.5),
            3.5
        );
    }

    #[test]
    fn hash_wraps_negative_coordinates() {
        let texture = identity_texture();

        assert_eq!(texture.hash_index(-1, 0, 0), texture.hash_index(255, 0, 0));
        assert_eq!(texture.hash_index(0, -1, 0), texture.hash_index(0, 255, 0));
        assert_eq!(texture.hash_index(0, 0, -1), texture.hash_index(0, 0, 255));
    }

    #[test]
    fn hash_index_uses_xor_of_wrapped_permutations() {
        let texture = identity_texture();

        assert_eq!(texture.hash_index(3, 5, 7), 1);
        assert_eq!(texture.hash_index(-1, 0, 0), 255);
    }

    #[test]
    fn perlin_noise_at_integer_point_is_zero() {
        let texture = identity_texture();
        assert_eq!(texture.perlin_noise(Point::new(3.0, 0.0, 0.0)), 0.0);
        assert_eq!(texture.perlin_noise(Point::new(-1.0, 0.0, 0.0)), 0.0);
    }
}
