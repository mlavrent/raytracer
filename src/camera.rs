use std::array;
use std::f64::consts::PI;

use nalgebra::{Vector3, Vector2, vector};

use crate::raytracer::ray::Ray;
use crate::utils::{Position, rad_to_deg};
use crate::shapes::rectangle::Rectangle;

pub struct Camera {
  pub eye_position: Position,
  pub viewport: Rectangle,
  pub pixel_density: f64,
}

impl Camera {
  pub fn focal_vector(&self) -> Vector3<f64> { self.viewport.center() - self.eye_position }

  pub fn focal_distance(&self) -> f64 { self.focal_vector().magnitude() }

  pub fn viewport_width(&self) -> f64 { self.viewport.top_edge.norm() }

  pub fn viewport_height(&self) -> f64 { self.viewport.left_edge.norm() }

  pub fn pixel_width(&self) -> u32 { (self.viewport_width() * self.pixel_density) as u32 }

  pub fn pixel_height(&self) -> u32 { (self.viewport_height() * self.pixel_density) as u32 }

  pub fn hfov_rad(&self) -> f64 { 2.0 * (self.viewport_width() / (2.0 * self.focal_distance())).atan() }

  pub fn vfov_rad(&self) -> f64 { 2.0 * (self.viewport_height() / (2.0 * self.focal_distance())).atan() }

  pub fn hfov_deg(&self) -> f64 { rad_to_deg(self.hfov_rad()) }

  pub fn vfov_deg(&self) -> f64 { rad_to_deg(self.vfov_rad()) }

  pub fn get_pixel_center_ray(&self, pixel_coords: Vector2<u32>) -> Ray {
    let offset: Vector2<f64> = pixel_coords.cast() / self.pixel_density;
    let pixel_position: Position = self.viewport.position_from_offset(offset);

    Ray::new(self.eye_position, pixel_position - self.eye_position)
  }

  pub fn get_pixel_rays<const N: usize>(&self, pixel_coords: Vector2<u32>) -> [Ray; N] {
    array::from_fn(|_| {
      let offset: Vector2<f64> = (pixel_coords.cast() + vector![rand::random(), rand::random()]) / self.pixel_density;
      let pixel_position: Position = self.viewport.position_from_offset(offset);

      Ray::new(self.eye_position, pixel_position - self.eye_position)
    })
  }
}
