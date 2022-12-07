use std::{collections::HashSet, fs};

fn compare_4(a: &char, b: &char, c: &char, d: &char) -> bool {
  a != b && a != c && a != d && b != c && b != d && c != d
}

fn part1(inp: &str) -> i32 {
  let chars: Vec<char> = inp.chars().collect();

  for i in 3..chars.len() {
    if compare_4(&chars[i - 3], &chars[i - 2], &chars[i - 1], &chars[i]) {
      return i as i32 + 1;
    }
  }
  panic!("None found.");
}

fn part2(inp: &str) -> i32 {
  let chars: Vec<char> = inp.chars().collect();
  let mut seen: HashSet<char> = HashSet::new();
  seen.reserve(14);

  for i in 0..(chars.len() - 14) {
    let sub = chars.get(i..(i + 14)).unwrap();
    if sub.iter().all(|ch| seen.insert(*ch)) {
      return i as i32 + 14;
    }
    seen.clear();
  }
  panic!("None found.");
}

pub fn exec_part1() -> i32 {
  part1(&fs::read_to_string("./inputs/day06.txt").unwrap())
}
pub fn exec_part2() -> i32 {
  part2(&fs::read_to_string("./inputs/day06.txt").unwrap())
}

#[cfg(test)]
mod tests {
  use super::*;

  const EXAMPLEDATAA: (&str, i32, i32) = ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19);
  const EXAMPLEDATAB: (&str, i32, i32) = ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23);
  const EXAMPLEDATAC: (&str, i32, i32) = ("nppdvjthqldpwncqszvftbrmjlhg", 6, 23);
  const EXAMPLEDATAD: (&str, i32, i32) = ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29);
  const EXAMPLEDATAE: (&str, i32, i32) = ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26);

  #[test]
  fn part1_example_data() {
    let result = part1(EXAMPLEDATAA.0);
    assert_eq!(result, EXAMPLEDATAA.1);
    let result = part1(EXAMPLEDATAB.0);
    assert_eq!(result, EXAMPLEDATAB.1);
    let result = part1(EXAMPLEDATAC.0);
    assert_eq!(result, EXAMPLEDATAC.1);
    let result = part1(EXAMPLEDATAD.0);
    assert_eq!(result, EXAMPLEDATAD.1);
    let result = part1(EXAMPLEDATAE.0);
    assert_eq!(result, EXAMPLEDATAE.1);
  }
  #[test]
  fn part2_example_data() {
    let result = part2(EXAMPLEDATAA.0);
    assert_eq!(result, EXAMPLEDATAA.2);
    let result = part2(EXAMPLEDATAB.0);
    assert_eq!(result, EXAMPLEDATAB.2);
    let result = part2(EXAMPLEDATAC.0);
    assert_eq!(result, EXAMPLEDATAC.2);
    let result = part2(EXAMPLEDATAD.0);
    assert_eq!(result, EXAMPLEDATAD.2);
    let result = part2(EXAMPLEDATAE.0);
    assert_eq!(result, EXAMPLEDATAE.2);
  }
}
