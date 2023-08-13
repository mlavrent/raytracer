use crate::materials::Material;
use crate::utils::Position;
use crate::raytracer::ray::Ray;

pub mod sphere;
pub mod rectangle;

pub struct RenderableShape<S: Shape> {
  pub shape: S,
  pub material: Material,
}

pub trait Shape {
  fn ray_hits(ray: &Ray) -> Option<Position>;
}
