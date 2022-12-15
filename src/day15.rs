use std::cmp::Ordering;
use std::ops::RangeInclusive;

use util::{FromChar, FromLine, FromLines, read, run, Vec2};

mod util;

fn main() {
    let (t0, input) = run(|| read::<Input, _>("inputs/day15.txt"));
    let (t1, p1) = run(|| input.part_1());
    let (t2, p2) = run(|| input.part_2());

    println!("Part 1 : {}", p1);
    println!("Part 2 : {}", p2);
    println!("Time : {} ns", (t0 + t1 + t2).as_nanos());
}

#[derive(Debug)]
struct Input {
    report: Report,
}

impl Input {
    fn part_1(&self) -> u64 {
        println!("{:#?}", self.report);

        0
    }

    fn part_2(&self) -> u64 {
        0
    }
}

type Position = Vec2<i64>;

#[derive(Debug)]
struct Report {
    sensors: Vec<Sensor>,
}

impl Report {
    fn slice(&self, height: i64) -> Vec<Slice> {
        // Collect all slices.
        let mut slices: Vec<Slice> = self.sensors
            .iter()
            .filter_map(|sensor| {
                // Find all sensors that overlap with line at height.
                let x = sensor.position.x();
                let y = sensor.position.y();
                let distance = sensor.distance;

                // How much does this sensor overlap ?
                let overlap = distance - (height - y).abs();
                if overlap >= 0 {
                    Some(Slice(x - overlap..=x + overlap))
                } else {
                    None
                }
            })
            .collect();

        // Merge slices.
        let mut current = 0; // Current slice that we are merging into.
        slices.sort(); // Sort slices (by start value).

        for i in 1..slices.len() {}
        todo!()
    }
}

#[derive(Debug)]
struct Sensor {
    position: Position,
    distance: i64,
}

#[derive(Debug, Eq, PartialEq)]
struct Slice(RangeInclusive<i64>);

impl Slice {
    fn start(&self) -> &i64 {
        self.0.start()
    }

    fn end(&self) -> &i64 {
        self.0.end()
    }

    fn overlap(&self, other : &Self) -> bool {
        self.end() <= other.start()
    }

    fn merge(&self, other : &Self) -> Self {
        Self()
    }
}

impl Ord for Slice {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start().cmp(&other.start())
    }
}

impl PartialOrd<Self> for Slice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromLines for Input {
    fn from_lines(lines: &[&str]) -> Self {
        let report = Report::from_lines(lines);

        Self {
            report
        }
    }
}

impl FromLines for Report {
    fn from_lines(lines: &[&str]) -> Self {
        let sensors = lines.iter().map(line_to!(Sensor)).collect();

        Self {
            sensors
        }
    }
}

impl FromLine for Sensor {
    fn from_line(line: &str) -> Self {
        fn find_coordinate(mut line: &str, start_delimiter: char, end_delimiter: Option<char>) -> (&str, &str) {
            let start_pos = line.find(start_delimiter).expect("delimiter should exist");
            let end_pos = end_delimiter.map(|end_delimiter| line.find(end_delimiter).expect("delimiter should exist")).unwrap_or(line.len());
            let coordinate = &line[start_pos + 1..end_pos];
            (coordinate, &line[end_pos..])
        }

        let (x, line) = find_coordinate(line, '=', Some(','));
        let (y, line) = find_coordinate(line, '=', Some(':'));
        let sensor = vec2!(i64::from_line(x), i64::from_line(y));

        let (x, line) = find_coordinate(line, '=', Some(','));
        let (y, _) = find_coordinate(line, '=', None);
        let beacon = vec2!(i64::from_line(x), i64::from_line(y));
        let distance = (sensor.x() - beacon.x()).abs() + (sensor.y() - beacon.y()).abs();

        Self {
            position: sensor,
            distance,
        }
    }
}