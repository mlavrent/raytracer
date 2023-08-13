use crate::{utils::Position, raytracer::ray::Ray};
use super::Shape;

pub struct Sphere {
  pub center: Position,
  pub radius: f64,
}

impl Shape for Sphere {
  fn ray_hits(&self, ray: &crate::raytracer::ray::Ray) -> Option<Ray> {
    let center_to_ray_origin = ray.origin - self.center;

    // solve the quadratic equation
    let a = ray.direction.norm_squared();
    let b = 2.0 * (ray.direction.dot(&center_to_ray_origin));
    let c = center_to_ray_origin.dot(&center_to_ray_origin) - self.radius.powi(2);
    let discriminant = b.powi(2) - 4.0 * a * c;

    if discriminant < 0.0 {
      None
    }
    else {
      let distance_to_hit_1 = (-b + discriminant.sqrt()) / (2.0 * a);
      let distance_to_hit_2 = (-b - discriminant.sqrt()) / (2.0 * a);

      let first_hit_distance = if distance_to_hit_1 < 0.0 && distance_to_hit_2 < 0.0 { None }
        else if distance_to_hit_1 < 0.0 { Some(distance_to_hit_2) }
        else if distance_to_hit_2 < 0.0 { Some(distance_to_hit_1) }
        else { Some(f64::min(distance_to_hit_1, distance_to_hit_2)) };

      first_hit_distance.map(|min_distance| {
        let hit_position = ray.at_distance(min_distance);
        let surface_normal = hit_position - self.center;
        Ray::new(hit_position, surface_normal)
      })
    }
  }
}
