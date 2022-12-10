use util::{FromLine, FromLines, read};

mod util;

fn main() {
    // Read data
    let Data(buffer) = read("inputs/day6.txt");

    // Part 1
    let result = buffer.marker(4).expect("first marker should be in input data");
    println!("Part 1 : {}", result);

    // Part 2
    let result = buffer.marker(14).expect("second marker should be in input data");
    println!("Part 2 : {}", result);
}

#[derive(Debug)]
struct Data(Buffer);

impl FromLines for Data {
    fn from_lines(lines: &[&str]) -> Self {
        let buffer = Buffer::from_line(lines.get(0).expect("data should have one (and only one) line"));

        Self(buffer)
    }
}

#[derive(Debug)]
struct Buffer(String);

impl Buffer {
    fn marker(&self, len: usize) -> Option<usize> {
        if self.0.len() < len { panic!("marker length should be smaller or equal to buffer length"); }

        (len..self.0.len()).filter(move |it| {
            let slice: Vec<char> = self.0[*it - len..*it].chars().collect();
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
        let buffer = line.to_owned();

        Self(buffer)
    }
}