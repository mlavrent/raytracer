use nalgebra::vector;
use shapes::sphere::Sphere;

mod shapes;
mod materials;
mod utils;
mod raytracer;
mod camera;

fn main() {
  let sphere = Sphere {
    center: vector![0f64, 0f64, 0f64],
    radius: 2.5,
  };
}
