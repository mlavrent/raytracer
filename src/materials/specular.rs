use crate::raytracer::ray::Ray;
use crate::shapes::HitInfo;
use crate::utils::Color;

use super::{Material, ScatterInfo};


pub struct ReflectiveMaterial {
  pub color: Color,
}

impl Material for ReflectiveMaterial {
  fn scatter_ray(&self, in_ray: &Ray, hit_info: &HitInfo) -> ScatterInfo {
    let in_ray_normal_component = in_ray.direction.dot(&hit_info.hit_normal.direction.into_inner());
    let reflected_direction = in_ray.direction.into_inner() - (2.0 * in_ray_normal_component * hit_info.hit_normal.direction.into_inner());

    ScatterInfo {
      scattered_ray: Ray::new(hit_info.hit_normal.origin, reflected_direction),
      attenuation: self.color,
    }
  }
}
