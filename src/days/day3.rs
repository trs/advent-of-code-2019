use crate::utils::{request, lines};

use std::fmt;

#[derive(Clone, Copy)]
struct Point {
  x: f32,
  y: f32
}

impl fmt::Debug for Point {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}

#[derive(Clone, Copy)]
struct Line {
  p1: Point,
  p2: Point,
  length: f32
}

impl fmt::Debug for Line {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "[{:?}, {:?}] len: {}", self.p1, self.p2, self.length)
  }
}

pub fn run() {
  // let text = request::get("https://adventofcode.com/2019/day/3/input").unwrap();
  // let wires = lines::parse_lines(text);

  let wires = vec!(
    "R75,D30,R83,U83,L12,D49,R71,U7,L72".to_string(),
    "U62,R66,U55,R34,D71,R55,D58,R83".to_string()
  );

  let origin_point = Point {
    x: 0.0,
    y: 0.0
  };

  let first_lines = get_lines(&wires[0]);
  let second_lines = get_lines(&wires[1]);

  let mut manhattan_distance: Option<f32> = None;
  let mut combined_steps: f32 = 0.0;

  for l1 in &first_lines {
    for l2 in &second_lines {
      let intersection = find_intersections(l1, l2);

      if intersection.is_some() {
        let distance = get_distance(origin_point, intersection.unwrap());

        println!("Distance = {}", distance);

        if manhattan_distance.is_none() {
          manhattan_distance = Some(distance);
        } else {
          manhattan_distance = Some(f32::min(manhattan_distance.unwrap(), distance));
        }
      }
    }
  }

  println!("Part 1: {}", manhattan_distance.unwrap());
}

fn get_lines(moves: &String) -> Vec<Line> {
  let mut lines: Vec<Line> = vec!();

  for mut m in moves.split(",").map(|m| m.to_string()) {
    let direction = m.remove(0);
    let amount = m.parse::<f32>().expect(&format!("Bad digit: {}", m));


    let last_point = lines.last().unwrap_or(&Line {
      p1: Point {x: 0.0, y: 0.0},
      p2: Point { x: 0.0, y: 0.0 },
      length: 0.0
    }).p2;
    let mut next_point = last_point.clone();

    match direction {
      'R' => next_point.x += amount,
      'L' => next_point.x -= amount,
      'U' => next_point.y += amount,
      'D' => next_point.y -= amount,
      n @ _ => {
        panic!("Unknown direction: {}", n);
      }
    }

    lines.push(Line {
      p1: last_point,
      p2: next_point,
      length: get_distance(last_point, next_point)
    });
  }

  // Ignore lines from start (0, 0)
  lines.remove(0);

  lines
}

fn lines_overlap(l1: &Line, l2: &Line) -> bool {
  let a = l1.p1.x;
  let b = l1.p1.y;
  let c = l1.p2.x;
  let d = l1.p2.y;

  let p = l2.p1.x;
  let q = l2.p1.y;
  let r = l2.p2.x;
  let s = l2.p2.y;

  let det = (c - a) * (s - q) - (r - p) * (d - b);

  if det == 0.0 {
    return false;
  }

  let lambda = ((s - q) * (r - a) + (p - r) * (s - b)) / det;
  let gamma = ((b - d) * (r - a) + (c - a) * (s - b)) / det;

  (0.0 < lambda && lambda < 1.0) && (0.0 < gamma && gamma < 1.0)
}

fn find_intersections(l1: &Line, l2: &Line) -> Option<Point> {

  if !lines_overlap(l1, l2) {
    return None;
  }

  let a1 = l1.p2.y - l1.p1.y;
  let b1 = l1.p1.x - l1.p2.x;
  let c1 = (a1 * l1.p1.x) + (b1 * l1.p1.y);

  let a2 = l2.p2.y - l2.p1.y;
  let b2 = l2.p1.x - l2.p2.x;
  let c2 = (a2 * l2.p1.x) + (b2 * l2.p1.y);

  let determinant = (a1 * b2) - (a2 * b1);
  if determinant == 0.0 {
    return None;
  }

  let x = (b2 * c1 - b1 * c2) / determinant;
  let y = (a1 * c2 - a2 * c1) / determinant;

  let intersection = Point {
    x,
    y
  };

  println!("{:?} and {:?} intersect at {:?}", l1, l2, intersection);

  Some(intersection)
}

fn get_distance(p1: Point, p2: Point) -> f32 {
  let a = p1.x.abs() - p2.x.abs();
  let b = p1.y.abs() - p2.y.abs();

  (a + b).abs()
}
