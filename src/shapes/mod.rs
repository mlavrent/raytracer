use crate::materials::Material;
use crate::raytracer::ray::Ray;

pub mod sphere;
pub mod rectangle;

pub struct RenderableShape<S: Shape> {
  pub shape: S,
  pub material: Material,
}

pub trait Shape {
  /// Returns the ray whose origin is the hit point and whose direction
  /// is the surface normal vector at the hit point, if the shape was
  /// hit by the provided ray.
  fn ray_hits(&self, ray: &Ray) -> Option<Ray>;
}
