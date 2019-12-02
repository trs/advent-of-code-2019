use crate::utils;

fn calculate_fuel_amount(fuel: i32) -> i32 {
  (fuel as f32 / 3.0).floor() as i32 - 2
}

fn calculate_fuel_amount_with_mass(fuel: i32, sum: i32) -> i32 {
  let amount = calculate_fuel_amount(fuel);
  if amount <= 0 {
    return sum
  }

  calculate_fuel_amount_with_mass(amount, sum + amount)
}

pub fn run() {
  let modules = utils::request::get("https://adventofcode.com/2019/day/1/input").unwrap();

  // Part 1
  let mut sum = 0;
  for module in &modules {
    let fuel = module.parse::<i32>().unwrap();
    let requirement = calculate_fuel_amount(fuel);
    sum += requirement;
  }
  println!("Part 1: {}", sum);

  // Part 2
  let mut sum = 0;
  for module in &modules {
    let fuel = module.parse::<i32>().unwrap();
    let requirement = calculate_fuel_amount_with_mass(fuel, 0);
    sum += requirement;
  }
  println!("Part 2: {}", sum);
}
