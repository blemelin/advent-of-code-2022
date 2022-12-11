use std::ops::RangeInclusive;

use util::{FromLine, FromLines, read};

mod util;

fn main() {
    let input: Input = read("inputs/day4.txt");
    println!("Part 1 : {}", input.part_1());
    println!("Part 2 : {}", input.part_2());
}

#[derive(Debug)]
struct Input {
    pairs: Vec<Pair>,
}

impl Input {
    fn part_1(&self) -> usize {
        self.pairs
            .iter()
            .filter(|it| it.has_full_overlap())
            .count()
    }

    fn part_2(&self) -> usize {
        self.pairs
            .iter()
            .filter(|it| it.has_overlap())
            .count()
    }
}

impl FromLines for Input {
    fn from_lines(lines: &[&str]) -> Self {
        let pairs = lines.iter().map(line_to!(Pair)).collect();

        Self {
            pairs
        }
    }
}

#[derive(Debug)]
struct Pair {
    lhs: Assignment,
    rhs: Assignment,
}

impl Pair {
    fn has_full_overlap(&self) -> bool {
        let lhs_start = self.lhs.start();
        let lhs_end = self.lhs.end();
        let rhs_start = self.rhs.start();
        let rhs_end = self.rhs.end();

        // Left contains right or right contains left.
        (rhs_start <= lhs_start && rhs_end >= lhs_end) || (lhs_start <= rhs_start && lhs_end >= rhs_end)
    }

    fn has_overlap(&self) -> bool {
        let lhs_start = self.lhs.start();
        let lhs_end = self.lhs.end();
        let rhs_start = self.rhs.start();
        let rhs_end = self.rhs.end();

        // Left overlap with right for at least one.
        lhs_start <= rhs_end && lhs_end >= rhs_start
    }
}

impl FromLine for Pair {
    fn from_line(line: &str) -> Self {
        let (lhs, rhs) = line.split_once(',').expect("pair should have a left and a right assignment");

        Self {
            lhs: Assignment::from_line(lhs),
            rhs: Assignment::from_line(rhs),
        }
    }
}

#[derive(Debug)]
struct Assignment {
    range: RangeInclusive<u64>,
}

impl Assignment {
    fn start(&self) -> u64 {
        *self.range.start()
    }

    fn end(&self) -> u64 {
        *self.range.end()
    }
}

impl FromLine for Assignment {
    fn from_line(line: &str) -> Self {
        let (start, end) = line.split_once('-').expect("assignment should have a start and a end value");
        let start = u64::from_line(start);
        let end = u64::from_line(end);

        Self {
            range: start..=end
        }
    }
}