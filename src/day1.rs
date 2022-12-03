use std::{vec};

const _EXAMPLE: &str = "1000\n\
2000\n\
3000\n\
\n\
4000\n\
\n\
5000\n\
6000\n\
\n\
7000\n\
8000\n\
9000\n\
\n\
10000\n\
";

pub fn run() {
  let contents = std::fs::read_to_string("inputs/1").expect("Input not found");
  // let contents = _EXAMPLE;
  let mut elves = vec![];
  let mut elf = 0;
  for line in contents.split('\n').collect::<Vec<&str>>() {
    if line == "" {
      elves.push(elf);
      elf = 0;
    } else {
      elf += line.parse::<i32>().unwrap();
    }
  }
  println!("{:?}", elves.iter().max().unwrap());
  elves.sort();
  elves.reverse();
  println!("{:?}", elves.iter().take(3).sum::<i32>())
}
