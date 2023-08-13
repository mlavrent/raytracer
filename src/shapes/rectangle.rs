use nalgebra::{Vector3, Vector2, vector};
use crate::utils::Position;
use super::Shape;


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

  pub fn position_from_percent_offset(&self, percent_offsets: Vector2<f64>) -> Position {
    self.top_left + (self.top_edge * percent_offsets[0]) + (self.left_edge * percent_offsets[1])
  }

  pub fn position_from_offset(&self, offsets: Vector2<f64>) -> Position {
    self.position_from_percent_offset(vector![offsets[0] / self.top_edge.norm(), offsets[1] / self.left_edge.norm()])
  }
}

impl Shape for Rectangle {
  fn ray_hits(&self, ray: &crate::raytracer::ray::Ray) -> Option<Position> {
    todo!()
  }
}
