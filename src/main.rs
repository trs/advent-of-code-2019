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
}

fn main() {
  let args: Vec<String> = env::args().collect();

  match args[1].parse::<i32>().unwrap() {
    1 => days::day1::run(),
    2 => days::day2::run(),
    _ => panic!("Expected day as first argument")
  };
}
