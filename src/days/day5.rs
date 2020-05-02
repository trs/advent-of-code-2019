use crate::utils::request;

#[derive(PartialEq, Debug)]
enum OpCode {
  Add = 1,
  Multiply = 2,
  Input = 3,
  Output = 4,
  JumpIfTrue = 5,
  JumpIfFalse = 6,
  LessThan = 7,
  Equals = 8,
  End = 99
}

impl From<i32> for OpCode {
  fn from(value: i32) -> Self {
    match value {
      1 => OpCode::Add,
      2 => OpCode::Multiply,
      3 => OpCode::Input,
      4 => OpCode::Output,
      5 => OpCode::JumpIfTrue,
      6 => OpCode::JumpIfFalse,
      7 => OpCode::LessThan,
      8 => OpCode::Equals,
      99 => OpCode::End,
      _ => panic!("Invalid OpCode {}", value)
    }
  }
}


#[derive(PartialEq, Debug)]
enum ParameterMode {
  Position = 0,
  Immediate = 1
}

impl From<i32> for ParameterMode {
  fn from(value: i32) -> Self {
    match value {
      0 => ParameterMode::Position,
      1 => ParameterMode::Immediate,
      _ => panic!("Invalid ParameterMode {}", value)
    }
  }
}

#[derive(Debug)]
struct Instruction {
  op_code: OpCode,
  mode_1: ParameterMode,
  mode_2: ParameterMode,
  mode_3: ParameterMode
}

fn parse_instruction(instruction: i32) -> Instruction {
  let op_code = OpCode::from(instruction % 100);
  let modes = instruction / 100;

  let mode_1 = ParameterMode::from(modes % 10);
  let mode_2 = ParameterMode::from(modes / 10 % 10);
  let mode_3 = ParameterMode::from(modes / 100 % 10);

  Instruction {
    op_code,
    mode_1,
    mode_2,
    mode_3
  }
}

#[test]
fn test_parse_instruction() {
  let instruction = parse_instruction(10102);
  assert_eq!(instruction.op_code, OpCode::Multiply);
  assert_eq!(instruction.mode_1, ParameterMode::Immediate);
  assert_eq!(instruction.mode_2, ParameterMode::Position);
  assert_eq!(instruction.mode_3, ParameterMode::Immediate);
}

fn resolve_value(codes: &Vec<i32>, value: i32, mode: &ParameterMode) -> i32 {
  let resolved = match mode {
    ParameterMode::Position => codes[value as usize],
    ParameterMode::Immediate => value
  };

  resolved
}

fn run_instructions(mut codes: Vec<i32>, input: i32) {
  let mut pointer: usize = 0;
  while pointer < codes.len() {
    let instruction = parse_instruction(codes[pointer]);

    let inc = match instruction.op_code {
      OpCode::Add => {
        let val_1 = resolve_value(&codes, codes[pointer + 1], &instruction.mode_1);
        let val_2 = resolve_value(&codes, codes[pointer + 2], &instruction.mode_2);
        let into = codes[pointer + 3];

        codes[into as usize] = val_1 + val_2;

        4
      },
      OpCode::Multiply => {
        let val_1 = resolve_value(&codes, codes[pointer + 1], &instruction.mode_1);
        let val_2 = resolve_value(&codes, codes[pointer + 2], &instruction.mode_2);
        let into = codes[pointer + 3];

        codes[into as usize] = val_1 * val_2;

        4
      },
      OpCode::Input => {
        let result = codes[pointer + 1];
        codes[result as usize] = input;

        2
      },
      OpCode::Output => {
        let result = codes[pointer + 1];
        let output = codes[result as usize];

        println!("[!] Output: {}", output);

        2
      },
      OpCode::JumpIfTrue => {
        let val_1 = resolve_value(&codes, codes[pointer + 1], &instruction.mode_1);
        let val_2 = resolve_value(&codes, codes[pointer + 2], &instruction.mode_2);

        if val_1 != 0 {
          pointer = val_2 as usize;
          0
        } else {
          3
        }
      },
      OpCode::JumpIfFalse => {
        let val_1 = resolve_value(&codes, codes[pointer + 1], &instruction.mode_1);
        let val_2 = resolve_value(&codes, codes[pointer + 2], &instruction.mode_2);

        if val_1 == 0 {
          pointer = val_2 as usize;
          0
        } else {
          3
        }
      },
      OpCode::LessThan => {
        let val_1 = resolve_value(&codes, codes[pointer + 1], &instruction.mode_1);
        let val_2 = resolve_value(&codes, codes[pointer + 2], &instruction.mode_2);
        // let into = resolve_value(&codes, codes[pointer + 3], &instruction.mode_3);
        let into = codes[pointer + 3];

        codes[into as usize] = if val_1 < val_2 { 1 } else { 0 };

        4
      },
      OpCode::Equals => {
        let val_1 = resolve_value(&codes, codes[pointer + 1], &instruction.mode_1);
        let val_2 = resolve_value(&codes, codes[pointer + 2], &instruction.mode_2);
        // let into = resolve_value(&codes, codes[pointer + 3], &instruction.mode_3);
        let into = codes[pointer + 3];

        codes[into as usize] = if val_1 == val_2 { 1 } else { 0 };

        4
      },
      OpCode::End => break,
      // _ => panic!("Unsupported OpCode: {:?}", instruction.op_code)
    };

    pointer += inc;
  }
}

pub fn run() {
  let text = request::get("https://adventofcode.com/2019/day/5/input").unwrap();
  let codes: Vec<i32> = text.split(",").map(|s| s.parse::<i32>().unwrap()).collect();

  println!("Part 1");

  run_instructions(codes.clone(), 1);

  println!("Part 2");

  run_instructions(codes.clone(), 5);
}
