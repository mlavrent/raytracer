use camera::Camera;
use materials::{Material, ScatterType::{*, self}};
use nalgebra::vector;
use raytracer::scene::Scene;
use shapes::{sphere::Sphere, rectangle::Rectangle, RenderableShape};
use utils::DiscreteDistribution;

mod shapes;
mod materials;
mod utils;
mod raytracer;
mod camera;

// constants that configure the ray tracer
const NUM_RAYS_PER_PIXEL: usize = 25;
const MAX_RAY_BOUNCES: usize = 10;
const GAMMA_CORRECTION: f64 = 2.0;

fn main() {
  let ground_sphere = RenderableShape {
    shape: Box::new(Sphere {
      center: vector![0.0, 1.0, -100.5],
      radius: 100.0,
    }),
    material: Material {
      color: vector![0.8, 0.8, 0.0],
      scatter_percentages: DiscreteDistribution::new([(ScatterType::Diffuse, 1.0), (ScatterType::Specular, 0.0), (ScatterType::Refractive(1.0), 0.0)])
    }
  };
  let center_sphere = RenderableShape {
    shape: Box::new(Sphere {
      center: vector![0.0, 1.0, 0.0],
      radius: 0.5,
    }),
    material: Material {
      color: vector![0.7, 0.3, 0.3],
      scatter_percentages: DiscreteDistribution::new([(ScatterType::Diffuse, 0.0), (ScatterType::Specular, 1.0), (ScatterType::Refractive(1.0), 0.0)])
    }
  };
  let left_sphere = RenderableShape {
    shape: Box::new(Sphere {
      center: vector![-1.0, 1.0, 0.0],
      radius: 0.5,
    }),
    material: Material {
      color: vector![0.8, 0.8, 0.8],
      scatter_percentages: DiscreteDistribution::new([(ScatterType::Diffuse, 1.0), (ScatterType::Specular, 0.0), (ScatterType::Refractive(1.0), 0.0)]),
    },
  };
  let right_sphere = RenderableShape {
    shape: Box::new(Sphere {
      center: vector![1.0, 1.0, 0.0],
      radius: 0.5,
    }),
    material: Material {
      color: vector![0.8, 0.6, 0.2],
      scatter_percentages: DiscreteDistribution::new([(ScatterType::Diffuse, 0.0), (ScatterType::Specular, 1.0), (ScatterType::Refractive(1.0), 0.0)]),
    },
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

  let scene = Scene { camera, objects: vec![center_sphere, ground_sphere, left_sphere, right_sphere] };
  let image = scene.render_scene();
  image.save("results.png").expect("Failed to export image.");
}
