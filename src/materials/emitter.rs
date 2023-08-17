use crate::utils::Color;
use crate::raytracer::ray::Ray;
use crate::shapes::HitInfo;
use super::{Material, ScatterInfo};

pub struct EmitterMaterial {
  pub color: Color,
}

impl Material for EmitterMaterial {
  fn scatter_ray(&self, in_ray: &Ray, hit_info: &HitInfo) -> ScatterInfo {
    let cosine_incidence_angle = hit_info.cosine_incidence_angle(in_ray);

    ScatterInfo { scattered_ray: None, attenuation: self.color * cosine_incidence_angle }
  }
}
