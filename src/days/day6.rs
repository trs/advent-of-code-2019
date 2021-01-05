use crate::utils::request;

use std::collections::HashMap;

pub fn run() {
  let orbits_input = request::get("https://adventofcode.com/2019/day/6/input").unwrap();

  let mut child_orbit_hash: HashMap<&str, &str> = HashMap::new();

  for line in orbits_input.lines() {
    let orbit = line.split(")").collect::<Vec<&str>>();
    child_orbit_hash.insert(orbit[0], orbit[1]);
  }

  fn count_orbit(child_orbit_hash: &HashMap<&str, &str>, child: &str) -> usize {
    match child_orbit_hash.get(child) {
      Some(&parent) => 1 + count_orbit(&child_orbit_hash, parent),
      None => 0
    }
  };

  let orbit_sum = child_orbit_hash.keys().fold(0, |sum, &child| sum + count_orbit(&child_orbit_hash, child));

  println!("Part 1: {}", orbit_sum);

  // Part 2:
  // Shortest path from YOU to SAN
  // Dijkstra's algorithm?
}
