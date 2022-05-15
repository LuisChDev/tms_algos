use rand::prelude::*;

use crate::{solution::{TSP, Solution}, simulated_annealing, ant_colony_optimization};

pub fn basic_tsp() -> TSP {
  let matr = vec![
    vec![0.0, 22.0, 3.0, 15.0],
    vec![4.0, 0.0, 60.0, 17.0],
    vec![7.0, 8.0, 0.0, 125.0],
    vec![71.0, 8.0, 9.0, 0.0],
  ];

  TSP::new(matr, None)
}

pub fn calculating() {
  println!("calculating optimal solution.");

  let matr = vec![
    vec![0.0, 22.0, 3.0, 15.0],
    vec![4.0, 0.0, 60.0, 17.0],
    vec![7.0, 8.0, 0.0, 125.0],
    vec![71.0, 8.0, 9.0, 0.0],
  ];

  let mut rng = thread_rng();
  let mut _large_matrix: Vec<Vec<f64>> = Vec::new();
  for _ in 0..1000 {
    let mut row: Vec<f64> = Vec::new();
    for _ in 0..1000 {
      row.push(rng.gen_range(0.0..1000.0));
    }
    _large_matrix.push(row);
  }

  println!("creating solution model.");
  let mut sol = TSP::new(matr, None);
  let sol2 = sol.clone();

  println!("optimizing using annealing");
  simulated_annealing::anneal(&mut sol);

  println!("optimizing using ant colony optimization.");
  let result = ant_colony_optimization::ant_colony_optimize(&sol2, 0.1);
  println!("optimal result is {:?}", result);
  println!(
    "cost for this result is {:?}",
    TSP::calc_cost(&result, sol2.distances())
  )
}
