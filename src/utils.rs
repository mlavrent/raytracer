use image::{Rgb, RgbImage};
use nalgebra::{Vector3, Vector2};

pub type Position = Vector3<f64>;
pub type Color = Rgb<f64>;
pub type UVCoords = Vector2<f64>;

pub fn color_to_eight_bit(color: Color) -> Rgb<u8> {
  todo!()
}

pub fn export_to_file(file: &str, image_data: RgbImage) {

}
