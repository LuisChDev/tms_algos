use rand::prelude::*;
use std::fs;
use toml;
use log::trace;

use crate::params::Params;
use crate::solution::Solution;

/// updates the pheromone readings of the table by a given formula.
/// In addition, pheromone evaporation is carried out across the whole table.
fn update_pherom<T: Solution>(
  params: &Params,
  pheromones: &mut Vec<Vec<f64>>,
  route: &Vec<usize>,
  distances: &Vec<Vec<f64>>,
) {
  pheromones.iter_mut().for_each(|x| {
    for y in x {
      *y = (*y) * params.evaporation_factor * 0.000001;
    }
  });

  trace!("updating pheromone for route {:?}", route);
  let cost = T::calc_cost(route, distances);
  for i in 0..(route.len() - 1) {
    pheromones[route[i]][route[i + 1]] += 1.0 / cost;
  }
  pheromones[route[usize::checked_sub(route.len(), 1).unwrap()]][0] += 1.0 / cost;

  trace!("resulting pheromone is {:?}", pheromones);
}

/**
 * calculate probability of an ant in a given location of visiting any of
 * the yet-to-be-visited locations.
 */
fn calc_prob(
  origin: usize,
  dests: &Vec<usize>,
  distances: &Vec<Vec<f64>>,
  pheromones: &Vec<Vec<f64>>,
  fac_dst: f64,
  fact_frm: f64,
  probs: &mut Vec<f64>,
) {
  // probability of going to each location
  let vals: Vec<f64> = dests
    .iter()
    .map(|&dest| {
      (1.0 / distances[origin][dest]).powf(fac_dst) * (pheromones[origin][dest]).powf(fact_frm)
    })
    .collect();

  // sum of all values
  let total_sum: f64 = vals.iter().sum();

  // probability of each value
  let prob_each: Vec<f64> = vals.iter().map(|val| val / total_sum).collect();

  // the list of probabilities is given as cumulative values.
  let mut cumulative = 0.0;
  for (i, prob) in prob_each.iter().enumerate() {
    cumulative += prob;
    probs.push(cumulative);
    trace!("probs so far: {:?}", probs);
  }
}

/// simulate an ant making the trip through the system following some simple
/// rules.
fn ant<T: Solution>(
  params: &Params,
  solution: &T,
  best_route: &mut Vec<usize>,
  pheromone_matrix: &mut Vec<Vec<f64>>,
  probs: &mut Vec<f64>,
  rem_locs: &mut Vec<usize>,
  cur_loc: &mut usize,
) {
  let mut rand = thread_rng();
  let mut route = Vec::with_capacity(solution.locs().len());
  route.push(solution.locs()[0]);

  // initialize the rem_locs vector w/ all the locations, except the initial one
  // (for now just the first one in the array).
  *rem_locs = solution.locs()[1..].to_vec();

  trace!("{:?}", solution.locs());
  trace!("current list of locations is {:?}", rem_locs);

  while rem_locs.len() > 0 {
    let hit: f64 = rand.gen();

    calc_prob(
      *cur_loc,
      &rem_locs,
      solution.distances(),
      &pheromone_matrix,
      params.visibility_weight,
      params.pheromone_weight,
      probs,
    );

    trace!("calculated probabilities are: {:?}", probs);

    for (i, prob) in probs.iter().enumerate() {
      if hit < *prob {
        route.push(rem_locs[i]);
        rem_locs.swap_remove(i);
        break;
      }
    }

    // clean the array for future use.
    probs.clear();

    trace!("Current route is {:?}", route);
  }

  update_pherom::<T>(params, pheromone_matrix, &route, solution.distances());

  if best_route.len() == 0 || T::calc_cost(&route, solution.distances()) < T::calc_cost(&best_route, solution.distances()) {
    *best_route = route;
  }
}

/**
 * emulates multiple "ants" traveling from origin to destination, following
 * a simple rule: the chance of picking n as the next city to visit is given
 * by it's proximity and a pheromone factor. After completing the track, the
 * picked paths get a pheromone update inversely proportional to the full
 * length of the track. the very first ants will make completely random
 * passages to help set up the board. since we are dividing here, make sure
 * all paths have at least a tiny (but non-zero) amount of pheromone to
 * prevent division-by-zero errors.
 */
pub fn ant_colony_optimize<T: Solution>(solution: &T, init_pherom: f64) -> Vec<usize> {
  // store the current best known route.
  let mut best_route: Vec<usize> = Vec::with_capacity(solution.locs().len());

  // opens configuration file.
  let params: Params = match fs::read_to_string("./params.toml") {
    Ok(conf_str) => match toml::from_str(&conf_str) {
      Ok(conf) => conf,
      Err(err) => panic!("Error while parsing params file: {}", err.to_string()),
    },
    Err(err) => panic!("error while opening params file: {}", err.to_string()),
  };

  // initialize the pheromone matrix with some value.
  let mut pheromone_matrix: Vec<Vec<f64>> = Vec::new();
  for _ in 0..solution.locs().len() {
    let mut pheromone_row = Vec::new();

    for _ in 0..solution.locs().len() {
      pheromone_row.push(init_pherom);
    }

    pheromone_matrix.push(pheromone_row);
  }

  // create and allocate the probabilities vector.
  // max length here would be locations - 1.
  let mut probs: Vec<f64> = Vec::with_capacity(solution.locs().len() - 1);
  let mut rem_locs: Vec<usize> = Vec::with_capacity(solution.locs().len() - 1);
  let mut cur_loc: usize = 0;

  // send an ant to complete the trip. On each location, check where to go to
  // next based on the probability function.
  for _ in 0..params.number_of_ants {
    ant(
      &params,
      solution,
      &mut best_route,
      &mut pheromone_matrix,
      &mut probs,
      &mut rem_locs,
      &mut cur_loc,
    );
  }

  best_route
}
