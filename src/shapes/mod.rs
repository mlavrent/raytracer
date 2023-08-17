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

impl HitInfo {
  pub fn cosine_incidence_angle(&self, in_ray: &Ray) -> f64 {
    -self.hit_normal.direction.dot(&in_ray.direction.into_inner())
  }
}

pub struct RenderableShape<'a> {
  pub shape: &'a (dyn Hittable + Sync),
  pub material: &'a (dyn Material + Sync),
}
