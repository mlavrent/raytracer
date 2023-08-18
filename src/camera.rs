use std::array;

use nalgebra::{Vector3, Vector2, vector, Unit};

use crate::raytracer::ray::Ray;
use crate::utils::{Position, rad_to_deg, deg_to_rad};
use crate::shapes::rectangle::Rectangle;

pub struct Camera {
  pub eye_position: Position,
  pub viewport: Rectangle,
  pub pixel_density: f64,
}

impl Camera {
  pub fn new(position: Position, focal_point: Position, hfov_deg: f64, vfov_deg: f64, up_direction: Vector3<f64>, pixel_width: u32) -> Self {
    let focal_vector = focal_point - position;
    let focal_length = focal_vector.magnitude();
    let viewport_width = 2.0 * focal_length * deg_to_rad(hfov_deg / 2.0).tan();
    let viewport_height = 2.0 * focal_length * deg_to_rad(vfov_deg / 2.0).tan();

    // guarantee the viewport is perpendicular to the focal vector
    let perp_right_dir = Unit::new_normalize(focal_vector.cross(&up_direction));
    let perp_up_dir = Unit::new_normalize(perp_right_dir.cross(&focal_vector));

    Camera {
      eye_position: position,
      viewport: Rectangle::new_from_center(focal_point, viewport_width, viewport_height, perp_up_dir, perp_right_dir),
      pixel_density: (pixel_width as f64) / viewport_width,
    }
  }

  pub fn focal_vector(&self) -> Vector3<f64> { self.viewport.center() - self.eye_position }

  pub fn focal_length(&self) -> f64 { self.focal_vector().magnitude() }

  pub fn viewport_width(&self) -> f64 { self.viewport.top_edge.norm() }

  pub fn viewport_height(&self) -> f64 { self.viewport.left_edge.norm() }

  pub fn pixel_width(&self) -> u32 { (self.viewport_width() * self.pixel_density) as u32 }

  pub fn pixel_height(&self) -> u32 { (self.viewport_height() * self.pixel_density) as u32 }

  pub fn hfov_rad(&self) -> f64 { 2.0 * (self.viewport_width() / (2.0 * self.focal_length())).atan() }

  pub fn vfov_rad(&self) -> f64 { 2.0 * (self.viewport_height() / (2.0 * self.focal_length())).atan() }

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
