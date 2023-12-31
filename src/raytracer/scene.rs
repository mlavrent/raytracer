use image::{ImageBuffer, RgbImage, Rgb, Pixel};
use itertools::Itertools;
use nalgebra::vector;
use rayon::prelude::{ParallelIterator, IntoParallelRefIterator};

use crate::{NUM_RAYS_PER_PIXEL, MAX_RAY_BOUNCES, GAMMA_CORRECTION};
use crate::camera::Camera;
use crate::shapes::RenderableShape;
use crate::utils::{Color, color_to_eight_bit, average, gamma_correction};
use super::ray::Ray;

pub struct Scene<'a> {
  pub camera: Camera,
  pub objects: Vec<RenderableShape<'a>>,
  pub background_color: Color,
}

impl<'a> Scene<'a> {
  pub fn render_scene(&self) -> RgbImage {
    let pixels = (0..self.camera.pixel_height()).cartesian_product(0..self.camera.pixel_width()).collect_vec();
    let colors = pixels.par_iter().map(|(pixel_y, pixel_x)| self.get_pixel_color(*pixel_x, *pixel_y));
    let image_buffer = colors.flat_map(|rgb| rgb.channels().to_vec());

    ImageBuffer::from_vec(self.camera.pixel_width(), self.camera.pixel_height(), image_buffer.collect()).unwrap()
  }

  fn get_pixel_color(&self, pixel_x: u32, pixel_y: u32) -> Rgb<u8> {
    let rays: [Ray; NUM_RAYS_PER_PIXEL] = self.camera.get_pixel_rays(vector![pixel_x, pixel_y]);
    let ray_colors = rays.map(|r| self.get_ray_color(&r, 0));

    let average_color = average(&ray_colors).unwrap_or(vector![0.0, 0.0, 0.0]);
    let gamma_corrected = gamma_correction(average_color, GAMMA_CORRECTION);
    color_to_eight_bit(gamma_corrected)
  }

  fn get_ray_color(&self, ray: &Ray, num_ray_bounces: usize) -> Color {
    if num_ray_bounces > MAX_RAY_BOUNCES { return self.background_color; }

    let all_hits = self.objects.iter()
      .filter_map(|object| object.shape.ray_hits(ray).map(|hit_info| (object, hit_info)))
      .filter(|(_, hit_info)| hit_info.distance_to_hit > 1e-7);
    let nearest_hit = all_hits.min_by(|h1, h2| h1.1.distance_to_hit.total_cmp(&h2.1.distance_to_hit));

    match nearest_hit {
      None => self.background_color,
      Some((object, hit_info)) => {
        let scatter_info = object.material.scatter_ray(ray, &hit_info);
        match scatter_info.scattered_ray {
          Some(scattered_ray) => self.get_ray_color(&scattered_ray, num_ray_bounces + 1).component_mul(&scatter_info.attenuation),
          None => scatter_info.attenuation,
        }
      },
    }
  }
}
