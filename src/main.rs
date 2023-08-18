use camera::Camera;
use materials::{diffuse::DiffuseMaterial, specular::ReflectiveMaterial, refractive::RefractiveMaterial, emitter::EmitterMaterial};
use nalgebra::vector;
use raytracer::scene::Scene;
use shapes::{sphere::Sphere, rectangle::Rectangle, RenderableShape};

mod shapes;
mod materials;
mod utils;
mod raytracer;
mod camera;

// constants that configure the ray tracer
const NUM_RAYS_PER_PIXEL: usize = 500;
const MAX_RAY_BOUNCES: usize = 6;
const GAMMA_CORRECTION: f64 = 2.0;

fn main() {
  let ground_material = DiffuseMaterial { color: vector![0.46, 0.49, 0.54] };
  let diffuse_material = DiffuseMaterial { color: vector![0.7, 0.2, 0.7] };
  let mirror_material = ReflectiveMaterial::new(vector![1.0, 0.8, 0.8], 0.2);
  let glass_material = RefractiveMaterial { refraction_index: 1.5, color: vector![1.0, 1.0, 1.0] };
  let sun_material = EmitterMaterial { color: 5.0 * vector![1.0, 1.0, 1.0] };

  let ground_sphere = RenderableShape {
    shape: &Sphere {
      center: vector![0.0, 1.0, -100.5],
      radius: 100.0,
    },
    material: &ground_material,
  };
  let center_sphere = RenderableShape {
    shape: &Sphere {
      center: vector![-1.7, 5.0, 0.6],
      radius: 0.4,
    },
    material: &diffuse_material,
  };
  let left_sphere = RenderableShape {
    shape: &Sphere {
      center: vector![-1.25, 1.0, 0.0],
      radius: 0.8,
    },
    material: &glass_material,
  };
  let right_sphere = RenderableShape {
    shape: &Sphere {
      center: vector![1.25, 1.0, 0.0],
      radius: 0.5,
    },
    material: &mirror_material,
  };

  // let camera = Camera {
  //   eye_position: vector![0.0, -4.0, 0.0],
  //   viewport: Rectangle {
  //     top_left: vector![-2.0, 1.0, 1.0],
  //     top_edge: vector![4.0, 0.0, 0.0],
  //     left_edge: vector![0.0, 0.0, -2.0],
  //   },
  //   pixel_density: 250.0,
  // };
  let camera = Camera::new(vector![-2.0, -6.0, 3.0], vector![0.0, 1.0, 0.0], 60.0, 35.0, 800);

  let scene = Scene {
    camera,
    objects: vec![center_sphere, ground_sphere, left_sphere, right_sphere],
    background_color: 0.6 * vector![1.0, 1.0, 1.0],
  };
  let image = scene.render_scene();
  image.save("results.png").expect("Failed to export image.");
}
