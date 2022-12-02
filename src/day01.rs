use std::fs;

fn part1(inp: &str) -> i32 {
  let mut values: Vec<i32> = vec![];
  let mut value = 0;
  inp.lines().into_iter().for_each(|l| {
    if let Ok(v) = l.parse::<i32>() {
      value += v;
      return;
    }
    values.push(value);
    value = 0;
  });
  values.push(value);

  values.sort();
  values.last().unwrap().clone()
}
fn part2(inp: &str) -> i32 {
  let mut values: Vec<i32> = vec![];
  let mut value = 0;
  inp.lines().into_iter().for_each(|l| {
    if let Ok(v) = l.parse::<i32>() {
      value += v;
      return;
    }
    values.push(value);
    value = 0;
  });
  values.push(value);

  values.sort();
  values.split_at(values.len() - 3).1.iter().sum()
}

pub fn exec_part1() -> i32 {
  part1(&fs::read_to_string("./inputs/day01.txt").unwrap())
}
pub fn exec_part2() -> i32 {
  part2(&fs::read_to_string("./inputs/day01.txt").unwrap())
}

#[cfg(test)]
mod tests {
  use super::*;

  const EXAMPLEDATA: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

  #[test]
  fn part1_works() {
    let result = part1(EXAMPLEDATA);
    assert_eq!(result, 24000);
  }
  #[test]
  fn part2_works() {
    let result = part2(EXAMPLEDATA);
    assert_eq!(result, 45000);
  }
}
