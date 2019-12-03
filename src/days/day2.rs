use crate::utils::request;

fn run_instructions(mut codes: Vec<usize>) -> usize {
  let mut pos = 0;
  while pos < codes.len() {
    let op_code = codes[pos];
    let val1_index = codes[pos + 1];
    let val2_index = codes[pos + 2];
    let result_index = codes[pos + 3];

    let val1 = codes[val1_index];
    let val2 = codes[val2_index];

    match op_code {
      1 => codes[result_index] = val1 + val2,
      2 => codes[result_index] = val1 * val2,
      99 => break,
      _ => panic!(format!("Invalid OP Code: {}", op_code))
    }

    pos += 4;
  }
  codes[0]
}

pub fn run() {
  let text = request::get("https://adventofcode.com/2019/day/2/input").unwrap();
  let codes: Vec<usize> = text.split(",").map(|s| s.parse::<usize>().unwrap()).collect();

  // Part 1
  let mut result = codes.clone();
  result[1] = 12;
  result[2] = 2;

  let ret = run_instructions(result);
  println!("Part 1: {}", ret);

  // Part 2
  let mut noun = 0;
  let mut verb = 0;

  'outer: for n in 0..99 {
    for v in 0..99 {
      noun = n;
      verb = v;

      let mut result = codes.clone();
      result[1] = noun;
      result[2] = verb;

      let ret = run_instructions(result);
      if ret == 19690720 {
        break 'outer;
      }
    }
  }
  let output = 100 * noun + verb;
  println!("Part 2: {}", output);
}
