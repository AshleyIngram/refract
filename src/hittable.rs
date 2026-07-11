use crate::{direction::UnitDirection, interval::Interval, point::Point, ray::Ray};

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitResult>;
}

pub struct HitResult {
    pub point: Point,
    pub t: f32,
    pub normal: UnitDirection,
    pub front_face: bool,
}

impl HitResult {
    pub fn new(ray: &Ray, point: Point, t: f32, outward_normal: UnitDirection) -> Self {
        let front_face = ray.direction.dot(*outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            (-*outward_normal).normalize()
        };
        Self {
            point,
            t,
            front_face,
            normal,
        }
    }
}
