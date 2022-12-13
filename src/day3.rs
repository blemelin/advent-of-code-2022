use util::{FromChar, FromLine, FromLines, read, run};

mod util;

fn main() {
    let (t0, input) = run(|| read::<Input, _>("inputs/day3.txt"));
    let (t1, p1) = run(|| input.part_1());
    let (t2, p2) = run(|| input.part_2());

    println!("Part 1 : {}", p1);
    println!("Part 2 : {}", p2);
    println!("Time : {} ns", (t0 + t1 + t2).as_nanos());
}

#[derive(Debug)]
struct Input {
    rucksacks: Vec<Rucksack>,
}

impl Input {
    fn part_1(&self) -> u64 {
        self.rucksacks
            .iter()
            .filter_map(|it| it.find_duplicate())
            .map(|it| it.priority())
            .sum()
    }

    fn part_2(&self) -> u64 {
        self.rucksacks
            .chunks(3)
            .map(Group::new)
            .filter_map(|it| it.find_duplicate())
            .map(|it| it.priority())
            .sum()
    }
}

#[derive(Debug)]
struct Rucksack {
    items: Vec<Item>,
}

impl Rucksack {
    fn find_duplicate(&self) -> Option<Item> {
        let (lhs, rhs) = self.items.split_at(self.items.len() / 2);

        lhs.iter()
            .find(|it| rhs.contains(*it))
            .map(|it| *it)
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Item(u8);

impl Item {
    fn priority(&self) -> u64 {
        self.0 as u64
    }
}

#[derive(Debug)]
struct Group<'a>(&'a [Rucksack]);

impl<'a> Group<'a> {
    fn new(rucksacks: &'a [Rucksack]) -> Self {
        Self(rucksacks)
    }

    fn find_duplicate(&self) -> Option<Item> {
        self.0
            .iter()
            .map(|it| it.items.clone())
            .reduce(|mut acc, items| {
                acc.retain(|it| items.contains(it));
                acc
            })
            .and_then(|it| it.first().cloned())
    }
}

impl FromLines for Input {
    fn from_lines(lines: &[&str]) -> Self {
        let rucksacks = lines.iter().map(line_to!(Rucksack)).collect();

        Self {
            rucksacks
        }
    }
}

impl FromLine for Rucksack {
    fn from_line(line: &str) -> Self {
        let items: Vec<Item> = line.chars().map(char_to!(Item)).collect();

        Self {
            items
        }
    }
}

impl FromChar for Item {
    fn from_char(char: char) -> Self {
        let code = char as u8;
        match code {
            b'a'..=b'z' => Self(code - b'a' + 1),
            b'A'..=b'Z' => Self(code - b'A' + 1 + 26),
            _ => panic!("\"{char}\" is not a valid item")
        }
    }
}