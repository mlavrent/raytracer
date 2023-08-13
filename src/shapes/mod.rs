use image::Rgb;
use nalgebra::UnitVector3;
use crate::materials::Material;
use crate::raytracer::ray::Ray;
use crate::raytracer::scene::Scene;
use crate::utils::{Color, Position};

pub mod sphere;
pub mod rectangle;

pub trait Hittable {
  fn ray_hits(&self, ray: &Ray) -> Option<HitInfo>;
}

pub struct HitInfo {
  pub hit_normal: Ray,
  pub distance_to_hit: f64,
}

pub struct RenderableShape {
  pub shape: Box<dyn Hittable>,
  pub material: Material,
}
