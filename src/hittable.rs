use crate::{
    direction::Direction,
    point::Point,
    interval::Interval,
    ray::Ray,
};

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitResult>;
}

pub struct HitResult {
    pub point: Point,
    pub t: f32,
    pub normal: Direction,
}
