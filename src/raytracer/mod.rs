pub mod ray;
pub mod scene;

use crate::utils::Color;
use image::ImageBuffer;
use nalgebra::{DMatrix, vector};
use ray::Ray;
use scene::Scene;

pub fn render_scene(scene: Scene) -> DMatrix<> {
  scene.camera.pixel_points().map(|p| get_ray_color(scene.camera.pixel_to_ray(p), &scene))
}

pub fn get_ray_color(ray: Ray, _: &Scene) -> Color {
  // just return the z-height of the ray as red, this should result in a gradient
  vector![*ray.direction.into_inner().index((0, 1)), 0.0, 0.0]
}
