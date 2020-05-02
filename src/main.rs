#[macro_use]
extern crate dotenv_codegen;

use std::env;

mod utils {
  pub mod request;
  pub mod lines;
}

mod days {
  pub mod day1;
  pub mod day2;
  pub mod day3;
  pub mod day4;
}

fn main() {
  let args: Vec<String> = env::args().collect();

  match args[1].parse::<i32>().expect(&format!("Expected a number, got {}", args[1])) {
    1 => days::day1::run(),
    2 => days::day2::run(),
    3 => days::day3::run(),
    4 => days::day4::run(),
    _ => panic!("Expected day as first argument")
  };
}
