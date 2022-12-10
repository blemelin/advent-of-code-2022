use std::ops::RangeInclusive;

use util::{FromLine, FromLines, read};

mod util;

fn main() {
    // Read data
    let Data(pairs) = read("inputs/day4.txt");

    // Part 1
    let count = pairs.iter().filter(|it| it.has_full_overlap()).count();
    println!("Part 1 : {}", count);

    // Part 2
    let count = pairs.iter().filter(|it| it.has_overlap()).count();
    println!("Part 2 : {}", count);
}

#[derive(Debug)]
struct Data(Vec<Pair>);

impl FromLines for Data {
    fn from_lines(lines: &[&str]) -> Self {
        let pairs = lines.iter().map(line_to!(Pair)).collect();

        Self(pairs)
    }
}

#[derive(Debug)]
struct Pair {
    lhs: Assignment,
    rhs: Assignment,
}

impl Pair {
    fn has_overlap(&self) -> bool {
        self.lhs.contains(&self.rhs) || self.rhs.contains(&self.lhs)
    }

    fn has_full_overlap(&self) -> bool {
        self.lhs.contains_all(&self.rhs) || self.rhs.contains_all(&self.lhs)
    }
}

impl FromLine for Pair {
    fn from_line(line: &str) -> Self {
        let mut parts = line.split(',');
        let lhs = parts.next().expect("pair should have a first assignment");
        let rhs = parts.next().expect("pair should have a second assignment");

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
    fn contains(&self, other: &Self) -> bool {
        for i in other.range.clone() {
            if self.range.contains(&i) { return true; }
        }
        false
    }

    fn contains_all(&self, other: &Self) -> bool {
        for i in other.range.clone() {
            if !self.range.contains(&i) { return false; }
        }
        true
    }
}

impl FromLine for Assignment {
    fn from_line(line: &str) -> Self {
        let mut parts = line.split('-');
        let start = parts.next().expect("assignment should have a start value");
        let end = parts.next().expect("assignment should have a end value");
        let start = u64::from_line(start);
        let end = u64::from_line(end);

        Self {
            range: start..=end
        }
    }
}