pub mod ray;
pub mod scene;

use crate::utils::color_to_eight_bit;
use image::{RgbImage, ImageBuffer};
use nalgebra::vector;
use scene::Scene;

pub fn render_scene(scene: &Scene) -> RgbImage {
  println!("Image size: ({0}, {1})", scene.camera.pixel_width(), scene.camera.pixel_height());
  ImageBuffer::from_fn(scene.camera.pixel_width(), scene.camera.pixel_height(),
    |pixel_x, pixel_y| {
      let ray = scene.camera.get_pixel_ray(vector![pixel_x, pixel_y]);
      let pixel_color_normalized = scene.get_ray_color(&ray);
      color_to_eight_bit(pixel_color_normalized)
    })
}
