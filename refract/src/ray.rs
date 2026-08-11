use super::{direction::Direction, point::Point};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    origin: Point,
    direction: Direction,
    inverse_direction: Direction,
    time: f32,
}

impl Ray {
    pub fn new(origin: Point, direction: Direction) -> Self {
        Self {
            origin,
            direction,
            inverse_direction: direction.inverse(),
            time: 0.0,
        }
    }

    pub fn new_at_time(origin: Point, direction: Direction, time: f32) -> Self {
        Self {
            origin,
            direction,
            inverse_direction: direction.inverse(),
            time,
        }
    }

    pub fn origin(&self) -> Point {
        self.origin
    }

    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn inverse_direction(&self) -> Direction {
        self.inverse_direction
    }

    pub fn time(&self) -> f32 {
        self.time
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

        assert_eq!(ray.time(), 0.0);
    }

    #[test]
    fn ray_new_stores_inverse_direction() {
        let direction = Direction::new(2.0, 4.0, 5.0);
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), direction);

        assert_eq!(ray.inverse_direction(), direction.inverse());
    }

    #[test]
    fn ray_new_at_time_stores_inverse_direction() {
        let direction = Direction::new(-1.0, 0.0, 2.0);
        let ray = Ray::new_at_time(Point::new(0.0, 0.0, 0.0), direction, 0.5);

        assert_eq!(ray.inverse_direction(), direction.inverse());
        assert_eq!(ray.time(), 0.5);
    }
}
