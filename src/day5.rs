use std::iter::repeat_with;
use util::{FromLine, FromLines, read};

mod util;

fn main() {
    // Read data
    let Data(supplies, commands) = read("inputs/day5.txt");

    // Part 1
    let mut part1 = supplies.clone();
    part1.apply(&commands);
    println!("Part 1 : {}", part1.peek());

    // Part 2
    let mut part2 = supplies.clone();
    part2.apply_multiple(&commands);
    println!("Part 2 : {}", part2.peek());
}

#[derive(Debug)]
struct Data(Supplies, Vec<Command>);

impl FromLines for Data {
    fn from_lines(lines: &[&str]) -> Self {
        let mut parts = lines.split(on_empty_line!());
        let supplies = Supplies::from_lines(parts.next().expect("input data supplies part should exist"));
        let commands = parts.next().expect("input data commands part should exist").iter().map(line_to!(Command)).collect();

        Self(supplies, commands)
    }
}

#[derive(Debug, Clone)]
struct Supplies(Vec<Stack>);

impl Supplies {
    fn peek(&self) -> String {
        self.0.iter().filter_map(|it| it.peek()).collect()
    }

    fn apply(&mut self, commands: &[Command]) {
        let stacks = &mut self.0;
        for command in commands {
            for _ in 0..command.count {
                let item = stacks[command.source - 1].pop();
                stacks[command.destination - 1].push(item);
            }
        }
    }

    fn apply_multiple(&mut self, commands: &[Command]) {
        let stacks = &mut self.0;
        for command in commands {
            let items = stacks[command.source - 1].pop_multiple(command.count);
            stacks[command.destination - 1].push_multiple(items);
        }
    }
}

impl FromLines for Supplies {
    fn from_lines(lines: &[&str]) -> Self {
        if lines.len() < 2 { panic!("supplies data should have at least one row (excluding the footer)"); }

        let lines = &lines[..lines.len() - 1]; // Last line (stack indexes) is not used.
        let nb_cols = (lines[0].len() + 1) / 4; // Each column takes 4 chars.
        let mut stacks: Vec<Vec<char>> = repeat_with(|| Vec::new()).take(nb_cols).collect();
        stacks = lines.iter().fold(stacks, |mut acc, it| {
            for (i, chunk) in it.chars().collect::<Vec<char>>().chunks(4).enumerate() {
                match chunk.iter().nth(1) { // Ex: "[D]". Supply name is at index 1.
                    Some(' ') => { /* Skip. Empty. */ }
                    Some(item) => { acc[i].insert(0, *item) }
                    _ => { /* Skip. Most likely end of line. */ }
                }
            }
            acc
        });

        Self(stacks.into_iter().map(|it| Stack(it)).collect())
    }
}

#[derive(Debug, Clone)]
struct Stack(Vec<char>);

impl Stack {
    fn peek(&self) -> Option<char> {
        self.0.last().map(|it| *it)
    }

    fn pop(&mut self) -> char {
        self.0.pop().expect("stack should not be empty")
    }

    fn push(&mut self, item: char) {
        self.0.push(item)
    }

    fn pop_multiple(&mut self, size: usize) -> Vec<char> {
        let start = self.0.len() - size;
        let end = self.0.len();
        self.0.splice(start..end, []).collect()
    }

    fn push_multiple(&mut self, mut items: Vec<char>) {
        self.0.append(&mut items);
    }
}

#[derive(Debug)]
struct Command {
    count: usize,
    source: usize,
    destination: usize,
}

impl FromLine for Command {
    fn from_line(line: &str) -> Self {
        let mut parts = line.split(' ');
        let count = usize::from_line(parts.nth(1).expect("command should have a count"));
        let source = usize::from_line(parts.nth(1).expect("command should have a source"));
        let destination = usize::from_line(parts.nth(1).expect("command should have a destination"));

        Self {
            count,
            source,
            destination,
        }
    }
}