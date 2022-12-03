mod day01;
mod day02;

pub fn main() {
  println!("Advent of code 2022; rust training!");
  println!("\n- Day 01 - -");
  println!("  P1 answer: {}", day01::exec_part1());
  println!("  P2 answer: {}", day01::exec_part2());
  println!("\n- Day 02 - -");
  println!("  P1 answer: {}", day02::exec_part1());
  println!("  P2 answer: {}", day02::exec_part2());
}
