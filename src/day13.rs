use std::cmp::Ordering;

use util::{FromLine, FromLines, read};

mod util;

fn main() {
    let input: Input = read("inputs/day13.txt");
    println!("Part 1 : {}", input.part_1());
    println!("Part 2 : {}", input.part_2());
}

#[derive(Debug)]
struct Input {
    pairs: Vec<PacketPair>,
}

impl Input {
    fn part_1(&self) -> usize {
        self.pairs
            .iter()
            .enumerate()
            .filter(|(_, it)| it.0.cmp(&it.1).is_le())
            .map(|(i, _)| i + 1)
            .sum()
    }

    fn part_2(&self) -> usize {
        let divider1 = Packet(PacketData::List(vec![PacketData::List(vec![PacketData::Number(2)])]));
        let divider2 = Packet(PacketData::List(vec![PacketData::List(vec![PacketData::Number(6)])]));

        let mut packets = Vec::with_capacity(self.pairs.len() * 2 + 2);
        for pair in &self.pairs {
            packets.push(pair.0.clone());
            packets.push(pair.1.clone());
        }
        packets.push(divider1.clone());
        packets.push(divider2.clone());
        packets.sort();

        let divider1_idx = packets.iter().position(|it| *it == divider1).unwrap_or(0) + 1;
        let divider2_idx = packets.iter().position(|it| *it == divider2).unwrap_or(0) + 1;

        divider1_idx * divider2_idx
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct PacketPair(Packet, Packet);

#[derive(Debug, Eq, PartialEq, Clone)]
struct Packet(PacketData);

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum PacketData {
    Number(u64),
    List(Vec<PacketData>),
}

impl Ord for PacketData {
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

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromLines for Input {
    fn from_lines(lines: &[&str]) -> Self {
        let pairs = lines.split(on_empty_line!()).map(lines_to!(PacketPair)).collect();

        Self {
            pairs
        }
    }
}

impl FromLines for PacketPair {
    fn from_lines(lines: &[&str]) -> Self {
        if lines.len() != 2 { panic!("packet pairs should have two packets"); }

        let (lhs, rhs) = (Packet::from_line(lines[0]), Packet::from_line(lines[1]));

        Self(lhs, rhs)
    }
}

impl FromLine for Packet {
    fn from_line(line: &str) -> Self {
        let data = PacketData::from_line(line);

        Self(data)
    }
}

impl FromLine for PacketData {
    fn from_line(line: &str) -> Self {
        if line.len() < 1 { panic!("packet data should not be empty"); }

        match &line[0..1] {
            "[" => {
                let mut depth = 0;

                Self::List(
                    line[1..line.len() - 1]
                        .split(|it| {
                            if it == '[' { depth += 1 } else if it == ']' { depth -= 1 };
                            it == ',' && depth == 0
                        })
                        .filter(|it| !it.is_empty())
                        .map(line_to!(PacketData))
                        .collect()
                )
            }
            _ => {
                // Number
                Self::Number(
                    u64::from_line(line)
                )
            }
        }
    }
}