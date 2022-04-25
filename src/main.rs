mod solution;
mod params;

mod ant_colony_optimization;
mod simulated_annealing;

use rand::prelude::*;

use crate::solution::Solution;


fn main() {
  println!("calculating optimal solution.");

  let _matr = vec![
    vec![1.0, 2.0, 3.0, 15.0],
    vec![4.0, 5.0, 6.0, 17.0],
    vec![7.0, 8.0, 9.0, 12.0],
    vec![7.0, 8.0, 9.0, 9.0]
  ];

  let mut rng = thread_rng();
  let mut large_matrix: Vec<Vec<f64>> = Vec::new();
  for _ in 0..1000 {
    let mut row: Vec<f64> = Vec::new();
    for _ in 0..1000 {
      row.push(rng.gen_range(0.0..1000.0));
    }
    large_matrix.push(row);
  }


  println!("creating solution model.");
  let mut sol = solution::TSP::new(large_matrix, None);
  let sol2 = sol.clone();

  // println!("optimizing using annealing");
  // simulated_annealing::anneal(&mut sol);

  println!("optimizing using ant colony optimization.");
  let result = ant_colony_optimization::ant_colony_optimize(&sol2, 0.1);
  println!("optimal result is {:?}", result);
}
