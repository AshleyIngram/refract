use crate::{color::Color, direction::UnitDirection, hittable::HitResult, ray::Ray};

pub enum ReflectionType {
    #[allow(
        dead_code,
        reason = "Will be used by the user to select the reflection type"
    )]
    Diffuse,
    Lambertian,
}

pub struct ScatterResult {
    pub attenuation: Color,
    pub scattered: Ray,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_result: &HitResult) -> Option<ScatterResult>;
}

pub struct Matte {
    albedo: Color,
    reflection_type: ReflectionType,
}

impl Matte {
    pub fn new(albedo: Color, reflection_type: ReflectionType) -> Self {
        Self {
            albedo,
            reflection_type,
        }
    }
}

impl Material for Matte {
    fn scatter(&self, _ray: &Ray, hit_result: &HitResult) -> Option<ScatterResult> {
        let direction = match self.reflection_type {
            ReflectionType::Diffuse => *hit_result.normal + *UnitDirection::random_unit_direction(),
            ReflectionType::Lambertian => {
                *UnitDirection::random_hemisphere_direction(hit_result.normal)
            }
        };
        let scatter_direction = if direction.near_zero() {
            *hit_result.normal
        } else {
            direction
        };
        let scattered = Ray::new(hit_result.point, scatter_direction);

        Some(ScatterResult {
            attenuation: self.albedo,
            scattered,
        })
    }
}

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_result: &HitResult) -> Option<ScatterResult> {
        let reflected = ray.direction.reflect(hit_result.normal);
        let scattered = Ray::new(hit_result.point, reflected);

        Some(ScatterResult {
            attenuation: self.albedo,
            scattered,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{direction::Direction, point::Point};

    use super::*;

    fn hit_result_at(
        point: Point,
        outward_normal: UnitDirection,
        material: Arc<dyn Material>,
    ) -> HitResult {
        let ray = Ray::new(Point::new(0.0, 0.0, 0.0), Direction::new(0.0, 0.0, -1.0));
        HitResult::new(&ray, point, 1.0, outward_normal, material)
    }

    #[test]
    fn matte_scatter_passes_through_albedo() {
        let albedo = Color::new(0.1, 0.2, 0.5);
        let matte: Arc<dyn Material> = Arc::new(Matte::new(albedo, ReflectionType::Lambertian));
        let point = Point::new(0.0, 0.0, -1.0);
        let normal = Direction::new(0.0, 1.0, 0.0).normalize();
        let hit = hit_result_at(point, normal, Arc::clone(&matte));
        let incident = Ray::new(Point::new(0.0, 0.0, 0.0), Direction::new(0.0, -1.0, 0.0));

        let scatter = matte.scatter(&incident, &hit).unwrap();

        assert_eq!(scatter.attenuation, albedo);
    }

    #[test]
    fn matte_scatter_origin_is_hit_point() {
        let matte: Arc<dyn Material> = Arc::new(Matte::new(
            Color::new(1.0, 1.0, 1.0),
            ReflectionType::Lambertian,
        ));
        let point = Point::new(1.0, 2.0, 3.0);
        let normal = Direction::new(0.0, 1.0, 0.0).normalize();
        let hit = hit_result_at(point, normal, Arc::clone(&matte));
        let incident = Ray::new(Point::new(0.0, 0.0, 0.0), Direction::new(0.0, -1.0, 0.0));

        let scatter = matte.scatter(&incident, &hit).unwrap();

        assert_eq!(scatter.scattered.origin, point);
    }

    #[test]
    fn matte_lambertian_scatter_is_reproducible_with_seed() {
        crate::rng::reseed(99);
        let matte: Arc<dyn Material> = Arc::new(Matte::new(
            Color::new(1.0, 1.0, 1.0),
            ReflectionType::Lambertian,
        ));
        let point = Point::new(0.0, 0.0, 0.0);
        let normal = Direction::new(0.0, 1.0, 0.0).normalize();
        let hit = hit_result_at(point, normal, Arc::clone(&matte));
        let incident = Ray::new(Point::new(0.0, 1.0, 0.0), Direction::new(0.0, -1.0, 0.0));

        let first = matte.scatter(&incident, &hit).unwrap();
        crate::rng::reseed(99);
        let second = matte.scatter(&incident, &hit).unwrap();

        assert_eq!(first.scattered.direction, second.scattered.direction);
    }

    #[test]
    fn metal_scatter_reflects_off_floor() {
        let albedo = Color::new(0.8, 0.6, 0.2);
        let metal: Arc<dyn Material> = Arc::new(Metal::new(albedo));
        let point = Point::new(0.0, 0.0, -1.0);
        let normal = Direction::new(0.0, 1.0, 0.0).normalize();
        let hit = hit_result_at(point, normal, Arc::clone(&metal));
        let incident = Ray::new(Point::new(0.0, 1.0, 0.0), Direction::new(0.0, -1.0, 0.0));

        let scatter = metal.scatter(&incident, &hit).unwrap();

        assert_eq!(scatter.attenuation, albedo);
        assert_eq!(scatter.scattered.origin, point);
        assert_eq!(scatter.scattered.direction, Direction::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn metal_scatter_reflects_at_45_degrees() {
        let metal: Arc<dyn Material> = Arc::new(Metal::new(Color::new(1.0, 1.0, 1.0)));
        let point = Point::new(0.0, 0.0, 0.0);
        let normal = Direction::new(0.0, 1.0, 0.0).normalize();
        let hit = hit_result_at(point, normal, Arc::clone(&metal));
        let incident = Ray::new(Point::new(-1.0, 1.0, 0.0), Direction::new(1.0, -1.0, 0.0));

        let scatter = metal.scatter(&incident, &hit).unwrap();

        assert_eq!(scatter.scattered.direction, Direction::new(1.0, 1.0, 0.0));
    }
}
