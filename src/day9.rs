use std::collections::HashSet;

use util::{FromLine, FromLines, read, Vec2};

mod util;

fn main() {
    // Read data
    let Data(commands) = read("inputs/day9.txt");

    // Part 1
    let mut rope = Rope::<2>::new();
    for command in &commands {
        rope.apply(command);
    }
    println!("Part 1 : {:?}", rope.visited());

    // Part 2
    let mut rope = Rope::<10>::new();
    for command in &commands {
        rope.apply(command);
    }
    println!("Part 2 : {:?}", rope.visited());
}

#[derive(Debug)]
struct Data(Vec<Command>);

impl FromLines for Data {
    fn from_lines(lines: &[&str]) -> Self {
        let commands = lines.iter().map(line_to!(Command)).collect();

        Self(commands)
    }
}

#[derive(Debug)]
struct Rope<const N: usize> {
    knots: [Vec2; N],
    visited: HashSet<Vec2>,
}

impl<const N: usize> Rope<N> {
    fn new() -> Self {
        if N < 2 { panic!("rope length should be at least 2"); }

        Self {
            knots: [vec2!(0,0); N],
            visited: HashSet::from([vec2!(0,0)]),
        }
    }

    fn apply(&mut self, command: &Command) {
        let length = command.length;
        let head_direction = command.direction;
        for _ in 0..length {
            // Apply motion to head.
            self.knots[0] += head_direction;

            // Apply motion to following knots.
            for i in 0..N - 1 {
                let head = self.knots[i];
                let tail = &mut self.knots[i + 1];

                let distance_x = head.x() - tail.x();
                let distance_y = head.y() - tail.y();
                let distance = distance_x.abs().max(distance_y.abs());
                if distance > 1 {
                    let move_x = distance_x.clamp(-1, 1);
                    let move_y = distance_y.clamp(-1, 1);
                    let tail_direction = vec2!(move_x, move_y);
                    *tail += tail_direction;
                }
            }

            // Record position visited by tail.
            self.visited.insert(self.knots[N - 1]);
        }
    }

    fn visited(&self) -> usize {
        self.visited.len()
    }
}

#[derive(Debug)]
struct Command {
    direction: Vec2,
    length: usize,
}

impl FromLine for Command {
    fn from_line(line: &str) -> Self {
        let mut parts = line.split(' ');
        let direction = parts.next().expect("command should have a direction");
        let length = usize::from_line(parts.next().expect("command should have a length"));

        let direction = match direction {
            "U" => vec2!(0,1),
            "D" => vec2!(0,-1),
            "L" => vec2!(-1,0),
            "R" => vec2!(1,0),
            _ => panic!("{direction} is not a valid direction")
        };

        Self {
            direction,
            length,
        }
    }
}