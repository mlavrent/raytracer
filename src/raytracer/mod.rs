pub mod ray;
pub mod scene;

use crate::{utils::{Color, color_to_eight_bit}, shapes::Shape};
use image::{RgbImage, ImageBuffer, Rgb};
use nalgebra::vector;
use ray::Ray;
use scene::Scene;

pub fn render_scene(scene: Scene) -> RgbImage {
  println!("Image size: ({0}, {1})", scene.camera.pixel_width(), scene.camera.pixel_height());
  ImageBuffer::from_fn(scene.camera.pixel_width(), scene.camera.pixel_height(),
    |pixel_x, pixel_y| color_to_eight_bit(get_ray_color(scene.camera.get_pixel_ray(vector![pixel_x, pixel_y]), &scene)))
}

pub fn get_ray_color(ray: Ray, scene: &Scene) -> Color {
  let ray_hit = scene.sphere.shape.ray_hits(&ray);

  match ray_hit {
    Some(_) => Rgb([1.0, 0.0, 0.0]),
    None => Rgb([1.0, 1.0, -(ray.direction.into_inner().z - 1.0) / 2.0]),
  }
}
