use crate::camera::Camera;
use crate::shapes::Renderable;
use crate::utils::Color;

use super::ray::Ray;

pub struct Scene<'a> {
  pub camera: Camera,
  pub shapes: Vec<&'a dyn Renderable>,
}

impl Scene<'_> {
  pub fn get_ray_color(&self, ray: &Ray) -> Color {
    todo!()
  }
}
