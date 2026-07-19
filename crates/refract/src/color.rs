use std::ops::{Add, AddAssign, Mul};

use crate::rng::random_range;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub fn random() -> Self {
        Self {
            r: random_range(0.0..1.0),
            g: random_range(0.0..1.0),
            b: random_range(0.0..1.0),
        }
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

impl AddAssign<Color> for Color {
    fn add_assign(&mut self, other: Color) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
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

impl Mul<Color> for Color {
    type Output = Self;

    fn mul(self, other: Color) -> Self {
        Self {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_add_correct() {
        let color1 = Color::new(1.0, 2.0, 3.0);
        let color2 = Color::new(4.0, 5.0, 6.0);

        let result = color1 + color2;

        assert_eq!(result, Color::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn color_add_assign_correct() {
        let mut color = Color::new(1.0, 2.0, 3.0);
        let color2 = Color::new(4.0, 5.0, 6.0);

        color += color2;

        assert_eq!(color, Color::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn color_mul_f32_correct() {
        let color = Color::new(1.0, 2.0, 3.0);
        let multiplier = 2.0;
        let expected_result = Color::new(2.0, 4.0, 6.0);

        assert_eq!(multiplier * color, expected_result);
        assert_eq!(color * multiplier, expected_result);
    }

    #[test]
    fn color_mul_color_correct() {
        let color1 = Color::new(1.0, 2.0, 3.0);
        let color2 = Color::new(4.0, 5.0, 6.0);
        let expected_result = Color::new(4.0, 10.0, 18.0);

        assert_eq!(color1 * color2, expected_result);
    }
}
