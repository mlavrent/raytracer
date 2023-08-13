use nalgebra::{Vector3, UnitVector3, Unit};
use crate::utils::Position;

pub struct Ray {
  pub origin: Position,
  pub direction: UnitVector3<f64>,
}

impl Ray {
  pub fn new(origin: Position, direction: Vector3<f64>) -> Ray {
    Ray {
      origin,
      direction: Unit::new_normalize(direction),
    }
  }

  pub fn at_distance(&self, distance: f64) -> Position {
    self.origin + (self.direction.into_inner() * distance)
  }
}
