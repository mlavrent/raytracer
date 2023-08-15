use rand::prelude::Distribution;
use crate::{raytracer::ray::Ray, shapes::HitInfo};
use crate::utils::{Color, WeightedEnumDistribution};

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

  fn diffuse_scatter(&self, in_ray: &Ray, hit_info: &HitInfo) -> Ray {
    println!("Doing diffuse!");
    todo!()
  }

  fn specular_scatter(&self, in_ray: &Ray, hit_info: &HitInfo) -> Ray {
    todo!()
  }

  fn refractive_scatter(&self, in_ray: &Ray, hit_info: &HitInfo) -> Ray {
    todo!()
  }
}
