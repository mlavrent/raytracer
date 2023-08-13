use image::Rgb;
use nalgebra::{Vector3, vector};

pub type Position = Vector3<f64>;
pub type Color = Vector3<f64>;

pub fn color_to_eight_bit(color: Color) -> Rgb<u8> {
  let eight_bit_color = color.map(channel_to_eight_bit);
  Rgb([eight_bit_color[0], eight_bit_color[1], eight_bit_color[2]])
}

fn channel_to_eight_bit(percentage: f64) -> u8 {
  (percentage * (u8::MAX as f64 - u8::MIN as f64) + u8::MIN as f64) as u8
}

pub fn average(vecs: &[Vector3<f64>]) -> Option<Vector3<f64>> {
  if vecs.len() == 0 {
    None
  }
  else {
    Some(vecs.iter().fold(vector![0.0, 0.0, 0.0], |b, v| b + v) / vecs.len() as f64)
  }
}
