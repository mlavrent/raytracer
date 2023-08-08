use crate::utils::Position;

pub struct Camera {
  eye_ppsition: Position,
  viewport: Viewport,
}

pub struct Viewport {
  top_left: Position,
  top_right: Position,
  bottom_left: Position,
}
