use camera::Camera;
use materials::Material;
use nalgebra::vector;
use shapes::{sphere::Sphere, rectangle::Rectangle, RenderableShape};

mod shapes;
mod materials;
mod utils;
mod raytracer;
mod camera;

fn main() {
  let red_sphere = RenderableShape {
    shape: Sphere {
      center: vector![0f64, 0f64, 0f64],
      radius: 2.5,
    },
    material: Material { color: vector![0.5, 0.4, 0.7] }
  };

  let camera = Camera {
    eye_position: vector![],
    viewport: Rectangle {
      top_left: vector![],
      top_edge: vector![],
      left_edge: vector![],
    }
  };
}
