use util::{FromLine, FromLines, read};

mod util;

fn main() {
    // Read data
    let Data(elves) = read("inputs/day1.txt");

    // Part 1
    let best_elf = elves.last().expect("input data should have at least one elf");
    let calories = best_elf.calories();
    println!("Part 1 : {}", calories);

    // Part 2
    let best_elves: Vec<&Elf> = elves.iter().rev().take(3).collect();
    let calories: u64 = best_elves.iter().map(|it| it.calories()).sum();
    println!("Part 2 : {}", calories);
}

#[derive(Debug)]
struct Data(Vec<Elf>);

impl FromLines for Data {
    fn from_lines(lines: &[&str]) -> Self {
        let mut elves: Vec<Elf> = lines.split(on_empty_line!()).map(lines_to!(Elf)).collect();
        elves.sort_by_key(Elf::calories);

        Self(elves)
    }
}

#[derive(Debug, Default)]
struct Elf {
    calories: u64,
}

impl Elf {
    fn calories(&self) -> u64 {
        self.calories
    }
}

impl FromLines for Elf {
    fn from_lines(lines: &[&str]) -> Self {
        let calories = lines.iter().map(line_to!(u64)).sum();

        Self {
            calories
        }
    }
}