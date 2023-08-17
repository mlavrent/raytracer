use nalgebra::Vector3;
use rand::prelude::Distribution;

use crate::utils::DiscreteDistribution;
use crate::{shapes::HitInfo, utils::Color};
use crate::raytracer::ray::Ray;
use super::specular::reflection_direction;
use super::{Material, ScatterInfo};


pub struct RefractiveMaterial {
  pub refraction_index: f64,
  pub color: Color,
}

impl RefractiveMaterial {
  fn reflectance(&self, cosine_incidence_angle: f64) -> f64 {
    // uses Shlick's approximation
    let r0 = ((1.0 - self.refraction_index) / (1.0 + self.refraction_index)).powi(2);
    let r = r0 + (1.0 - r0) * (1.0 - cosine_incidence_angle).powi(5);
    r.clamp(0.0, 1.0)
  }
}

impl Material for RefractiveMaterial {
  fn scatter_ray(&self, in_ray: &Ray, hit_info: &HitInfo) -> ScatterInfo {
    let cosine_incidence_angle = -hit_info.hit_normal.direction.dot(&in_ray.direction);
    let reflectance = self.reflectance(cosine_incidence_angle);

    let refraction_ratio = if cosine_incidence_angle < 0.0 {self.refraction_index} else {1.0/self.refraction_index};

    let scatter_type_distribution = DiscreteDistribution::new([(ScatterType::Reflection, reflectance), (ScatterType::Refraction, 1.0 - reflectance)]);
    let scatter_direction = match scatter_type_distribution.sample(&mut rand::thread_rng()) {
      ScatterType::Reflection => reflection_direction(in_ray.direction.into_inner(), hit_info.hit_normal.direction.into_inner()),
      ScatterType::Refraction => refraction_direction(in_ray.direction.into_inner(), hit_info.hit_normal.direction.into_inner(), refraction_ratio),
    };

    // TODO: use 1/refraction_index if normal & ray have angle < 90 i.e. the cosine is positive
    // let scatter_direction = refraction_direction(in_ray.direction.into_inner(), hit_info.hit_normal.direction.into_inner(), refraction_ratio);

    ScatterInfo {
      scattered_ray: Ray::new(hit_info.hit_normal.origin, scatter_direction),
      attenuation: self.color,
    }
  }
}

pub(super) fn refraction_direction(in_direction: Vector3<f64>, normal: Vector3<f64>, refraction_index: f64) -> Vector3<f64> {
  let cosine_incidence_angle = normal.dot(&in_direction);

  in_direction * refraction_index
    + normal * (refraction_index * cosine_incidence_angle - (1.0 - refraction_index.powi(2) * (1.0 - cosine_incidence_angle.powi(2)).sqrt()))
}

#[derive(Clone, Copy)]
enum ScatterType {
  Reflection,
  Refraction,
}
