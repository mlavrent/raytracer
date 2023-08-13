use camera::Camera;
use enum_map::enum_map;
use materials::{Material, ScatterType::*};
use nalgebra::vector;
use raytracer::scene::Scene;
use shapes::{sphere::Sphere, rectangle::Rectangle, RenderableShape};

mod shapes;
mod materials;
mod utils;
mod raytracer;
mod camera;

// constants that configure the ray tracer
const NUM_RAYS_PER_PIXEL: usize = 20;
const MAX_RAY_BOUNCES: usize = 10;

fn main() {
  let small_sphere = RenderableShape {
    shape: Box::new(Sphere {
      center: vector![0.0, 1.0, 0.0],
      radius: 0.5,
    }),
    material: Material { color: vector![1.0, 0.0, 0.0], scatter_percentages: enum_map! { Diffuse => 1.0, _ => 0.0 } }
  };
  let big_sphere = RenderableShape {
    shape: Box::new(Sphere {
      center: vector![0.0, 1.0, -100.5],
      radius: 100.0,
    }),
    material: Material { color: vector![0.0, 1.0, 0.0], scatter_percentages: enum_map! { Diffuse => 1.0, _ => 0.0 } }
  };

  let camera = Camera {
    eye_position: vector![0.0, 0.0, 0.0],
    viewport: Rectangle {
      top_left: vector![-2.0, 1.0, 1.0],
      top_edge: vector![4.0, 0.0, 0.0],
      left_edge: vector![0.0, 0.0, -2.0],
    },
    pixel_density: 150.0,
  };

  let scene = Scene { camera, objects: vec![small_sphere, big_sphere] };
  let image = scene.render_scene();
  image.save("results.png").expect("Failed to export image.");
}
