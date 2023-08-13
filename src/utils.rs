use image::{Rgb, Pixel};
use nalgebra::Vector3;

pub type Position = Vector3<f64>;
pub type Color = Rgb<f64>;

pub fn color_to_eight_bit(color: Color) -> Rgb<u8> {
  let renormalized_image = color.map(|x| (x * (u8::MAX as f64 - u8::MIN as f64) + u8::MIN as f64));
  Rgb([renormalized_image.channels()[0] as u8, renormalized_image.channels()[1] as u8, renormalized_image.channels()[2] as u8])
}

pub fn average_color(colors: &[Color]) -> Color {
  todo!()
}
