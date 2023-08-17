use nalgebra::Vector3;

use crate::raytracer::ray::Ray;
use crate::shapes::HitInfo;
use crate::utils::Color;

use super::{Material, ScatterInfo};

pub struct ReflectiveMaterial {
  pub color: Color,
}

impl Material for ReflectiveMaterial {
  fn scatter_ray(&self, in_ray: &Ray, hit_info: &HitInfo) -> ScatterInfo {
    ScatterInfo {
      scattered_ray: Ray::new(hit_info.hit_normal.origin, reflection_direction(in_ray.direction.into_inner(), hit_info.hit_normal.direction.into_inner())),
      attenuation: self.color,
    }
  }
}

pub(super) fn reflection_direction(in_direction: Vector3<f64>, normal: Vector3<f64>) -> Vector3<f64> {
  let in_ray_normal_component = in_direction.dot(&normal);
  in_direction - (2.0 * in_ray_normal_component * normal)
}
