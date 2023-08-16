use crate::shapes::HitInfo;
use crate::raytracer::ray::Ray;
use super::{Material, ScatterInfo};


pub struct RefractiveMaterial {

}

impl Material for RefractiveMaterial {
  fn scatter_ray(&self, in_ray: &Ray, hit_info: &HitInfo) -> ScatterInfo {
    ScatterInfo {
      scattered_ray: Ray::new(hit_info.hit_normal.origin, todo!()),
      attenuation: todo!(),
    }
  }
}
