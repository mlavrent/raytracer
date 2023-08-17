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
