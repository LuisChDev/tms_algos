use std::fmt::Debug;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Params {
  pub evaporation_factor: f64,
  pub number_of_ants: u32,
  pub visibility_weight: f64,
  pub pheromone_weight: f64
}
