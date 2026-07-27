use crate::direction::Direction;
use crate::point::Point;

#[derive(Debug, PartialEq)]
pub struct Ray {
    pub origin: Point,
    pub direction: Direction,
    pub time: f32,
}

impl Ray {
    pub fn new(origin: Point, direction: Direction) -> Self {
        Self {
            origin,
            direction,
            time: 0.0,
        }
    }

    pub fn new_at_time(origin: Point, direction: Direction, time: f32) -> Self {
        Self {
            origin,
            direction,
            time,
        }
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

    #[test]
    fn ray_new_defaults_time_to_zero() {
        let ray = Ray::new(Point::new(1.0, 2.0, 3.0), Direction::new(4.0, 5.0, 6.0));

        assert_eq!(ray.time, 0.0);
    }
}
