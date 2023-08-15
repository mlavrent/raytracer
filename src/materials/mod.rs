use rand::prelude::Distribution;
use crate::{raytracer::ray::Ray, shapes::HitInfo};
use crate::utils::{Color, WeightedEnumDistribution, LambertianDistribution};

pub struct Material {
  pub color: Color,
  pub scatter_percentages: WeightedEnumDistribution<ScatterType, 3>,
}

#[derive(Clone, Copy)]
pub enum ScatterType {
  Diffuse,
  Specular,
  Refractive,
}

impl Material {
  pub fn scatter_ray(&self, in_ray: &Ray, hit_info: &HitInfo) -> Ray {
    match self.scatter_percentages.sample(&mut rand::thread_rng()) {
      ScatterType::Diffuse => self.diffuse_scatter(in_ray, hit_info),
      ScatterType::Specular => self.specular_scatter(in_ray, hit_info),
      ScatterType::Refractive => self.refractive_scatter(in_ray, hit_info),
    }
  }

  fn diffuse_scatter(&self, _: &Ray, hit_info: &HitInfo) -> Ray {
    let scatter_distribution = LambertianDistribution {
      point: hit_info.hit_normal.origin,
      normal: hit_info.hit_normal.direction,
    };

    Ray::new(hit_info.hit_normal.origin, scatter_distribution.sample(&mut rand::thread_rng()))
  }

  fn specular_scatter(&self, in_ray: &Ray, hit_info: &HitInfo) -> Ray {
    todo!()
  }

  fn refractive_scatter(&self, in_ray: &Ray, hit_info: &HitInfo) -> Ray {
    todo!()
  }
}
