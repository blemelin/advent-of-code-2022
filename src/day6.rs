use util::{FromLine, FromLines, read, run};

mod util;

fn main() {
    let (t0, input) = run(|| read::<Input, _>("inputs/day6.txt"));
    let (t1, p1) = run(|| input.part_1());
    let (t2, p2) = run(|| input.part_2());

    println!("Part 1 : {}", p1);
    println!("Part 2 : {}", p2);
    println!("Time : {} ns", (t0 + t1 + t2).as_nanos());
}

#[derive(Debug)]
struct Input {
    buffer: Buffer,
}

impl Input {
    fn part_1(&self) -> usize {
        self.buffer.find_marker(4).unwrap_or(0)
    }

    fn part_2(&self) -> usize {
        self.buffer.find_marker(14).unwrap_or(0)
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

impl FromLines for Input {
    fn from_lines(lines: &[&str]) -> Self {
        let buffer = Buffer::from_line(lines.get(0).expect("input should have a buffer"));

        Self {
            buffer
        }
    }
}

impl FromLine for Buffer {
    fn from_line(line: &str) -> Self {
        let buffer = line.chars().collect();

        Self(buffer)
    }
}