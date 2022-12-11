use util::{FromChar, FromLine, FromLines, read};

mod util;

fn main() {
    let input: Input = read("inputs/day18.txt");
    println!("Part 1 : {}", input.part_1());
    println!("Part 2 : {}", input.part_2());
}

#[derive(Debug)]
struct Input {}

impl Input {
    fn part_1(&self) -> u64 {
        0
    }

    fn part_2(&self) -> u64 {
        0
    }
}

impl FromLines for Input {
    fn from_lines(lines: &[&str]) -> Self {
        todo!()
    }
}