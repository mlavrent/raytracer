use crate::camera::Camera;
use crate::shapes::sphere::Sphere;
use crate::shapes::RenderableShape;

pub struct Scene {
  pub camera: Camera,
  pub sphere: RenderableShape<Sphere>,
}
