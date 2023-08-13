use image::Rgb;
use crate::materials::Material;
use crate::raytracer::ray::Ray;
use crate::raytracer::scene::Scene;
use crate::utils::Color;

pub mod sphere;
pub mod rectangle;

pub trait Hittable {
  /// Returns the ray whose origin is the hit point and whose direction
  /// is the surface normal vector at the hit point, if the shape was
  /// hit by the provided ray.
  fn ray_hits(&self, ray: &Ray) -> Option<Ray>;
}

pub trait Renderable {
  fn get_ray_color(&self, ray: &Ray, scene: &Scene) -> Color;
}

pub struct RenderableShape<S> {
  pub shape: S,
  pub material: Material,
}

impl<S> Renderable for RenderableShape<S> where S : Hittable {
  fn get_ray_color(&self, ray: &Ray, scene: &Scene) -> Color {
    let ray_hit = self.shape.ray_hits(&ray);

    match ray_hit {
      Some(hit_normal) => Rgb([(1.0 + hit_normal.direction.z) / 2.0, 0.0, 0.0]),
      None => Rgb([0.0, 0.0, 0.0]),
    }
  }
}
