use nalgebra::{Vector3, Unit, UnitVector3};

use crate::{utils::Position, raytracer::ray::Ray};

use super::{Hittable, HitInfo};

pub struct Triangle {
  vertex_1: Position,
  vertex_2: Position,
  vertex_3: Position,
}

impl Triangle {
  pub fn new(vertices: [Position; 3]) -> Self { Triangle { vertex_1: vertices[0], vertex_2: vertices[1], vertex_3: vertices[2] } }

  pub fn plane_normal(&self) -> UnitVector3<f64> { Unit::new_normalize((self.vertex_2 - self.vertex_1).cross(&(self.vertex_3 - self.vertex_1))) }

  pub fn plane_origin_offset(&self) -> f64 { -self.plane_normal().dot(&self.vertex_1) }

  pub fn point_projects_inside(&self, point: Position) -> bool {
    true // TODO:
  }
}

impl Hittable for Triangle {
  fn ray_hits(&self, ray: &Ray) -> Option<HitInfo> {
    let denominator = self.plane_normal().dot(&ray.direction);

    if denominator == 0.0 {
      None
    } else {
      let distance_to_hit = -(self.plane_normal().dot(&ray.origin) + self.plane_origin_offset()) / denominator;
      let hit_position = ray.at_distance(distance_to_hit);

      (distance_to_hit > 0.0 && self.point_projects_inside(hit_position)).then_some(HitInfo {
        hit_normal: Ray { origin: ray.at_distance(distance_to_hit), direction: self.plane_normal() },
        distance_to_hit,
      })
    }
  }
}
