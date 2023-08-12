use nalgebra::{Vector3, Vector2};

use crate::raytracer::ray::Ray;
use crate::utils::{Position, UVCoords};
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

  pub fn get_pixel_ray(&self, pixel_coords: Vector2<u32>) -> Ray {
    let uv_coords: UVCoords = pixel_coords * self.pixel_density;
    let pixel_position: Position = self.viewport.position_from_uv_coords(uv_coords);

    Ray {
      origin: self.eye_position,
      direction: pixel_position - self.eye_position
    }
  }

  pub fn point_to_ray(&self, pixel: Position) -> Ray { Ray::new(self.eye_position, pixel - self.eye_position) }
}
