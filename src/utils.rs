use nalgebra::{Vector3, DMatrix, Vector2};

pub type Position = Vector3<f64>;
pub type Color = Vector3<f64>;
pub type UVCoords = Vector2<f64>;

pub fn export_to_file(file: &str, image_data: DMatrix<Color>) {

}
