use std::collections::HashSet;

use util::{FromLine, FromLines, read, Vec2};

mod util;

fn main() {
    let input: Input = read("inputs/day9.txt");
    println!("Part 1 : {}", input.part_1());
    println!("Part 2 : {}", input.part_2());
}

#[derive(Debug)]
struct Input {
    motions: Vec<Motion>,
}

impl Input {
    fn part_1(&self) -> usize {
        let mut rope = Rope::<2>::new();
        for motion in &self.motions {
            rope.apply(motion);
        }
        rope.visited_count()
    }

    fn part_2(&self) -> usize {
        let mut rope = Rope::<10>::new();
        for motion in &self.motions {
            rope.apply(motion);
        }
        rope.visited_count()
    }
}

impl FromLines for Input {
    fn from_lines(lines: &[&str]) -> Self {
        let commands = lines.iter().map(line_to!(Motion)).collect();

        Self {
            motions: commands
        }
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

    fn apply(&mut self, motion: &Motion) {
        let length = motion.length;
        let head_direction = motion.direction;
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

    fn visited_count(&self) -> usize {
        self.visited.len()
    }
}

#[derive(Debug)]
struct Motion {
    direction: Vec2,
    length: usize,
}

impl FromLine for Motion {
    fn from_line(line: &str) -> Self {
        let (direction, length) = line.split_once(' ').expect("motions should have a direction and a length");

        let direction = match direction {
            "U" => vec2!(0,1),
            "D" => vec2!(0,-1),
            "L" => vec2!(-1,0),
            "R" => vec2!(1,0),
            _ => panic!("{direction} is not a valid direction")
        };
        let length = usize::from_line(length);

        Self {
            direction,
            length,
        }
    }
}