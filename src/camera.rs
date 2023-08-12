use crate::utils::Position;
use crate::shapes::rectangle::Rectangle;

pub struct Camera {
  pub eye_position: Position,
  pub viewport: Rectangle,
  pub pixel_width: u32,
}

impl Camera {
  pub fn focal_length(&self) -> f64 { (self.viewport.center() - self.eye_position).norm() }

  pub fn aspect_ratio(&self) -> f64 { self.viewport.width() / self.viewport.height() }

  pub fn pixel_height(&self) -> u32 { self.pixel_width as f64 / self.aspect_ratio() }
}
