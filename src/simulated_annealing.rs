use rand::prelude::*;

use crate::solution::Solution;


/// tries to find the most efficient solution to the given routing problem with a cost function.
/// this is done by randomly disturbing the existing solution and occasionally keeping a worse result,
/// with the idea that over time, we'll reach the optimal solution.
pub fn anneal<T: Solution>(solution: &mut T) {
  let cooling: f64 = 0.99;
  let mut temperature: f64 = 100.0;
  let mut current_cost: f64 = f64::MAX;

  while temperature > 1.0 {
    // println!("current temp is {}", temperature);
    println!("current cost is {}", current_cost);

    let mut rng = thread_rng();
    let first = rng.gen_range(0..solution.locs().len());
    let second = rng.gen_range(0..solution.locs().len());

    solution.swap(first, second);

    if solution.cost() > current_cost {
      let coin_toss = rng.gen_range(1.0..101.0);
      if coin_toss > temperature {
        solution.swap(first, second);
      }
    }

    current_cost = solution.cost();
    temperature = temperature * cooling;
  }
}
