use util::{FromLine, FromLines, read, run};

mod util;

fn main() {
    let (t0, input) = run(|| read::<Input, _>("inputs/day1.txt"));
    let (t1, p1) = run(|| input.part_1());
    let (t2, p2) = run(|| input.part_2());

    println!("Part 1 : {}", p1);
    println!("Part 2 : {}", p2);
    println!("Time : {} ns", (t0 + t1 + t2).as_nanos());
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

#[derive(Debug)]
struct Elf {
    calories: u64,
}

impl FromLines for Input {
    fn from_lines(lines: &[&str]) -> Self {
        let mut elves: Vec<Elf> = lines.split(is_empty!()).map(lines_to!(Elf)).collect();
        elves.sort_by_key(|it| it.calories);

        Self {
            elves
        }
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