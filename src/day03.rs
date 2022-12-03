use std::{
  collections::{HashMap, HashSet},
  fs,
};

fn priority_from_char(ch: char) -> u8 {
  if ch.is_lowercase() {
    ch as u8 - 96
  } else {
    ch as u8 - 64 + 26
  }
}

fn part1(inp: &str) -> i32 {
  inp
    .lines()
    .map(|l| {
      let chrs: Vec<(usize, char)> = l.trim().chars().enumerate().collect();
      let mut seen: HashSet<char> = HashSet::new();
      let half = chrs.len() / 2;
      for (i, ch) in chrs {
        if i < half {
          seen.insert(ch);
        } else if seen.contains(&ch) {
          return priority_from_char(ch) as i32;
        }
      }
      panic!("No error found in backpack.");
    })
    .sum()
}

fn part2(inp: &str) -> i32 {
  let chars_by_line: Vec<Vec<char>> = inp.lines().map(|l| l.trim().chars().collect()).collect();
  let mut sum = 0;
  for i in (0..chars_by_line.len()).step_by(3) {
    let mut track: HashMap<char, i32> = HashMap::new();

    for ch in &chars_by_line[i] {
      track.insert(*ch, 2);
    }

    let mut seen: HashSet<char> = HashSet::new();
    for ch in &chars_by_line[i + 1] {
      if seen.insert(*ch) {
        if let Some(k) = track.get_mut(&ch) {
          *k -= 1;
        }
      }
    }
    for ch in &chars_by_line[i + 2] {
      if track.get(&ch).unwrap_or(&0) == &1i32 {
        sum += priority_from_char(*ch) as i32;
        break;
      }
    }
  }

  sum
}

pub fn exec_part1() -> i32 {
  part1(&fs::read_to_string("./inputs/day03.txt").unwrap())
}
pub fn exec_part2() -> i32 {
  part2(&fs::read_to_string("./inputs/day03.txt").unwrap())
}

#[cfg(test)]
mod tests {
  use super::*;

  const EXAMPLEDATA: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
  jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
  PmmdzqPrVvPwwTWBwg
  wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
  ttgJtRGJQctTZtZT
  CrZsJsPPZsGzwwsLwLmpwMDw";

  #[test]
  fn part1_example_data() {
    let result = part1(EXAMPLEDATA);
    assert_eq!(result, 157);
  }
  #[test]
  fn part2_example_data() {
    let result = part2(EXAMPLEDATA);
    assert_eq!(result, 70);
  }
}
