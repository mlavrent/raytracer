use nalgebra::Vector3;

use crate::utils::Position;


pub struct Rectangle {
  top_left: Position,
  top_edge: Vector3<f64>,
  left_edge: Vector3<f64>,
}

impl Rectangle {
  pub fn bottom_left(&mut self) -> Position { self.top_left + self.left_edge }

  pub fn top_right(&mut self) -> Position { self.top_left + self.top_edge }

  pub fn bottom_right(&mut self) -> Position { self.top_left + self.top_edge + self.left_edge }
}
