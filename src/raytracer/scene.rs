use image::{ImageBuffer, RgbImage};
use nalgebra::vector;

use crate::{NUM_RAYS_PER_PIXEL, MAX_RAY_BOUNCES, GAMMA_CORRECTION};
use crate::camera::Camera;
use crate::shapes::RenderableShape;
use crate::utils::{Color, color_to_eight_bit, average, gamma_correction};
use super::ray::Ray;

pub struct Scene {
  pub camera: Camera,
  pub objects: Vec<RenderableShape>,
}

impl Scene {
  pub fn render_scene(&self) -> RgbImage {
    ImageBuffer::from_fn(self.camera.pixel_width(), self.camera.pixel_height(),
      |pixel_x, pixel_y| {
        let rays: [Ray; NUM_RAYS_PER_PIXEL] = self.camera.get_pixel_rays(vector![pixel_x, pixel_y]);
        let ray_colors = rays.map(|r| self.get_ray_color(&r, 0));

        let average_color = average(&ray_colors).unwrap_or(vector![0.0, 0.0, 0.0]);
        let gamma_corrected = gamma_correction(average_color, GAMMA_CORRECTION);
        color_to_eight_bit(gamma_corrected)
      })
  }

  fn get_ray_color(&self, ray: &Ray, num_ray_bounces: usize) -> Color {
    if num_ray_bounces > MAX_RAY_BOUNCES { return vector![0.0, 0.0, 0.0]; }

    let all_hits= self.objects.iter().filter_map(|object| object.shape.ray_hits(ray).map(|hit_info| (object, hit_info)));
    let nearest_hit = all_hits.min_by(|h1, h2| h1.1.distance_to_hit.total_cmp(&h2.1.distance_to_hit));

    match nearest_hit {
      None => vector![1.0, ray.direction.y * 0.5 + 1.0, 1.0],
      Some((object, hit_info)) => {
        let scattered_ray = object.material.scatter_ray(ray, &hit_info);
        let scattered_color = self.get_ray_color(&scattered_ray, num_ray_bounces + 1);

        // TODO: more interesting stuff will go in here
        scattered_color
      },
    }
  }
}
