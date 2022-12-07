use std::fs;

fn parse_state(inp: &str) -> Vec<Vec<char>> {
  inp
    .lines()
    .filter(|l| l.contains("[")) //Remove lines without data points.
    .map(|l| {
      l.split(|c| c == '[' || c == ']')
        .filter(|v| v.len() > 0) //Remove entries which has zero-length. These are created where split was done.
        .collect::<Vec<&str>>()
    })
    .fold(
      vec![Vec::new()] as Vec<Vec<char>>,
      |mut acc, cols_and_values| {
        let mut col_index = 0;
        cols_and_values.iter().for_each(|v| {
          //A value can be either a single letter or a number of spaces.
          if v.starts_with(' ') {
            //If it is spaces check how many and add columns to the accumulator accordingly.
            let space_count = v.chars().count();
            if space_count > 1 {
              let reminder = space_count % 4;
              let col_count = (space_count / 4) + reminder;
              col_index += col_count;
            } else {
              col_index += 1;
            }
            for i in 0..(col_index + 1) {
              if acc.len() == i {
                acc.push(Vec::new());
              }
            }
          } else {
            //If the value is not a space push the first (and only) char on to the column currently targeted.
            acc
              .get_mut(col_index)
              .unwrap()
              .push(v.chars().next().unwrap());
          }
        });
        acc
      },
    )
}
fn parse_moves(inp: &str) -> Vec<(usize, usize, usize)> {
  //A move looks like "move 1 from 2 to 1".
  inp
    .lines()
    .map(|l| l.split(' ').collect::<Vec<&str>>())
    .map(|l| {
      //After splitting on space the count is index 1, from is index 3 and to is index 5.
      (
        l[1].parse::<usize>().unwrap(),     //count, number of slots to move
        l[3].parse::<usize>().unwrap() - 1, //1 indexed FROM, make 0 index
        l[5].parse::<usize>().unwrap() - 1, //1 indexed TO, make 0 index
      )
    })
    .collect()
}

fn part1(inp: &str) -> String {
  let state_and_moves: Vec<&str> = inp.split("\n\n").collect();
  let mut state = parse_state(state_and_moves.get(0).unwrap());
  let moves = parse_moves(state_and_moves.get(1).unwrap());

  moves.iter().for_each(|(count, from_index, to_index)| {
    let mut chars_to_move: Vec<char> = state
      .get_mut(*from_index)
      .unwrap()
      .drain(0..*count)
      .rev()
      .collect();
    let to = state.get_mut(*to_index).unwrap();
    chars_to_move.append(to);
    *to = chars_to_move;
  });

  state.iter().fold("".to_owned(), |mut acc, column| {
    acc.push(*column.get(0).unwrap());
    acc
  })
}

fn part2(inp: &str) -> String {
  let state_and_moves: Vec<&str> = inp.split("\n\n").collect();
  let mut state = parse_state(state_and_moves.get(0).unwrap());
  let moves = parse_moves(state_and_moves.get(1).unwrap());

  moves.iter().for_each(|(count, from_index, to_index)| {
    let mut chars_to_move: Vec<char> = state
      .get_mut(*from_index)
      .unwrap()
      .drain(0..*count)
      // .rev() NOTE: This is the only difference from part 1.
      .collect();
    let to = state.get_mut(*to_index).unwrap();
    chars_to_move.append(to);
    *to = chars_to_move;
  });
  state.iter().fold("".to_owned(), |mut acc, column| {
    acc.push(*column.get(0).unwrap());
    acc
  })
}

pub fn exec_part1() -> String {
  part1(&fs::read_to_string("./inputs/day05.txt").unwrap())
}
pub fn exec_part2() -> String {
  part2(&fs::read_to_string("./inputs/day05.txt").unwrap())
}

#[cfg(test)]
mod tests {
  use super::*;

  const EXAMPLEDATA: &str = "
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

  #[test]
  fn part1_example_data() {
    let result = part1(EXAMPLEDATA);
    assert_eq!(result, "CMZ");
  }
  #[test]
  fn part2_example_data() {
    let result = part2(EXAMPLEDATA);
    assert_eq!(result, "MCD");
  }
}
