use crate::shapes::HitInfo;
use crate::raytracer::ray::Ray;
use super::Material;


pub struct RefractiveMaterial {

}

impl Material for RefractiveMaterial {
  fn scatter_ray(&self, in_ray: &Ray, hit_info: &HitInfo) -> Ray {
    Ray::new(hit_info.hit_normal.origin, todo!())
  }
}
