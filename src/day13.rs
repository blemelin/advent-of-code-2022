use std::cmp::Ordering;

use util::{FromLine, FromLines, read, run};

mod util;

fn main() {
    let (t0, input) = run(|| read::<Input, _>("inputs/day13.txt"));
    let (t1, p1) = run(|| input.part_1());
    let (t2, p2) = run(|| input.part_2());

    println!("Part 1 : {}", p1);
    println!("Part 2 : {}", p2);
    println!("Time : {} ns", (t0 + t1 + t2).as_nanos());
}

#[derive(Debug)]
struct Input {
    packets: Vec<Packet>,
}

impl Input {
    fn part_1(&self) -> usize {
        self.packets
            .chunks_exact(2)
            .enumerate()
            .filter(|(_, pair)| pair[0].cmp(&pair[1]).is_le())
            .map(|(i, _)| i + 1)
            .sum()
    }

    fn part_2(&self) -> usize {
        [Packet::Number(2), Packet::Number(6)].iter().enumerate().map(|(i, divider)| {
            self.packets.iter().filter(|it| *it < divider).count() + i + 1
        }).product()
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Packet {
    Number(u64),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Number(lhs), Self::Number(rhs)) => {
                lhs.cmp(rhs)
            }
            (Self::List(lhs), Self::List(rhs)) => {
                lhs.cmp(rhs)
            }
            (Self::List(lhs), Self::Number(rhs)) => {
                lhs.cmp(&vec![Self::Number(*rhs)])
            }
            (Self::Number(lhs), Self::List(rhs)) => {
                vec![Self::Number(*lhs)].cmp(rhs)
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromLines for Input {
    fn from_lines(lines: &[&str]) -> Self {
        let packets = lines.iter().filter(is_not_empty!()).map(line_to!(Packet)).collect();

        Self {
            packets
        }
    }
}

impl FromLine for Packet {
    fn from_line(line: &str) -> Self {
        if line.len() < 1 { panic!("packet should not be empty"); }

        match &line[0..1] {
            "[" => {
                let mut depth = 0;

                Self::List(
                    line[1..line.len() - 1]
                        .split(|it| {
                            if it == '[' { depth += 1 } else if it == ']' { depth -= 1 };
                            it == ',' && depth == 0
                        })
                        .filter(is_not_empty!())
                        .map(line_to!(Packet))
                        .collect()
                )
            }
            _ => {
                Self::Number(
                    u64::from_line(line)
                )
            }
        }
    }
}