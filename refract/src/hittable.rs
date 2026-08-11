use std::sync::Arc;

use crate::{
    bounding_box::BoundingBox, direction::UnitDirection, interval::Interval, material::Material,
    point::Point, ray::Ray,
};

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitResult>;

    fn bounding_box(&self) -> BoundingBox;
}

pub struct HitResult {
    pub point: Point,
    pub t: f32,
    pub normal: UnitDirection,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
    pub u: f32,
    pub v: f32,
}

impl HitResult {
    pub fn new(
        ray: &Ray,
        point: Point,
        t: f32,
        u: f32,
        v: f32,
        outward_normal: UnitDirection,
        material: Arc<dyn Material>,
    ) -> Self {
        let front_face = ray.direction().dot(*outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            (-*outward_normal).normalize()
        };
        Self {
            point,
            t,
            u,
            v,
            front_face,
            normal,
            material,
        }
    }
}
