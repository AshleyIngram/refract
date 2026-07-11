use crate::direction::Direction;
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Point { x, y, z }
    }
}

impl Add<Direction> for Point {
    type Output = Point;

    fn add(self, other: Direction) -> Point {
        Point::new(self.x + other.x, self.y + other.y, self.z + other.z)
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

impl AddAssign<Direction> for Point {
    fn add_assign(&mut self, other: Direction) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl AddAssign<&Direction> for Point {
    fn add_assign(&mut self, other: &Direction) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
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

impl SubAssign<Direction> for Point {
    fn sub_assign(&mut self, other: Direction) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl SubAssign<&Direction> for Point {
    fn sub_assign(&mut self, other: &Direction) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_add_correct() {
        let point = Point::new(1.0, 1.0, 1.0);
        let direction = Direction::new(1.0, 2.0, 3.0);
        let expected_result = Point::new(2.0, 3.0, 4.0);

        assert_eq!(point + direction, expected_result);
        assert_eq!(point + &direction, expected_result);
        assert_eq!(&point + direction, expected_result);
        assert_eq!(&point + &direction, expected_result);
    }

    #[test]
    fn point_add_assign_correct() {
        let mut point = Point::new(1.0, 1.0, 1.0);
        let direction = Direction::new(1.0, 2.0, 3.0);
        let expected_result = Point::new(2.0, 3.0, 4.0);

        point += direction;
        assert_eq!(point, expected_result);

        point = Point::new(1.0, 1.0, 1.0);
        point += &direction;
        assert_eq!(point, expected_result);
    }

    #[test]
    fn point_sub_correct() {
        let point = Point::new(1.0, 1.0, 1.0);
        let direction = Direction::new(1.0, 2.0, 3.0);
        let expected_result = Point::new(0.0, -1.0, -2.0);

        assert_eq!(point - direction, expected_result);
        assert_eq!(point - &direction, expected_result);
        assert_eq!(&point - direction, expected_result);
        assert_eq!(&point - &direction, expected_result);
    }

    #[test]
    fn point_sub_assign_correct() {
        let mut point = Point::new(1.0, 1.0, 1.0);
        let direction = Direction::new(1.0, 2.0, 3.0);
        let expected_result = Point::new(0.0, -1.0, -2.0);

        point -= direction;
        assert_eq!(point, expected_result);

        point = Point::new(1.0, 1.0, 1.0);
        point -= &direction;
        assert_eq!(point, expected_result);
    }
}