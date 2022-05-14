use log::trace;
use rand::prelude::*;
use rocket::serde::{ Serialize, Deserialize };

pub trait Solution {
  fn new(matrix: Vec<Vec<f64>>, route: Option<Vec<usize>>) -> Self;
  fn perturb(&mut self);
  fn swap(&mut self, first: usize, second: usize);

  fn cost(&self) -> f64;
  fn calc_cost(locations: &Vec<usize>, distances: &Vec<Vec<f64>>) -> f64;

  // getters, behind immutable references
  fn locs(&self) -> &Vec<usize>;
  fn distances(&self) -> &Vec<Vec<f64>>;
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TSP {
  cities: Vec<usize>,
  distances: Vec<Vec<f64>>,
}

impl Solution for TSP {
  fn new(distances: Vec<Vec<f64>>, route: Option<Vec<usize>>) -> TSP {
    match route {
      Some(given_route) => TSP { cities: given_route, distances },
      None => {
        let mut cities = Vec::new();

        // initial solution is just the cities in order
        for city in 0..distances.len() {
          cities.push(city)
        }

        TSP { cities, distances }
      },
    }
  }

  /// leaves the solution in a slightly, randomly changed state.
  /// returns the original position so it can be restored.
  /// can be called with two specific positions instead of generating them.
  fn perturb(&mut self) {
    let mut rng = thread_rng();
    let first = rng.gen_range(0..self.cities.len());
    let second = rng.gen_range(0..self.cities.len());

    self.cities.swap(first, second)
  }

  /// swaps two locations in the cities list.
  fn swap(&mut self, first: usize, second: usize) {
    self.cities.swap(first, second);
  }

  /// returns the current cost of the solution.
  fn calc_cost(locations: &Vec<usize>, distances: &Vec<Vec<f64>>) -> f64 {
    let mut final_cost = 0.0;


    trace!("this is the locations: {:?}", locations);
    let mut current = locations[0];
    let route = &locations[1..];

    for city in route {
      final_cost += distances[current][locations[*city]];
      current = *city;
    }

    let last_leg = distances[current][0];
    final_cost + last_leg
  }

  fn cost(&self) -> f64 {
    Self::calc_cost(&self.cities, &self.distances)
  }

  fn locs(&self) -> &Vec<usize> {
    &self.cities
  }

  fn distances(&self) -> &Vec<Vec<f64>> {
    &self.distances
  }
}
