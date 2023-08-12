use crate::materials::Material;

pub mod sphere;
pub mod rectangle;

pub struct RenderableShape<S> {
  pub shape: S,
  pub material: Material,
}
