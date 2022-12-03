use std::fs;

fn part1(inp: &str) -> i32 {
  inp
    .lines()
    .map(|l| match l.trim() {
      "A X" => 1 + 3, //Rock      Rock      Draw
      "A Y" => 2 + 6, //Rock      Paper     Win
      "A Z" => 3 + 0, //Rock      Scissors  Loss
      "B X" => 1 + 0, //Paper     Rock      Loss
      "B Y" => 2 + 3, //Paper     Paper     Draw
      "B Z" => 3 + 6, //Paper     Scissors  Win
      "C X" => 1 + 6, //Scissors  Rock      Win
      "C Y" => 2 + 0, //Scissors  Paper     Loss
      "C Z" => 3 + 3, //Scissors  Scissors  Draw
      _ => panic!("Unknown input found."),
    })
    .sum()
}
fn part2(inp: &str) -> i32 {
  inp
    .lines()
    .map(|l| match l.trim() {
      "A X" => 3 + 0, //Rock      Scissors  Loss
      "A Y" => 1 + 3, //Rock      Rock      Draw
      "A Z" => 2 + 6, //Rock      Paper     Win
      "B X" => 1 + 0, //Paper     Rock      Loss
      "B Y" => 2 + 3, //Paper     Paper     Draw
      "B Z" => 3 + 6, //Paper     Scissors  Win
      "C X" => 2 + 0, //Scissors  Paper     Loss
      "C Y" => 3 + 3, //Scissors  Scissors  Draw
      "C Z" => 1 + 6, //Scissors  Rock      Win
      _ => panic!("Unknown input found."),
    })
    .sum()
}

pub fn exec_part1() -> i32 {
  part1(&fs::read_to_string("./inputs/day02.txt").unwrap())
}
pub fn exec_part2() -> i32 {
  part2(&fs::read_to_string("./inputs/day02.txt").unwrap())
}

#[cfg(test)]
mod tests {
  use super::*;

  const EXAMPLEDATA: &str = "A Y
  B X
  C Z";

  #[test]
  fn part1_example_data() {
    let result = part1(EXAMPLEDATA);
    assert_eq!(result, 15);
  }
  #[test]
  fn part2_example_data() {
    let result = part2(EXAMPLEDATA);
    assert_eq!(result, 12);
  }
}
