use nalgebra::Vector3;
use rand::prelude::Distribution;

use crate::{raytracer::ray::Ray, utils::UniformSphereDistribution};
use crate::shapes::HitInfo;
use crate::utils::Color;

use super::{Material, ScatterInfo};

pub struct ReflectiveMaterial {
  pub color: Color,
  pub fuzz: f64,
  fuzz_distribution: UniformSphereDistribution,
}

impl ReflectiveMaterial {
  pub fn new(color: Color, fuzz: f64) -> ReflectiveMaterial {
    ReflectiveMaterial {
      color,
      fuzz,
      fuzz_distribution: UniformSphereDistribution { radius: fuzz },
    }
  }
}

impl Material for ReflectiveMaterial {
  fn scatter_ray(&self, in_ray: &Ray, hit_info: &HitInfo) -> ScatterInfo {
    let reflection_ray = Ray::new(hit_info.hit_normal.origin, reflection_direction(in_ray.direction.into_inner(), hit_info.hit_normal.direction.into_inner()));

    let fuzz_offset = self.fuzz_distribution.sample(&mut rand::thread_rng());
    let fuzzy_ray = Ray::new(reflection_ray.origin, reflection_ray.direction.into_inner() + fuzz_offset);

    ScatterInfo {
      scattered_ray: Some(fuzzy_ray),
      attenuation: self.color,
    }
  }
}

pub(super) fn reflection_direction(in_direction: Vector3<f64>, normal: Vector3<f64>) -> Vector3<f64> {
  let in_ray_normal_component = in_direction.dot(&normal);
  in_direction - (2.0 * in_ray_normal_component * normal)
}
