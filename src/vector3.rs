use std::marker::PhantomData;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PointMarker;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct DirectionMarker;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3InternalMarker;

#[derive(Debug, PartialEq)]
pub struct Vector3<T> {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    _marker: PhantomData<fn(T)>,
}

pub type Point = Vector3<PointMarker>;
pub type Direction = Vector3<DirectionMarker>;

#[allow(dead_code)] // Used in tests
type Vec3 = Vector3<Vector3InternalMarker>;

impl<T> Vector3<T> {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z,
            _marker: PhantomData,
        }
    }

    pub fn len_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn len(&self) -> f32 {
        self.len_squared().sqrt()
    }

    pub fn dot(&self, other: &Vector3<T>) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vector3<T>) -> Vector3<T> {
        Self::new(
            (self.y * other.z) - (self.z * other.y),
            (self.z * other.x) - (self.x * other.z),
            (self.x * other.y) - (self.y * other.x),
        )
    }

    pub fn normalize(&self) -> Vector3<T> {
        *self / self.len()
    }
}

impl Add<Direction> for Point {
    type Output = Point;

    fn add(self, other: Direction) -> Point {
        Point::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Add<Direction> for Direction {
    type Output = Direction;

    fn add(self, other: Direction) -> Direction {
        Direction::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl AddAssign<Direction> for Point {
    fn add_assign(&mut self, other: Direction) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl AddAssign<Direction> for Direction {
    fn add_assign(&mut self, other: Direction) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<T> Div<f32> for Vector3<T> {
    type Output = Self;

    fn div(self, other: f32) -> Self {
        assert!(other != 0.0, "Division by zero");
        assert!(!f32::is_nan(other), "Division by NaN");

        Self::new(self.x / other, self.y / other, self.z / other)
    }
}

impl<T> DivAssign<f32> for Vector3<T> {
    fn div_assign(&mut self, other: f32) {
        assert!(other != 0.0, "Division by zero");
        assert!(!f32::is_nan(other), "Division by NaN");

        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

impl<T> Mul for Vector3<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self::new(self.x * other.x, self.y * other.y, self.z * other.z)
    }
}

impl<T> MulAssign for Vector3<T> {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl<T> Mul<f32> for Vector3<T> {
    type Output = Self;

    fn mul(self, other: f32) -> Self {
        Self::new(self.x * other, self.y * other, self.z * other)
    }
}

impl<T> MulAssign<f32> for Vector3<T> {
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl<T> Mul<Vector3<T>> for f32 {
    type Output = Vector3<T>;

    fn mul(self, other: Vector3<T>) -> Vector3<T> {
        other * self
    }
}

impl<T> Neg for Vector3<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl Sub<Direction> for Point {
    type Output = Point;

    fn sub(self, other: Direction) -> Point {
        Point::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Sub<Point> for Point {
    type Output = Direction;

    fn sub(self, other: Point) -> Direction {
        Direction::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl SubAssign<Direction> for Point {
    fn sub_assign(&mut self, other: Direction) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl<'a, 'b> Add<&'b Direction> for &'a Point {
    type Output = Point;

    fn add(self, other: &'b Direction) -> Point {
        *self + *other
    }
}

impl<'a> Add<Direction> for &'a Point {
    type Output = Point;

    fn add(self, other: Direction) -> Point {
        *self + other
    }
}

impl<'a> Add<&'a Direction> for Point {
    type Output = Point;

    fn add(self, other: &'a Direction) -> Point {
        self + *other
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

impl AddAssign<&Direction> for Point {
    fn add_assign(&mut self, other: &Direction) {
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

impl<'a, 'b> Sub<&'b Direction> for &'a Point {
    type Output = Point;

    fn sub(self, other: &'b Direction) -> Point {
        *self - *other
    }
}

impl<'a> Sub<Direction> for &'a Point {
    type Output = Point;

    fn sub(self, other: Direction) -> Point {
        *self - other
    }
}

impl<'a> Sub<&'a Direction> for Point {
    type Output = Point;

    fn sub(self, other: &'a Direction) -> Point {
        self - *other
    }
}

impl<'a, 'b> Sub<&'b Point> for &'a Point {
    type Output = Direction;

    fn sub(self, other: &'b Point) -> Direction {
        *self - *other
    }
}

impl<'a> Sub<Point> for &'a Point {
    type Output = Direction;

    fn sub(self, other: Point) -> Direction {
        *self - other
    }
}

impl<'a> Sub<&'a Point> for Point {
    type Output = Direction;

    fn sub(self, other: &'a Point) -> Direction {
        self - *other
    }
}

impl SubAssign<&Direction> for Point {
    fn sub_assign(&mut self, other: &Direction) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl<'a, 'b, T> Mul<&'b Vector3<T>> for &'a Vector3<T> {
    type Output = Vector3<T>;

    fn mul(self, other: &'b Vector3<T>) -> Vector3<T> {
        *self * *other
    }
}

impl<'a, T> Mul<Vector3<T>> for &'a Vector3<T> {
    type Output = Vector3<T>;

    fn mul(self, other: Vector3<T>) -> Vector3<T> {
        *self * other
    }
}

impl<'a, T> Mul<&'a Vector3<T>> for Vector3<T> {
    type Output = Vector3<T>;

    fn mul(self, other: &'a Vector3<T>) -> Vector3<T> {
        self * *other
    }
}

impl<T> MulAssign<&Vector3<T>> for Vector3<T> {
    fn mul_assign(&mut self, other: &Vector3<T>) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl<'a, T> Div<f32> for &'a Vector3<T> {
    type Output = Vector3<T>;

    fn div(self, other: f32) -> Vector3<T> {
        *self / other
    }
}

impl<'a, T> Mul<f32> for &'a Vector3<T> {
    type Output = Vector3<T>;

    fn mul(self, other: f32) -> Vector3<T> {
        *self * other
    }
}

impl<T> Mul<&Vector3<T>> for f32 {
    type Output = Vector3<T>;

    fn mul(self, other: &Vector3<T>) -> Vector3<T> {
        self * *other
    }
}

impl<'a, T> Neg for &'a Vector3<T> {
    type Output = Vector3<T>;

    fn neg(self) -> Vector3<T> {
        -*self
    }
}

impl<T> Clone for Vector3<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for Vector3<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector3_add_correct() {
        let vec1 = Point::new(1.0, 1.0, 1.0);
        let vec2 = Direction::new(1.0, 2.0, 3.0);
        let expected_result = Point::new(2.0, 3.0, 4.0);

        assert_eq!(vec1 + vec2, expected_result);
        assert_eq!(vec1 + &vec2, expected_result);
        assert_eq!(&vec1 + vec2, expected_result);
        assert_eq!(&vec1 + &vec2, expected_result);
    }

    #[test]
    fn vector3_add_assign_correct() {
        let mut vec1 = Point::new(1.0, 1.0, 1.0);
        let vec2 = Direction::new(1.0, 2.0, 3.0);
        let expected_result = Point::new(2.0, 3.0, 4.0);

        vec1 += vec2;
        assert_eq!(vec1, expected_result);

        vec1 = Point::new(1.0, 1.0, 1.0);
        vec1 += &vec2;
        assert_eq!(vec1, expected_result);
    }

    #[test]
    fn vector3_div_correct() {
        let vec = Vec3::new(1.0, 2.0, 4.0);
        let expected_result = Vec3::new(0.5, 1.0, 2.0);

        assert_eq!(vec / 2.0, expected_result);
        assert_eq!(&vec / 2.0, expected_result);
    }

    #[test]
    #[should_panic(expected = "Division by zero")]
    fn vector3_div_by_zero_panics() {
        let vec = Vec3::new(1.0, 2.0, 4.0);

        let _result = vec / 0.0;
    }

    #[test]
    #[should_panic(expected = "Division by NaN")]
    fn vector3_div_by_nan_panics() {
        let vec = Vec3::new(1.0, 2.0, 4.0);

        let _result = vec / f32::NAN;
    }

    #[test]
    fn vector3_div_assign_correct() {
        let mut vec = Vec3::new(1.0, 2.0, 4.0);
        let expected_result = Vec3::new(0.5, 1.0, 2.0);

        vec /= 2.0;

        assert_eq!(vec, expected_result);
    }

    #[test]
    #[should_panic(expected = "Division by zero")]
    fn vector3_div_assign_by_zero_panics() {
        let mut vec = Vec3::new(1.0, 2.0, 4.0);

        vec /= 0.0;
    }

    #[test]
    #[should_panic(expected = "Division by NaN")]
    fn vector3_div_assign_by_nan_panics() {
        let mut vec = Vec3::new(1.0, 2.0, 4.0);

        vec /= f32::NAN;
    }

    #[test]
    fn vector3_mul_vector3_correct() {
        let vec1 = Vec3::new(2.0, 2.0, 2.0);
        let vec2 = Vec3::new(7.0, 5.0, 3.0);
        let expected_result = Vec3::new(14.0, 10.0, 6.0);

        assert_eq!(vec1 * vec2, expected_result);
        assert_eq!(vec1 * &vec2, expected_result);
        assert_eq!(&vec1 * vec2, expected_result);
        assert_eq!(&vec1 * &vec2, expected_result);
    }

    #[test]
    fn vector3_mul_assign_vector3_correct() {
        let mut vec1 = Vec3::new(2.0, 2.0, 2.0);
        let vec2 = Vec3::new(7.0, 5.0, 3.0);
        let expected_result = Vec3::new(14.0, 10.0, 6.0);

        vec1 *= vec2;
        assert_eq!(vec1, expected_result);

        vec1 = Vec3::new(2.0, 2.0, 2.0);
        vec1 *= &vec2;
        assert_eq!(vec1, expected_result);
    }

    #[test]
    fn vector3_mul_f32_correct() {
        let vec = Vec3::new(7.0, 5.0, 3.0);
        let expected_result = Vec3::new(14.0, 10.0, 6.0);

        assert_eq!(vec * 2.0, expected_result);
        assert_eq!(&vec * 2.0, expected_result);
    }

    #[test]
    fn vector3_mul_assign_f32_correct() {
        let mut vec = Vec3::new(7.0, 5.0, 3.0);
        let expected_result = Vec3::new(14.0, 10.0, 6.0);

        vec *= 2.0;

        assert_eq!(vec, expected_result);
    }

    #[test]
    fn f32_mul_vector3_correct() {
        let vec = Vec3::new(7.0, 5.0, 3.0);
        let expected_result = Vec3::new(14.0, 10.0, 6.0);

        assert_eq!(2.0 * vec, expected_result);
        assert_eq!(2.0 * &vec, expected_result);
    }

    #[test]
    fn vector3_neg_correct() {
        let vec = Vec3::new(-1.0, 0.0, 1.0);
        let expected_result = Vec3::new(1.0, 0.0, -1.0);

        assert_eq!(-vec, expected_result);
        assert_eq!(-&vec, expected_result);
    }

    #[test]
    fn vector3_sub_correct() {
        let vec1 = Point::new(1.0, 1.0, 1.0);
        let vec2 = Direction::new(1.0, 2.0, 3.0);
        let expected_result = Point::new(0.0, -1.0, -2.0);

        assert_eq!(vec1 - vec2, expected_result);
        assert_eq!(vec1 - &vec2, expected_result);
        assert_eq!(&vec1 - vec2, expected_result);
        assert_eq!(&vec1 - &vec2, expected_result);
    }

    #[test]
    fn vector3_sub_assign_correct() {
        let mut vec1 = Point::new(1.0, 1.0, 1.0);
        let vec2 = Direction::new(1.0, 2.0, 3.0);
        let expected_result = Point::new(0.0, -1.0, -2.0);

        vec1 -= vec2;
        assert_eq!(vec1, expected_result);

        vec1 = Point::new(1.0, 1.0, 1.0);
        vec1 -= &vec2;
        assert_eq!(vec1, expected_result);
    }

    #[test]
    fn vector3_len_squared_positive_correct() {
        let vec = Vec3::new(3.0, 4.0, 12.0);
        let expected_result = 169.0;

        let result = vec.len_squared();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn vector3_len_squared_negative_correct() {
        let vec = Vec3::new(-2.0, -2.0, -1.0);
        let expected_result = 9.0;

        let result = vec.len_squared();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn vector3_len_positive_correct() {
        let vec = Vec3::new(3.0, 4.0, 12.0);
        let expected_result = 13.0;

        let result = vec.len();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn vector3_len_negative_correct() {
        let vec = Vec3::new(-2.0, -2.0, -1.0);
        let expected_result = 3.0;

        let result = vec.len();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn vector3_dot_perpendicular_zero() {
        let vec1 = Vec3::new(1.0, 0.0, 0.0);
        let vec2 = Vec3::new(0.0, 1.0, 0.0);
        let expected_result = 0.0;

        let result = vec1.dot(&vec2);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn vector3_dot_same_correct() {
        let vec1 = Vec3::new(1.0, 2.0, 3.0);
        let vec2 = Vec3::new(1.0, 2.0, 3.0);
        let expected_result = 14.0;

        let result = vec1.dot(&vec2);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn vector3_dot_opposite_direction_negative() {
        let vec1 = Vec3::new(1.0, 2.0, 3.0);
        let vec2 = Vec3::new(-1.0, -2.0, -3.0);
        let expected_result = -14.0;

        let result = vec1.dot(&vec2);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn vector3_cross_axis() {
        let vec1 = Vec3::new(1.0, 0.0, 0.0);
        let vec2 = Vec3::new(0.0, 1.0, 0.0);
        let expected_result = Vec3::new(0.0, 0.0, 1.0);

        let result = vec1.cross(&vec2);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn vector3_cross_reverse_negative() {
        let vec1 = Vec3::new(1.0, 0.0, 0.0);
        let vec2 = Vec3::new(0.0, 1.0, 0.0);
        let expected_result = Vec3::new(0.0, 0.0, -1.0);

        let result = vec2.cross(&vec1);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn vector3_cross_parallel_zero() {
        let vec1 = Vec3::new(1.0, 2.0, 3.0);
        let vec2 = Vec3::new(2.0, 4.0, 6.0);
        let expected_result = Vec3::new(0.0, 0.0, 0.0);

        let result = vec1.cross(&vec2);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn vector3_cross_correct() {
        let vec1 = Vec3::new(2.0, 3.0, 4.0);
        let vec2 = Vec3::new(5.0, 6.0, 7.0);
        let expected_result = Vec3::new(-3.0, 6.0, -3.0);

        let result = vec1.cross(&vec2);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn vector3_normalize_correct() {
        let vec = Vec3::new(3.0, 4.0, 0.0);
        let expected_result = Vec3::new(0.6, 0.8, 0.0);
        let expected_length = 1.0;

        let result = vec.normalize();

        assert_eq!(result, expected_result);
        assert_eq!(result.len(), expected_length);
    }

    #[test]
    #[should_panic(expected = "Division by zero")]
    fn vector3_normalize_zero_correct() {
        let vec = Vec3::new(0.0, 0.0, 0.0);

        vec.normalize();
    }
}
