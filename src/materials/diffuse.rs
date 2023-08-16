use rand::prelude::Distribution;

use crate::utils::{Color, LambertianDistribution};
use crate::shapes::HitInfo;
use crate::raytracer::ray::Ray;
use super::Material;

pub struct DiffuseMaterial {
  pub color: Color,
}

impl Material for DiffuseMaterial {
  fn scatter_ray(&self, in_ray: &Ray, hit_info: &HitInfo) -> Ray {
    let scatter_distribution = LambertianDistribution {
      point: hit_info.hit_normal.origin,
      normal: hit_info.hit_normal.direction,
    };

    Ray::new(hit_info.hit_normal.origin, scatter_distribution.sample(&mut rand::thread_rng()))
  }
}
