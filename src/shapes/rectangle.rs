use nalgebra::Vector3;

use crate::utils::{Position, UVCoords};


pub struct Rectangle {
  pub top_left: Position,
  pub top_edge: Vector3<f64>,
  pub left_edge: Vector3<f64>,
}

impl Rectangle {
  pub fn bottom_left(&self) -> Position { self.top_left + self.left_edge }

  pub fn top_right(&self) -> Position { self.top_left + self.top_edge }

  pub fn bottom_right(&self) -> Position { self.top_left + self.top_edge + self.left_edge }

  pub fn width(&self) -> f64 { self.top_edge.norm() }

  pub fn height(&self) -> f64 { self.left_edge.norm() }

  pub fn center(&self) -> Position { self.top_left + self.top_edge.scale(0.5) + self.left_edge.scale(0.5) }

  pub fn position_from_uv_coords(&self, uv_coords: UVCoords) -> Position {
    self.top_left + (self.top_edge * uv_coords.index(0)) + (self.left_edge * uv_coords.index(1))
  }
}
