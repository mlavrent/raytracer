use crate::utils::Position;
use super::Shape;

pub struct Sphere {
  pub center: Position,
  pub radius: f64,
}

impl Shape for Sphere {
  fn ray_hits(&self, ray: &crate::raytracer::ray::Ray) -> Option<Position> {
    let ray_origin_to_center = self.center - ray.origin;

    // solve the quadratic equation
    let a = ray.direction.norm_squared();
    let b = 2.0 * (ray.direction.dot(&ray_origin_to_center));
    let c = ray_origin_to_center.dot(&ray_origin_to_center) - self.radius.powi(2);
    let discriminant = b.powi(2) - 4.0 * a * c;

    if discriminant < 0.0 {
      None
    }
    else {
      let distance_to_hit_1 = (-b + discriminant.sqrt()) / (2.0 * a);
      let distance_to_hit_2 = (-b - discriminant.sqrt()) / (2.0 * a);
      let min_distance = f64::min(distance_to_hit_1, distance_to_hit_2);
      let hit_position = ray.at_distance(min_distance);

      Some(hit_position)
    }
  }
}
