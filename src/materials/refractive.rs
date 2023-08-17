use nalgebra::Vector3;
use rand::prelude::Distribution;

use crate::shapes::{cos_incidence_angle, cos_incidence_angle_vec};
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
  fn reflectance(&self, cosine_incidence_angle: f64, refraction_ratio: f64) -> f64 {
    // uses Shlick's approximation
    let r0 = ((1.0 - refraction_ratio) / (1.0 + refraction_ratio)).powi(2);
    let r = r0 + (1.0 - r0) * (1.0 - cosine_incidence_angle).powi(5);
    r.clamp(0.0, 1.0)
  }
}

impl Material for RefractiveMaterial {
  fn scatter_ray(&self, in_ray: &Ray, hit_info: &HitInfo) -> ScatterInfo {
    let cos_incidence_angle = cos_incidence_angle(in_ray, hit_info);
    let refraction_ratio = if cos_incidence_angle > 0.0 { self.refraction_index } else { 1.0 / self.refraction_index };

    let reflectance = 0.0; // self.reflectance(cosine_incidence_angle, refraction_ratio); TODO: figure this out
    let scatter_type_distribution = DiscreteDistribution::new([(ScatterType::Reflection, reflectance), (ScatterType::Refraction, 1.0 - reflectance)]);
    let scatter_direction = match scatter_type_distribution.sample(&mut rand::thread_rng()) {
      ScatterType::Reflection => reflection_direction(in_ray.direction.into_inner(), hit_info.hit_normal.direction.into_inner()),
      ScatterType::Refraction => refraction_direction(in_ray.direction.into_inner(), hit_info.hit_normal.direction.into_inner(), refraction_ratio),
    };

    ScatterInfo {
      scattered_ray: Some(Ray::new(hit_info.hit_normal.origin, scatter_direction)),
      attenuation: self.color,
    }
  }
}

pub(super) fn refraction_direction(in_direction: Vector3<f64>, normal: Vector3<f64>, refraction_index: f64) -> Vector3<f64> {
  let cos_incidence_angle = cos_incidence_angle_vec(in_direction, normal).abs();

  in_direction * refraction_index
    + normal * (refraction_index * cos_incidence_angle - (1.0 - refraction_index.powi(2) * (1.0 - cos_incidence_angle.powi(2)).sqrt()))
}

#[derive(Clone, Copy)]
enum ScatterType {
  Reflection,
  Refraction,
}
