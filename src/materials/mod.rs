pub mod specular;
pub mod diffuse;
pub mod refractive;

use rand::prelude::Distribution;
use crate::{raytracer::ray::Ray, shapes::HitInfo};

pub trait Material {
  fn scatter_ray(&self, in_ray: &Ray, hit_info: &HitInfo) -> Ray;
}
