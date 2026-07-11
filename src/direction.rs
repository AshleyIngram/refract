use std::ops::{Add, AddAssign, Deref, Div, DivAssign, Mul, MulAssign, Neg};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Direction {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct UnitDirection(Direction);

impl Direction {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn len_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn len(&self) -> f32 {
        self.len_squared().sqrt()
    }

    pub fn dot(&self, other: Direction) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Direction) -> Direction {
        Self::new(
            (self.y * other.z) - (self.z * other.y),
            (self.z * other.x) - (self.x * other.z),
            (self.x * other.y) - (self.y * other.x),
        )
    }

    pub fn normalize(&self) -> UnitDirection {
        UnitDirection(*self / self.len())
    }
}

impl UnitDirection {
    pub fn normalize(self) -> UnitDirection {
        self
    }
}

impl Deref for UnitDirection {
    type Target = Direction;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Add<Direction> for Direction {
    type Output = Direction;

    fn add(self, other: Direction) -> Direction {
        Direction::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl<'a, 'b> Add<&'b Direction> for &'a Direction {
    type Output = Direction;

    fn add(self, other: &'b Direction) -> Direction {
        *self + *other
    }
}

impl<'a> Add<Direction> for &'a Direction {
    type Output = Direction;

    fn add(self, other: Direction) -> Direction {
        *self + other
    }
}

impl<'a> Add<&'a Direction> for Direction {
    type Output = Direction;

    fn add(self, other: &'a Direction) -> Direction {
        self + *other
    }
}

impl AddAssign<Direction> for Direction {
    fn add_assign(&mut self, other: Direction) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl AddAssign<&Direction> for Direction {
    fn add_assign(&mut self, other: &Direction) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl Div<f32> for Direction {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        assert!(other != 0.0, "Division by zero");
        assert!(!f32::is_nan(other), "Division by NaN");

        Self::new(self.x / other, self.y / other, self.z / other)
    }
}

impl<'a> Div<f32> for &'a Direction {
    type Output = Direction;

    fn div(self, other: f32) -> Direction {
        *self / other
    }
}

impl DivAssign<f32> for Direction {
    fn div_assign(&mut self, other: f32) {
        assert!(other != 0.0, "Division by zero");
        assert!(!f32::is_nan(other), "Division by NaN");

        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

impl Mul<Direction> for f32 {
    type Output = Direction;

    fn mul(self, other: Direction) -> Direction {
        other * self
    }
}

impl Mul<&Direction> for f32 {
    type Output = Direction;

    fn mul(self, other: &Direction) -> Direction {
        self * *other
    }
}

impl Mul<f32> for Direction {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self::new(self.x * other, self.y * other, self.z * other)
    }
}

impl<'a> Mul<f32> for &'a Direction {
    type Output = Direction;

    fn mul(self, other: f32) -> Direction {
        *self * other
    }
}


impl MulAssign<f32> for Direction {
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl Neg for Direction {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl<'a> Neg for &'a Direction {
    type Output = Direction;

    fn neg(self) -> Direction {
        -*self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
    fn direction_div_correct() {
        let direction = Direction::new(1.0, 2.0, 4.0);
        let expected_result = Direction::new(0.5, 1.0, 2.0);

        assert_eq!(direction / 2.0, expected_result);
        assert_eq!(&direction / 2.0, expected_result);
    }
    
    #[test]
    #[should_panic(expected = "Division by zero")]
    fn direction_div_by_zero_panics() {
        let direction = Direction::new(1.0, 2.0, 4.0);

        let _result = direction / 0.0;
    }

    #[test]
    #[should_panic(expected = "Division by NaN")]
    fn direction_div_by_nan_panics() {
        let direction = Direction::new(1.0, 2.0, 4.0);

        let _result = direction / f32::NAN;
    }

    #[test]
    fn direction_div_assign_correct() {
        let mut direction = Direction::new(1.0, 2.0, 4.0);
        let expected_result = Direction::new(0.5, 1.0, 2.0);

        direction /= 2.0;

        assert_eq!(direction, expected_result);
    }

    #[test]
    #[should_panic(expected = "Division by zero")]
    fn direction_div_assign_by_zero_panics() {
        let mut direction = Direction::new(1.0, 2.0, 4.0);

        direction /= 0.0;
    }

    #[test]
    #[should_panic(expected = "Division by NaN")]
    fn direction_div_assign_by_nan_panics() {
        let mut direction = Direction::new(1.0, 2.0, 4.0);

        direction /= f32::NAN;
    }

    #[test]
    fn direction_mul_f32_correct() {
        let direction = Direction::new(7.0, 5.0, 3.0);
        let expected_result = Direction::new(14.0, 10.0, 6.0);

        assert_eq!(direction * 2.0, expected_result);
        assert_eq!(&direction * 2.0, expected_result);
    }

    #[test]
    fn direction_mul_assign_f32_correct() {
        let mut direction = Direction::new(7.0, 5.0, 3.0);
        let expected_result = Direction::new(14.0, 10.0, 6.0);

        direction *= 2.0;

        assert_eq!(direction, expected_result);
    }

    #[test]
    fn f32_mul_direction_correct() {
        let direction = Direction::new(7.0, 5.0, 3.0);
        let expected_result = Direction::new(14.0, 10.0, 6.0);

        assert_eq!(2.0 * direction, expected_result);
        assert_eq!(2.0 * &direction, expected_result);
    }

    #[test]
    fn direction_neg_correct() {
        let direction = Direction::new(-1.0, 0.0, 1.0);
        let expected_result = Direction::new(1.0, 0.0, -1.0);

        assert_eq!(-direction, expected_result);
        assert_eq!(-&direction, expected_result);
    }

    #[test]
    fn direction_len_squared_positive_correct() {
        let direction = Direction::new(3.0, 4.0, 12.0);
        let expected_result = 169.0;

        let result = direction.len_squared();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn direction_len_squared_negative_correct() {
        let direction = Direction::new(-2.0, -2.0, -1.0);
        let expected_result = 9.0;

        let result = direction.len_squared();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn direction_len_positive_correct() {
        let direction = Direction::new(3.0, 4.0, 12.0);
        let expected_result = 13.0;

        let result = direction.len();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn direction_len_negative_correct() {
        let direction = Direction::new(-2.0, -2.0, -1.0);
        let expected_result = 3.0;

        let result = direction.len();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn direction_dot_perpendicular_zero() {
        let direction1 = Direction::new(1.0, 0.0, 0.0);
        let direction2 = Direction::new(0.0, 1.0, 0.0);
        let expected_result = 0.0;

        let result = direction1.dot(direction2);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn direction_dot_same_correct() {
        let direction = Direction::new(1.0, 2.0, 3.0);
        let direction2 = Direction::new(1.0, 2.0, 3.0);
        let expected_result = 14.0;

        let result = direction.dot(direction2);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn direction_dot_opposite_direction_negative() {
        let direction1 = Direction::new(1.0, 2.0, 3.0);
        let direction2 = Direction::new(-1.0, -2.0, -3.0);
        let expected_result = -14.0;

        let result = direction1.dot(direction2);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn direction_cross_axis() {
        let direction1 = Direction::new(1.0, 0.0, 0.0);
        let direction2 = Direction::new(0.0, 1.0, 0.0);
        let expected_result = Direction::new(0.0, 0.0, 1.0);

        let result = direction1.cross(direction2);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn direction_cross_reverse_negative() {
        let direction1 = Direction::new(1.0, 0.0, 0.0);
        let direction2 = Direction::new(0.0, 1.0, 0.0);
        let expected_result = Direction::new(0.0, 0.0, -1.0);

        let result = direction2.cross(direction1);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn direction_cross_parallel_zero() {
        let direction1 = Direction::new(1.0, 2.0, 3.0);
        let direction2 = Direction::new(2.0, 4.0, 6.0);
        let expected_result = Direction::new(0.0, 0.0, 0.0);

        let result = direction1.cross(direction2);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn direction_cross_correct() {
        let direction1 = Direction::new(2.0, 3.0, 4.0);
        let direction2 = Direction::new(5.0, 6.0, 7.0);
        let expected_result = Direction::new(-3.0, 6.0, -3.0);

        let result = direction1.cross(direction2);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn direction_normalize_correct() {
        let direction = Direction::new(3.0, 4.0, 0.0);
        let expected_result = UnitDirection(Direction::new(0.6, 0.8, 0.0));
        let expected_length = 1.0;

        let result = direction.normalize();

        assert_eq!(result, expected_result);
        assert_eq!(result.len(), expected_length);
    }

    #[test]
    #[should_panic(expected = "Division by zero")]
    fn vector3_normalize_zero_correct() {
        let direction = Direction::new(0.0, 0.0, 0.0);

        direction.normalize();
    }
}