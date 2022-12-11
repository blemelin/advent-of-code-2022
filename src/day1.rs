use util::{FromLine, FromLines, read};

mod util;

fn main() {
    let input: Input = read("inputs/day1.txt");
    println!("Part 1 : {}", input.part_1());
    println!("Part 2 : {}", input.part_2());
}

#[derive(Debug)]
struct Input {
    elves: Vec<Elf>,
}

impl Input {
    fn part_1(&self) -> u64 {
        self.elves
            .last()
            .map(|it| it.calories)
            .unwrap_or(0)
    }

    fn part_2(&self) -> u64 {
        self.elves
            .iter()
            .rev()
            .take(3)
            .map(|it| it.calories)
            .sum()
    }
}

impl FromLines for Input {
    fn from_lines(lines: &[&str]) -> Self {
        let mut elves: Vec<Elf> = lines.split(on_empty_line!()).map(lines_to!(Elf)).collect();
        elves.sort_by_key(|it| it.calories);

        Self {
            elves
        }
    }
}

#[derive(Debug)]
struct Elf {
    calories: u64,
}

impl FromLines for Elf {
    fn from_lines(lines: &[&str]) -> Self {
        let calories = lines.iter().map(line_to!(u64)).sum();

        Self {
            calories
        }
    }
}