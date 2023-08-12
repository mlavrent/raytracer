use camera::Camera;
use materials::Material;
use nalgebra::vector;
use raytracer::render_scene;
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
    eye_position: vector![0.0, 0.0, 0.0],
    viewport: Rectangle {
      top_left: vector![-2.0, 3.0, 1.5],
      top_edge: vector![4.0, 0.0, 0.0],
      left_edge: vector![0.0, 0.0, -3.0],
    },
    pixel_density: 600.0,
  };

  let image = render_scene(camera, vec![red_sphere]);
}
