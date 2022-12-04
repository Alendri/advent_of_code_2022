use std::fs;

fn part1(inp: &str) -> i32 {
  inp
    .lines()
    .filter(|l| {
      let values: Vec<i32> = l
        .trim()
        .split(|c| c == '-' || c == ',')
        .map(|v| v.parse().unwrap())
        .collect();

      match &values[0..4] {
        [a, b, c, d] if a <= c && b >= d => true,
        [a, b, c, d] if a >= c && b <= d => true,
        _ => false,
      }
    })
    .count() as i32
}

fn part2(inp: &str) -> i32 {
  inp
    .lines()
    .filter(|l| {
      let values: Vec<i32> = l
        .trim()
        .split(|c| c == '-' || c == ',')
        .map(|v| v.parse().unwrap())
        .collect();

      match &values[0..4] {
        [a, _, c, d] if a > c && a > d => false,
        [_, b, c, d] if b < c && b < d => false,
        _ => true,
      }
    })
    .count() as i32
}

pub fn exec_part1() -> i32 {
  part1(&fs::read_to_string("./inputs/day04.txt").unwrap())
}
pub fn exec_part2() -> i32 {
  part2(&fs::read_to_string("./inputs/day04.txt").unwrap())
}

#[cfg(test)]
mod tests {
  use super::*;

  const EXAMPLEDATA: &str = "2-4,6-8
  2-3,4-5
  5-7,7-9
  2-8,3-7
  6-6,4-6
  2-6,4-8";

  #[test]
  fn part1_example_data() {
    let result = part1(EXAMPLEDATA);
    assert_eq!(result, 2);
  }
  #[test]
  fn part2_example_data() {
    let result = part2(EXAMPLEDATA);
    assert_eq!(result, 4);
  }
}
