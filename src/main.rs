use camera::Camera;
use image::Rgb;
use materials::Material;
use nalgebra::vector;
use raytracer::{render_scene, scene::Scene};
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
    material: Material { color: Rgb([1.0, 0.0, 0.0]) }
  };

  let camera = Camera {
    eye_position: vector![0.0, 0.0, 0.0],
    viewport: Rectangle {
      top_left: vector![-2.0, 1.0, 1.0],
      top_edge: vector![4.0, 0.0, 0.0],
      left_edge: vector![0.0, 0.0, -2.0],
    },
    pixel_density: 60.0,
  };

  let scene = Scene { camera, shapes: vec![red_sphere]};
  let image = render_scene(scene);
  image.save("results.png").expect("Failed to export image.");
}
