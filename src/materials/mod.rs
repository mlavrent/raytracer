use enum_map::{EnumMap, Enum};

use crate::{utils::Color, raytracer::ray::Ray, shapes::HitInfo};

pub struct Material {
  pub color: Color,
  pub scatter_percentages: EnumMap<ScatterType, f64>,
}

#[derive(Enum)]
pub enum ScatterType {
  Diffuse,
  Specular,
  Refractive,
}

impl Material {
  pub fn scatter_ray(&self, in_ray: &Ray, hit_info: &HitInfo) -> Ray {
    todo!()
  }

  fn diffuse_scatter(&self, in_ray: &Ray, hit_info: &HitInfo) -> Ray {
    todo!()
  }

  fn specular_scatter(&self, in_ray: &Ray, hit_info: &HitInfo) -> Ray {
    todo!()
  }

  fn refractive_scatter(&self, in_ray: &Ray, hit_info: &HitInfo) -> Ray {
    todo!()
  }
}
