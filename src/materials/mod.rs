pub mod specular;
pub mod diffuse;
pub mod refractive;
pub mod emitter;

use crate::{raytracer::ray::Ray, shapes::HitInfo, utils::Color};

pub trait Material {
  fn scatter_ray(&self, in_ray: &Ray, hit_info: &HitInfo) -> ScatterInfo;
}

pub struct ScatterInfo {
  pub scattered_ray: Option<Ray>,
  pub attenuation: Color,
}
