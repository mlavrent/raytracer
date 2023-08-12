use nalgebra::{Vector3, UnitVector3, Unit};
use crate::utils::Position;

pub struct Ray {
  origin: Position,
  direction: UnitVector3<f64>,
}

impl Ray {
  pub fn new(origin: Position, direction: Vector3<f64>) -> Ray {
    Ray {
        origin,
        direction: Unit::new_normalize(direction),
    }
  }
}
