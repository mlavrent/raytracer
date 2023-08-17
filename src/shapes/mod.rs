use nalgebra::Vector3;

use crate::materials::Material;
use crate::raytracer::ray::Ray;

pub mod sphere;
pub mod rectangle;

pub trait Hittable {
  fn ray_hits(&self, ray: &Ray) -> Option<HitInfo>;
}

pub struct HitInfo {
  pub hit_normal: Ray,
  pub distance_to_hit: f64,
}

pub struct RenderableShape<'a> {
  pub shape: &'a (dyn Hittable + Sync),
  pub material: &'a (dyn Material + Sync),
}

pub(crate) fn cos_incidence_angle(in_ray: &Ray, hit_info: &HitInfo) -> f64 {
  -hit_info.hit_normal.direction.dot(&in_ray.direction.into_inner())
}

pub(crate) fn cos_incidence_angle_vec(in_direction: Vector3<f64>, normal: Vector3<f64>) -> f64 {
  -normal.dot(&in_direction) / (in_direction.magnitude() * normal.magnitude())
}
