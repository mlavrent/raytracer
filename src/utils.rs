use std::f64::consts::PI;

use image::Rgb;
use nalgebra::{Vector3, vector, UnitVector3};
use rand::{distributions::WeightedIndex, prelude::Distribution};

// ------- Type aliases -------
pub type Position = Vector3<f64>;
pub type Color = Vector3<f64>;

// ------ Color helper functions -------
pub fn color_to_eight_bit(color: Color) -> Rgb<u8> {
  let eight_bit_color = color.map(channel_to_eight_bit);
  Rgb([eight_bit_color[0], eight_bit_color[1], eight_bit_color[2]])
}

fn channel_to_eight_bit(channel: f64) -> u8 {
  (channel * (u8::MAX as f64 - u8::MIN as f64) + u8::MIN as f64) as u8
}

pub fn gamma_correction(color: Color, gamma: f64) -> Color {
  color.map(|channel| channel.powf(1.0 / gamma))
}

pub fn average(vecs: &[Vector3<f64>]) -> Option<Vector3<f64>> {
  (vecs.len() != 0).then_some(
    vecs.iter().fold(vector![0.0, 0.0, 0.0], |b, v| b + v) / vecs.len() as f64)
}

// ------ Geometry helper functions -------
pub fn deg_to_rad(rad: f64) -> f64 { rad * PI / 180.0 }

pub fn rad_to_deg(deg: f64) -> f64 { deg * 180.0 / PI }

// ------- Weighted random sampling from discrete set (enum) -------
pub struct DiscreteDistribution<A, const N: usize> {
  distribution: WeightedIndex<f64>,
  weighted_values: [(A, f64); N],
}

impl<A, const N: usize> DiscreteDistribution<A, N> {
  pub fn new(weighted_values: [(A, f64); N]) -> DiscreteDistribution<A, N> where A : Copy {
    DiscreteDistribution {
      distribution: WeightedIndex::new(weighted_values.map(|(_, w)| w)).unwrap(),
      weighted_values,
    }
  }

  pub fn get_weight_for_value(&self, value: &A) -> f64 where A: PartialEq {
    match self.weighted_values.iter().find(|&v| v.0 == *value) {
      Some(&(_, w)) => w,
      None => 0.0,
    }
  }
}

impl<A, const N: usize> Distribution<A> for DiscreteDistribution<A, N> where A : Copy {
  fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> A { self.weighted_values[self.distribution.sample(rng)].0 }
}

// ------- Lambertian distribution -------
pub struct LambertianDistribution {
  pub point: Position,
  pub normal: UnitVector3<f64>,
}

impl Distribution<Vector3<f64>> for LambertianDistribution {
  fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Position {
    let sphere_center = self.point + self.normal.into_inner();

    let random_x = rng.gen::<f64>() * 2.0 - 1.0; // TODO: refactor this to use the UniformSphereDistribution
    let random_y = rng.gen::<f64>() * 2.0 - 1.0;
    let offset = vector![random_x, random_y, 1.0 - (random_x.powi(2) + random_y.powi(2)).sqrt()];

    (sphere_center + offset) - self.point
  }
}

// ------- Uniform distribution on sphere -------
pub struct UniformSphereDistribution {
  pub radius: f64,
}

impl Distribution<Vector3<f64>> for UniformSphereDistribution {
  fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Vector3<f64> {
    let random_x = rng.gen::<f64>() * 2.0 - 1.0;
    let random_y = rng.gen::<f64>() * 2.0 - 1.0;

    self.radius * vector![random_x, random_y, 1.0 - (random_x.powi(2) + random_y.powi(2)).sqrt()]
  }
}
