use util::{FromLine, FromLines, read};

mod util;

fn main() {
    let input: Input = read("inputs/day6.txt");
    println!("Part 1 : {:?}", input.part_1());
    println!("Part 2 : {:?}", input.part_2());
}

#[derive(Debug)]
struct Input {
    buffer: Buffer,
}

impl Input {
    fn part_1(&self) -> Option<usize> {
        self.buffer.find_marker(4)
    }

    fn part_2(&self) -> Option<usize> {
        self.buffer.find_marker(14)
    }
}

impl FromLines for Input {
    fn from_lines(lines: &[&str]) -> Self {
        let buffer = Buffer::from_line(lines.get(0).expect("input should have a buffer"));

        Self {
            buffer
        }
    }
}

#[derive(Debug)]
struct Buffer(Vec<char>);

impl Buffer {
    fn find_marker(&self, len: usize) -> Option<usize> {
        if self.0.len() < len { panic!("marker length should be smaller or equal to buffer length"); }

        (len..self.0.len()).filter(move |it| {
            let slice = &self.0[*it - len..*it];
            for i in 0..len - 1 {
                for j in i + 1..len {
                    if slice[i] == slice[j] { return false; }
                }
            }
            true
        }).next()
    }
}

impl FromLine for Buffer {
    fn from_line(line: &str) -> Self {
        let buffer = line.chars().collect();

        Self(buffer)
    }
}