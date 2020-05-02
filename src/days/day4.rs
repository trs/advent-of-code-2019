use std::collections::HashMap;

pub fn run() {
  let input = "231832-767346";

  let range = input.split('-').map(|i| i.parse::<i32>().unwrap()).collect::<Vec<i32>>();
  let min = range[0];
  let max = range[1];

  let mut valid_passwords = 0;

  for value in min..max {
    if has_adjacent_digits(value) && digits_dont_decrease(value)   {
      valid_passwords += 1;
    }
  }

  println!("Part 1: {}", valid_passwords);

  valid_passwords = 0;

  for value in min..max {
    if has_adjacent_digits(value) && digits_dont_decrease(value) && has_two_adjacent_digits(value)  {
      valid_passwords += 1;
    }
  }

  println!("Part 2: {}", valid_passwords);
}

fn get_digits(mut input: i32) -> Vec<i32> {
  let mut digits: Vec<i32> = vec!();

  while input > 9 {
    digits.push(input % 10);
    input /= 10;
  }
  digits.push(input);
  digits.reverse();

  digits
}

#[test]
fn test_get_digits() {
  assert_eq!(get_digits(1234), vec!(1, 2, 3, 4));
}

fn has_adjacent_digits(input: i32) -> bool {
  let digits = get_digits(input);
  let mut last_digit: Option<i32> = None;

  for digit in digits {
    if last_digit.is_some() {
      if last_digit.unwrap() == digit {
        return true;
      }
    }
    last_digit = Some(digit);
  }

  false
}

fn has_two_adjacent_digits(input: i32) -> bool {
  let mut adjacent_digits: HashMap<i32, i32> = HashMap::new();

  let digits = get_digits(input);
  for digit in digits {
    adjacent_digits.insert(digit, adjacent_digits.get(&digit).unwrap_or(&0) + 1);
  }

  for count in adjacent_digits.values() {
    if *count == 2 {
      return true;
    }
  }

  false
}

#[test]
fn test_has_two_adjacent_digits() {
  assert_eq!(has_two_adjacent_digits(112233), true);
  assert_eq!(has_two_adjacent_digits(123444), false);
  assert_eq!(has_two_adjacent_digits(111122), true);
}

fn digits_dont_decrease(input: i32) -> bool {
  let digits = get_digits(input);
  let mut last_digit: Option<i32> = None;

  for digit in digits {
    if last_digit.is_some() {
      if digit < last_digit.unwrap() {
        return false;
      }
    }
    last_digit = Some(digit);
  }

  true
}

#[test]
fn test_digits_increase() {
  assert_eq!(digits_dont_decrease(1234), true);
  assert_eq!(digits_dont_decrease(1111), true);
  assert_eq!(digits_dont_decrease(1231), false);
}
