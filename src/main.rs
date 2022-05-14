#![feature(proc_macro_hygiene, decl_macro)]

//! This file contains the entrypoint to our service.

// data models
mod params;
mod solution;

// algorithms
mod ant_colony_optimization;
mod simulated_annealing;

// other
mod testing;

use rand::prelude::*;
use rocket::*;
use rocket_lamb::RocketExt;
use rocket::serde::json::Json;

use crate::solution::{Solution, TSP};


#[get("/")]
fn index() -> &'static str {
  "Hello, world!"
}

#[get("/test")]
fn testing() -> Json<TSP> {
  Json(basic_tsp())
}

#[post("/anneal", format = "json", data = "<problem>")]
fn anneal(problem: rocket::serde::json::Json<TSP>) -> String {
  let parsed: TSP = problem.0;
  format!("the cities, in order, are {:?} and the matrix is {:?}", parsed.locs(), parsed.distances())
}

fn main() -> _ {
  rocket::ignite().mount("/", routes![index, testing, anneal])
}
