use crate::utils::Position;
use super::Shape;

pub struct Sphere {
  pub center: Position,
  pub radius: f64,
}

impl Shape for Sphere {
  fn ray_hits(ray: &crate::raytracer::ray::Ray) -> Option<Position> {
    todo!()
  }
}
