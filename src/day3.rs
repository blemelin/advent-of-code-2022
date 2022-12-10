use util::{FromChar, FromLine, FromLines, read};

mod util;

fn main() {
    // Read data
    let Data(rucksacks) = read("inputs/day3.txt");

    // Part 1
    let result: u64 = rucksacks
        .iter()
        .filter_map(|it| it.find_duplicate())
        .map(|it| it.priority())
        .sum();
    println!("Part 1 : {}", result);

    // Part 2
    let result: u64 = rucksacks
        .chunks(3)
        .map(Group::from)
        .filter_map(|it| it.find_duplicate())
        .map(|it| it.priority())
        .sum();
    println!("Part 2 : {}", result);
}

#[derive(Debug)]
struct Data(Vec<Rucksack>);

impl FromLines for Data {
    fn from_lines(lines: &[&str]) -> Self {
        let rucksacks = lines.iter().map(line_to!(Rucksack)).collect();

        Self(rucksacks)
    }
}

#[derive(Debug)]
struct Rucksack(Vec<Item>);

impl Rucksack {
    fn find_duplicate(&self) -> Option<Item> {
        let len = self.0.len() / 2;
        let lhs = &self.0[0..len];
        let rhs = &self.0[len..];

        lhs.iter()
            .find(|it| rhs.contains(*it))
            .map(|it| *it)
    }

    fn items(&self) -> &[Item] {
        &self.0
    }
}

impl FromLine for Rucksack {
    fn from_line(line: &str) -> Self {
        let items : Vec<Item> = line.chars().map(char_to!(Item)).collect();
        if items.is_empty() { panic!("rucksack should have at least one item"); }

        Self(items)
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Item(u64);

impl Item {
    fn priority(&self) -> u64 {
        self.0
    }
}

impl FromChar for Item {
    fn from_char(char: char) -> Self {
        let code = char as u64;
        match code {
            /* a..z */ 97..=122 => Self(code - 97 + 1),
            /* A..Z */ 65..=90 => Self(code - 65 + 26 + 1),
            _ => panic!("\"{char}\" is not a valid item")
        }
    }
}

#[derive(Debug)]
struct Group<'a>(&'a [Rucksack]);

impl<'a> Group<'a> {
    fn from(rucksacks: &'a [Rucksack]) -> Self {
        Self(rucksacks)
    }

    fn find_duplicate(&self) -> Option<Item> {
        self.0
            .iter()
            .map(|it| it.items().to_owned())
            .reduce(|mut acc, it| {
                acc.retain(|x| it.contains(x));
                acc
            })
            .and_then(|it| it.first().map(|it| *it))
    }
}