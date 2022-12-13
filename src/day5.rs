use util::{FromLine, FromLines, read, run};

mod util;

fn main() {
    let (t0, input) = run(|| read::<Input, _>("inputs/day5.txt"));
    let (t1, p1) = run(|| input.part_1());
    let (t2, p2) = run(|| input.part_2());

    println!("Part 1 : {}", p1);
    println!("Part 2 : {}", p2);
    println!("Time : {} ns", (t0 + t1 + t2).as_nanos());
}

#[derive(Debug)]
struct Input {
    supplies: Supplies,
    commands: Vec<Command>,
}

impl Input {
    fn part_1(&self) -> String {
        let mut supplies = self.supplies.clone();
        supplies.apply(&self.commands);
        supplies.peek()
    }

    fn part_2(&self) -> String {
        let mut supplies = self.supplies.clone();
        supplies.apply_multiple(&self.commands);
        supplies.peek()
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

#[derive(Debug, Clone)]
struct Stack(Vec<char>);

impl Stack {
    fn new() -> Self {
        Self(Vec::new())
    }

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
        self.0.split_off(self.0.len() - size)
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

impl FromLines for Input {
    fn from_lines(lines: &[&str]) -> Self {
        let mut parts = lines.split(is_empty!());
        let supplies = Supplies::from_lines(parts.next().expect("input should have supplies"));
        let commands = parts.next().expect("input should have commands").iter().map(line_to!(Command)).collect();

        Self {
            supplies,
            commands,
        }
    }
}

impl FromLines for Supplies {
    fn from_lines(lines: &[&str]) -> Self {
        if lines.len() < 2 { panic!("supplies data should have at least one row (excluding the footer)"); }

        let lines = &lines[..lines.len() - 1]; // Last line (stack indexes) is not used.
        let nb_cols = (lines[0].len() + 1) / 4; // Each column takes 4 chars.

        let mut stacks = vec![Stack::new(); nb_cols];
        for line in lines.iter().rev() {
            for (i, chunk) in line.chars().collect::<Vec<char>>().chunks(4).enumerate() {
                let item = chunk.iter().nth(1).unwrap_or(&' ');
                if !item.is_whitespace() {
                    stacks[i].push(*item)
                }
            }
        }

        Self(stacks)
    }
}

impl FromLine for Command {
    fn from_line(line: &str) -> Self {
        let parts: Vec<&str> = line.splitn(6, ' ').collect();
        match parts[..] {
            ["move", count, "from", source, "to", destination] => {
                let count = usize::from_line(count);
                let source = usize::from_line(source);
                let destination = usize::from_line(destination);

                Self {
                    count,
                    source,
                    destination,
                }
            }
            _ => panic!("{line} is not a valid command")
        }
    }
}