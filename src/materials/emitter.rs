use crate::utils::Color;
use crate::raytracer::ray::Ray;
use crate::shapes::HitInfo;
use super::{Material, ScatterInfo};

pub struct EmitterMaterial {
  pub color: Color,
}

impl Material for EmitterMaterial {
  fn scatter_ray(&self, _: &Ray, _: &HitInfo) -> ScatterInfo {
    ScatterInfo { scattered_ray: None, attenuation: self.color }
  }
}
