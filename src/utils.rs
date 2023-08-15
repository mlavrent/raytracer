use image::Rgb;
use nalgebra::{Vector3, vector};
use rand::{distributions::WeightedIndex, prelude::Distribution};

// ------- Type aliases -------
pub type Position = Vector3<f64>;
pub type Color = Vector3<f64>;

// ------ Color helper functions -------
pub fn color_to_eight_bit(color: Color) -> Rgb<u8> {
  let eight_bit_color = color.map(channel_to_eight_bit);
  Rgb([eight_bit_color[0], eight_bit_color[1], eight_bit_color[2]])
}

fn channel_to_eight_bit(percentage: f64) -> u8 {
  (percentage * (u8::MAX as f64 - u8::MIN as f64) + u8::MIN as f64) as u8
}

pub fn average(vecs: &[Vector3<f64>]) -> Option<Vector3<f64>> {
  if vecs.len() == 0 {
    None
  }
  else {
    Some(vecs.iter().fold(vector![0.0, 0.0, 0.0], |b, v| b + v) / vecs.len() as f64)
  }
}

// ------- Weighted random sampling from discrete set (enum) -------
pub struct WeightedEnumDistribution<A, const N: usize> {
  distribution: WeightedIndex<f64>,
  weighted_values: [(A, f64); N],
}

impl<A, const N: usize> WeightedEnumDistribution<A, N> {
  pub fn new(weighted_values: [(A, f64); N]) -> WeightedEnumDistribution<A, N> where A : Copy {
    WeightedEnumDistribution {
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

impl<A, const N: usize> Distribution<A> for WeightedEnumDistribution<A, N> where A : Copy {
  fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> A { self.weighted_values[self.distribution.sample(rng)].0 }
}
