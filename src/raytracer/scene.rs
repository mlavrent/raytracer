use image::Rgb;

use crate::camera::Camera;
use crate::shapes::RenderableShape;
use crate::utils::Color;

use super::ray::Ray;

pub struct Scene {
  pub camera: Camera,
  pub objects: Vec<RenderableShape>,
}

impl Scene {
  pub fn get_ray_color(&self, ray: &Ray) -> Color {
    let all_hits= self.objects.iter().filter_map(|object| object.shape.ray_hits(ray).map(|hit_info| (object, hit_info)));
    let nearest_hit = all_hits.min_by(|h1, h2| h1.1.distance_to_hit.total_cmp(&h2.1.distance_to_hit));

    match nearest_hit {
      Some((_, hit_info)) => Rgb([(1.0 + hit_info.hit_normal.direction.z) / 2.0, 0.0, 0.0]),
      None => Rgb([0.0, 0.0, 0.0]),
    }
  }
}
