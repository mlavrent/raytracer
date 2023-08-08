use crate::materials::Material;

pub mod sphere;
pub mod rectangle;

pub struct RenderableShape<S> {
  shape: S,
  material: Material,
}
