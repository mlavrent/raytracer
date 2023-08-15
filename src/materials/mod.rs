use rand::prelude::Distribution;
use crate::{raytracer::ray::Ray, shapes::HitInfo};
use crate::utils::{Color, DiscreteDistribution, LambertianDistribution};

pub struct Material {
  pub color: Color,
  pub scatter_percentages: DiscreteDistribution<ScatterType, 3>,
}

#[derive(Clone, Copy)]
pub enum ScatterType {
  Diffuse,
  Specular,
  Refractive(f64),
}

impl Material {
  pub fn scatter_ray(&self, in_ray: &Ray, hit_info: &HitInfo) -> Ray {
    match self.scatter_percentages.sample(&mut rand::thread_rng()) {
      ScatterType::Diffuse => self.diffuse_scatter(in_ray, hit_info),
      ScatterType::Specular => self.specular_scatter(in_ray, hit_info),
      ScatterType::Refractive(refraction_index) => self.refractive_scatter(in_ray, hit_info, refraction_index),
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
    let in_ray_normal_component = in_ray.direction.dot(&hit_info.hit_normal.direction.into_inner());
    let reflected_direction = in_ray.direction.into_inner() - (2.0 * in_ray_normal_component * hit_info.hit_normal.direction.into_inner());

    Ray::new(hit_info.hit_normal.origin, reflected_direction)
  }

  fn refractive_scatter(&self, in_ray: &Ray, hit_info: &HitInfo, refraction_index: f64) -> Ray {
    // let incidence_angle
    Ray::new(hit_info.hit_normal.origin, todo!())
  }
}
