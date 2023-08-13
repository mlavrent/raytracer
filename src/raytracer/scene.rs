use image::{Rgb, ImageBuffer, RgbImage};
use nalgebra::vector;

use crate::NUM_RAYS_PER_PIXEL;
use crate::camera::Camera;
use crate::shapes::RenderableShape;
use crate::utils::{Color, color_to_eight_bit, average_color};
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
        let ray_colors = rays.map(|r| self.get_ray_color(&r));

        let average_color = average_color(&ray_colors);
        color_to_eight_bit(average_color)
      })
  }

  fn get_ray_color(&self, ray: &Ray) -> Color {
    let all_hits= self.objects.iter().filter_map(|object| object.shape.ray_hits(ray).map(|hit_info| (object, hit_info)));
    let nearest_hit = all_hits.min_by(|h1, h2| h1.1.distance_to_hit.total_cmp(&h2.1.distance_to_hit));

    match nearest_hit {
      None => Rgb([0.0, 0.0, 0.0]),
      Some((_, hit_info)) => {
        // TODO: more interesting stuff will go in here
        Rgb([(1.0 + hit_info.hit_normal.direction.z) / 2.0, 0.0, 0.0])
      },
    }
  }
}
