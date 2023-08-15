use camera::Camera;
use materials::{Material, ScatterType::{*, self}};
use nalgebra::vector;
use raytracer::scene::Scene;
use shapes::{sphere::Sphere, rectangle::Rectangle, RenderableShape};
use utils::WeightedEnumDistribution;

mod shapes;
mod materials;
mod utils;
mod raytracer;
mod camera;

// constants that configure the ray tracer
const NUM_RAYS_PER_PIXEL: usize = 50;
const MAX_RAY_BOUNCES: usize = 10;
const GAMMA_CORRECTION: f64 = 2.0;

fn main() {
  let small_sphere = RenderableShape {
    shape: Box::new(Sphere {
      center: vector![0.0, 1.0, 0.0],
      radius: 0.5,
    }),
    material: Material {
      color: vector![0.8, 0.5, 0.5],
      scatter_percentages: WeightedEnumDistribution::new([(ScatterType::Diffuse, 1.0), (ScatterType::Specular, 0.0), (ScatterType::Refractive, 0.0)])
    }
  };
  let big_sphere = RenderableShape {
    shape: Box::new(Sphere {
      center: vector![0.0, 1.0, -100.5],
      radius: 100.0,
    }),
    material: Material {
      color: vector![0.9, 0.9, 0.9],
      scatter_percentages: WeightedEnumDistribution::new([(ScatterType::Diffuse, 1.0), (ScatterType::Specular, 0.0), (ScatterType::Refractive, 0.0)])
    }
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
