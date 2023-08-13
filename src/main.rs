use camera::Camera;
use image::Rgb;
use materials::Material;
use nalgebra::vector;
use raytracer::scene::Scene;
use shapes::{sphere::Sphere, rectangle::Rectangle, RenderableShape};

mod shapes;
mod materials;
mod utils;
mod raytracer;
mod camera;

// constants that configure the ray tracer
const NUM_RAYS_PER_PIXEL: usize = 3;

fn main() {
  let red_sphere = RenderableShape {
    shape: Box::new(Sphere {
      center: vector![0.0, 5.0, 0.0],
      radius: 2.5,
    }),
    material: Material { color: Rgb([1.0, 0.0, 0.0]) }
  };

  let camera = Camera {
    eye_position: vector![0.0, 0.0, 0.0],
    viewport: Rectangle {
      top_left: vector![-2.0, 1.0, 1.0],
      top_edge: vector![4.0, 0.0, 0.0],
      left_edge: vector![0.0, 0.0, -2.0],
    },
    pixel_density: 120.0,
  };

  let scene = Scene { camera, objects: vec![red_sphere] };
  let image = scene.render_scene();
  image.save("results.png").expect("Failed to export image.");
}
