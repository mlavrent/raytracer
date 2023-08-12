use nalgebra::{Vector3, DMatrix};

use crate::raytracer::ray::Ray;
use crate::utils::Position;
use crate::shapes::rectangle::Rectangle;

pub struct Camera {
  pub eye_position: Position,
  pub viewport: Rectangle,
  pub pixel_density: f64,
}

impl Camera {
  pub fn focal_vector(&self) -> Vector3<f64> { self.viewport.center() - self.eye_position }

  pub fn viewport_width(&self) -> f64 { self.viewport.top_edge.norm() }

  pub fn viewport_height(&self) -> f64 { self.viewport.left_edge.norm() }

  pub fn pixel_width(&self) -> u32 { (self.viewport_width() * self.pixel_density) as u32 }

  pub fn pixel_height(&self) -> u32 { (self.viewport_height() * self.pixel_density) as u32 }

  pub fn pixel_points(&self) -> DMatrix<Position> { todo!() }

  pub fn pixel_to_ray(&self, pixel: Position) -> Ray { Ray::new(self.eye_position, pixel - self.eye_position) }
}
