use crate::{
    color::Color, direction::UnitDirection, hittable::HitResult, ray::Ray, rng::random_range,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReflectionType {
    Diffuse,
    Lambertian,
}

pub struct ScatterResult {
    pub attenuation: Color,
    pub scattered: Ray,
}

pub trait Material : Send + Sync {
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
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        Self {
            albedo,
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_result: &HitResult) -> Option<ScatterResult> {
        let reflected = (*ray.direction.reflect(hit_result.normal).normalize())
            + (self.fuzz * *UnitDirection::random_unit_direction());
        let scattered = Ray::new(hit_result.point, reflected);

        if scattered.direction.dot(*hit_result.normal) > 0.0 {
            Some(ScatterResult {
                attenuation: self.albedo,
                scattered,
            })
        } else {
            None
        }
    }
}

pub struct Dielectric {
    refractive_index: f32,
}

impl Dielectric {
    pub fn new(refractive_index: f32) -> Self {
        Self { refractive_index }
    }

    /// Reflectance determines the ratio of light that is reflected vs refracted for a given Ray intersecting with a Material at a given angle.
    ///
    /// Uses Schlick's approximation to calculate the reflectance.
    fn reflectance(&self, cosine: f32, refractive_ratio: f32) -> f32 {
        let r0 = f32::powi((1.0 - refractive_ratio) / (1.0 + refractive_ratio), 2);
        r0 + (1.0 - r0) * f32::powi(1.0 - cosine, 5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_result: &HitResult) -> Option<ScatterResult> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let (from_index, to_index) = if hit_result.front_face {
            (1.0, self.refractive_index)
        } else {
            (self.refractive_index, 1.0)
        };

        let unit_direction = *ray.direction.normalize();
        let cos_theta = f32::min(-unit_direction.dot(*hit_result.normal), 1.0);
        let sin_theta = f32::sqrt(1.0 - cos_theta * cos_theta);

        let refractive_ratio = from_index / to_index;
        let cannot_refract = sin_theta * refractive_ratio > 1.0;
        let direction = if cannot_refract
            || self.reflectance(cos_theta, refractive_ratio) > random_range(0.0..=1.0)
        {
            unit_direction.reflect(hit_result.normal)
        } else {
            unit_direction.refract(hit_result.normal, from_index, to_index)
        };

        Some(ScatterResult {
            attenuation,
            scattered: Ray::new(hit_result.point, direction),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::{direction::Direction, point::Point};

    use super::*;

    fn hit_result_at(
        incident: &Ray,
        point: Point,
        outward_normal: UnitDirection,
        material: Arc<dyn Material>,
    ) -> HitResult {
        HitResult::new(incident, point, 1.0, outward_normal, material)
    }

    #[test]
    fn matte_scatter_passes_through_albedo() {
        let albedo = Color::new(0.1, 0.2, 0.5);
        let matte: Arc<dyn Material> = Arc::new(Matte::new(albedo, ReflectionType::Lambertian));
        let point = Point::new(0.0, 0.0, -1.0);
        let normal = Direction::new(0.0, 1.0, 0.0).normalize();
        let incident = Ray::new(Point::new(0.0, 0.0, 0.0), Direction::new(0.0, -1.0, 0.0));
        let hit = hit_result_at(&incident, point, normal, Arc::clone(&matte));

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
        let incident = Ray::new(Point::new(0.0, 0.0, 0.0), Direction::new(0.0, -1.0, 0.0));
        let hit = hit_result_at(&incident, point, normal, Arc::clone(&matte));

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
        let incident = Ray::new(Point::new(0.0, 1.0, 0.0), Direction::new(0.0, -1.0, 0.0));
        let hit = hit_result_at(&incident, point, normal, Arc::clone(&matte));

        let first = matte.scatter(&incident, &hit).unwrap();
        crate::rng::reseed(99);
        let second = matte.scatter(&incident, &hit).unwrap();

        assert_eq!(first.scattered.direction, second.scattered.direction);
    }

    #[test]
    fn metal_scatter_reflects_off_floor() {
        let albedo = Color::new(0.8, 0.6, 0.2);
        let metal: Arc<dyn Material> = Arc::new(Metal::new(albedo, 0.0));
        let point = Point::new(0.0, 0.0, -1.0);
        let normal = Direction::new(0.0, 1.0, 0.0).normalize();
        let incident = Ray::new(Point::new(0.0, 1.0, 0.0), Direction::new(0.0, -1.0, 0.0));
        let hit = hit_result_at(&incident, point, normal, Arc::clone(&metal));

        let scatter = metal.scatter(&incident, &hit).unwrap();

        assert_eq!(scatter.attenuation, albedo);
        assert_eq!(scatter.scattered.origin, point);
        assert_eq!(scatter.scattered.direction, Direction::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn metal_scatter_reflects_at_45_degrees() {
        let metal: Arc<dyn Material> = Arc::new(Metal::new(Color::new(1.0, 1.0, 1.0), 0.0));
        let point = Point::new(0.0, 0.0, 0.0);
        let normal = Direction::new(0.0, 1.0, 0.0).normalize();
        let incident = Ray::new(Point::new(-1.0, 1.0, 0.0), Direction::new(1.0, -1.0, 0.0));
        let hit = hit_result_at(&incident, point, normal, Arc::clone(&metal));

        let scatter = metal.scatter(&incident, &hit).unwrap();

        assert_eq!(
            scatter.scattered.direction,
            *Direction::new(1.0, 1.0, 0.0).normalize()
        );
    }
}
