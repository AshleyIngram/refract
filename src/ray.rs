use crate::vector3::{Direction, Point};

#[derive(Debug, PartialEq)]
pub struct Ray {
    pub origin: Point,
    pub direction: Direction,
}

impl Ray {
    pub fn new(origin: Point, direction: Direction) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f32) -> Point {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_at_zero_origin() {
        let ray = Ray::new(Point::new(1.0, 2.0, 3.0), Direction::new(4.0, 5.0, 6.0));

        let result = ray.at(0.0);

        assert_eq!(result, Point::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn ray_at_t_correct() {
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Direction::new(1.0, 1.0, 1.0));

        let result = ray.at(5.0);

        assert_eq!(result, Point::new(5.0, 5.0, 5.0));
    }
}
