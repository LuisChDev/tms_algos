//! This file contains the entrypoint to our service.

// data models
mod params;
mod solution;

// algorithms
mod ant_colony_optimization;
mod simulated_annealing;

// other
mod testing;

use rocket::*;
use rocket::serde::json::Json;
use lambda_web::{is_running_on_lambda, launch_rocket_on_lambda, LambdaError};

use crate::solution::{Solution, TSP};
use crate::testing::basic_tsp;

#[get("/")]
fn index() -> &'static str {
  "Hello, world!"
}

#[get("/test")]
fn test() -> Json<TSP> {
  Json(basic_tsp())
}

#[post("/anneal", format = "json", data = "<problem>")]
fn anneal(problem: rocket::serde::json::Json<TSP>) -> String {
  let parsed: TSP = problem.0;
  format!("the cities, in order, are {:?} and the matrix is {:?}", parsed.locs(), parsed.distances())
}

#[rocket::main]
async fn main() -> Result<(), LambdaError> {
  let rocket = rocket::build().mount("/", routes![index, test, anneal]);
  if is_running_on_lambda() {
    launch_rocket_on_lambda(rocket).await?
  } else {
    rocket.launch().await?
  }
  Ok(())
}

// fn main() -> _ {
//   rocket::ignite().mount("/", routes![index, test, anneal])
// }
