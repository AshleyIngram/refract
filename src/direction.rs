use std::ops::{Add, AddAssign, Deref, Div, DivAssign, Mul, MulAssign, Neg, Sub};

use crate::rng;

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

    pub fn random() -> Self {
        Self::new(rng::random(), rng::random(), rng::random())
    }

    pub fn random_within_range(from: f32, to: f32) -> Self {
        Self::new(
            rng::random_range(from..to),
            rng::random_range(from..to),
            rng::random_range(from..to),
        )
    }

    pub fn near_zero(&self) -> bool {
        self.x.abs() < f32::EPSILON && self.y.abs() < f32::EPSILON && self.z.abs() < f32::EPSILON
    }

    pub fn reflect(&self, normal: UnitDirection) -> Direction {
        *self - (2.0 * self.dot(*normal) * *normal)
    }

    pub fn refract(
        &self,
        normal: UnitDirection,
        from_refractive_index: f32,
        to_refractive_index: f32,
    ) -> Direction {
        let etai_over_etat = from_refractive_index / to_refractive_index;
        let cos_theta = f32::min(-self.dot(*normal), 1.0);
        let out_perpendicular = etai_over_etat * (*self + cos_theta * *normal);
        let out_parallel = -f32::sqrt(f32::abs(1.0 - out_perpendicular.len_squared())) * *normal;

        out_perpendicular + out_parallel
    }
}

impl UnitDirection {
    pub fn normalize(self) -> UnitDirection {
        self
    }

    pub fn random_unit_direction() -> UnitDirection {
        loop {
            let direction = Direction::random_within_range(-1.0, 1.0);
            let len_squared = direction.len_squared();
            if len_squared < f32::EPSILON || len_squared > 1.0 {
                continue;
            }

            return direction.normalize();
        }
    }

    pub fn random_hemisphere_direction(normal: UnitDirection) -> UnitDirection {
        Self::orient_to_hemisphere(Self::random_unit_direction(), normal)
    }

    fn orient_to_hemisphere(direction: UnitDirection, normal: UnitDirection) -> UnitDirection {
        if direction.dot(*normal) > 0.0 {
            direction
        } else {
            -direction
        }
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

impl Neg for UnitDirection {
    type Output = UnitDirection;

    fn neg(self) -> Self {
        UnitDirection(-*self)
    }
}

impl<'a> Neg for &'a UnitDirection {
    type Output = UnitDirection;

    fn neg(self) -> UnitDirection {
        -*self
    }
}

impl Sub<Direction> for Direction {
    type Output = Direction;

    fn sub(self, other: Direction) -> Direction {
        self + (-other)
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
    fn direction_normalize_zero_correct() {
        let direction = Direction::new(0.0, 0.0, 0.0);

        direction.normalize();
    }

    #[test]
    fn orient_to_hemisphere_keeps_outward_direction() {
        let normal = Direction::new(0.0, 1.0, 0.0).normalize();
        let outward = Direction::new(0.1, 1.0, 0.2).normalize();

        let result = UnitDirection::orient_to_hemisphere(outward, normal);

        assert_eq!(result, outward);
        assert!(result.dot(*normal) > 0.0);
    }

    #[test]
    fn orient_to_hemisphere_flips_inward_direction() {
        let normal = Direction::new(0.0, 1.0, 0.0).normalize();
        let inward = Direction::new(0.1, -1.0, 0.2).normalize();

        let result = UnitDirection::orient_to_hemisphere(inward, normal);

        assert_eq!(result, -inward);
        assert!(result.dot(*normal) > 0.0);
    }

    #[test]
    fn near_zero_correct() {
        let near_zero = Direction::new(f32::EPSILON / 2.0, f32::EPSILON / 2.0, f32::EPSILON / 2.0);
        let not_near_zero = Direction::new(1.0, 1.0, 1.0);

        assert!(near_zero.near_zero());
        assert!(!not_near_zero.near_zero());
    }

    #[test]
    fn reflect_off_floor_flips_vertical_component() {
        // Ray straight down, floor normal up
        let incident = Direction::new(0.0, -1.0, 0.0);
        let normal = Direction::new(0.0, 1.0, 0.0).normalize();

        let result = incident.reflect(normal);

        assert_eq!(result, Direction::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn reflect_off_vertical_wall_reverses_horizontal() {
        let incident = Direction::new(1.0, 0.0, 0.0);
        let normal = Direction::new(-1.0, 0.0, 0.0).normalize();

        let result = incident.reflect(normal);

        assert_eq!(result, Direction::new(-1.0, 0.0, 0.0));
    }
    #[test]
    fn reflect_grazing_ray_unchanged() {
        let incident = Direction::new(1.0, 0.0, 0.0);
        let normal = Direction::new(0.0, 1.0, 0.0).normalize();

        let result = incident.reflect(normal);

        assert_eq!(result, incident);
    }
    #[test]
    fn reflect_45_degree_incidence() {
        let incident = Direction::new(1.0, -1.0, 0.0);
        let normal = Direction::new(0.0, 1.0, 0.0).normalize();

        let result = incident.reflect(normal);

        assert_eq!(result, Direction::new(1.0, 1.0, 0.0));
    }
    #[test]
    fn reflect_head_on_reverses_direction() {
        let incident = Direction::new(0.0, 0.0, -1.0);
        let normal = Direction::new(0.0, 0.0, 1.0).normalize();

        let result = incident.reflect(normal);

        assert_eq!(result, Direction::new(0.0, 0.0, 1.0));
    }
    #[test]
    fn reflect_incident_angle_equals_reflected_angle() {
        let incident = Direction::new(3.0, -4.0, 0.0);
        let normal = Direction::new(0.0, 1.0, 0.0).normalize();

        let reflected = incident.reflect(normal);

        assert!((incident.dot(*normal) + reflected.dot(*normal)).abs() < 1e-5);
    }

    #[test]
    fn refract_normal_incidence_continues_straight() {
        let incident = Direction::new(0.0, -1.0, 0.0);
        let normal = Direction::new(0.0, 1.0, 0.0).normalize();

        let result = incident.refract(normal, 1.0, 1.5);

        assert_eq!(result, Direction::new(0.0, -1.0, 0.0));
    }

    #[test]
    fn refract_air_to_glass_at_45_degrees() {
        let incident = Direction::new(1.0, -1.0, 0.0).normalize();
        let normal = Direction::new(0.0, 1.0, 0.0).normalize();

        let result = incident.refract(normal, 1.0, 1.5);

        assert!((result.x - 0.47140452).abs() < 1e-5);
        assert!((result.y - (-0.8819171)).abs() < 1e-5);
        assert_eq!(result.z, 0.0);
    }

    #[test]
    fn refract_into_denser_medium_bends_toward_normal() {
        let incident = Direction::new(1.0, -1.0, 0.0).normalize();
        let normal = Direction::new(0.0, 1.0, 0.0).normalize();

        let refracted = incident.refract(normal, 1.0, 1.5);

        assert!(-refracted.dot(*normal) > -incident.dot(*normal));
    }

    #[test]
    fn refract_air_to_glass_differs_from_glass_to_air() {
        let incident = Direction::new(1.0, -1.0, 0.0).normalize();
        let normal = Direction::new(0.0, 1.0, 0.0).normalize();

        let into_glass = incident.refract(normal, 1.0, 1.5);
        let out_of_glass = incident.refract(normal, 1.5, 1.0);

        assert_ne!(into_glass, out_of_glass);
    }
}
