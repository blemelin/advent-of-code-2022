use std::cmp::Ordering;

use util::{FromLine, FromLines, read, run, Vec2};

mod util;

const PART_1_HEIGHT: i64 = 2_000_000;
// const PART_1_HEIGHT: i64 = 10;
const PART_2_HEIGHT: i64 = 4_000_000;
// const PART_2_HEIGHT: i64 = 20;
const PART_2_WIDTH: i64 = PART_2_HEIGHT;
const PART_2_MULTIPLIER: i64 = 4_000_000;

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
    fn part_1(&self) -> i64 {
        // Extract slices.
        let slices = self.report.slices(PART_1_HEIGHT);

        // Merge slices
        let merged_slices = Report::merge(slices);

        // Counts positions covered by slice.
        Report::length(&merged_slices)
    }

    fn part_2(&self) -> i64 {
        let mut position = None;
        for y in 0..PART_2_HEIGHT {
            if y % 100 == 0 { println!("{}", y); } // Omg....
            let slices = self.report.slices(y);
            for x in 0..PART_2_WIDTH {
                let mut exist = false;
                for slice in &slices {
                    if slice.contains(x) {
                        exist = true;
                        break;
                    }
                }
                if !exist {
                    position = Some(vec2!(x, y));
                }
            }
        }
        position.map(|it| it.x() * PART_2_MULTIPLIER + it.y()).unwrap_or(0)
    }
}

type Position = Vec2<i64>;

#[derive(Debug)]
struct Report {
    sensors: Vec<Sensor>,
}

impl Report {
    fn slices(&self, height: i64) -> Vec<Slice> {
        self.sensors
            .iter()
            .filter_map(|sensor| {
                // Find all sensors that overlap with line at height.
                let x = sensor.position.x();
                let y = sensor.position.y();
                let distance = sensor.distance;

                // How much does this sensor overlap ?
                let overlap = distance - (height - y).abs();
                if overlap >= 0 {
                    Some(Slice::new(x - overlap, x + overlap))
                } else {
                    None
                }
            })
            .collect()
    }

    fn merge(mut slices: Vec<Slice>) -> Vec<Slice> {
        // Sort slices first (by start value).
        slices.sort();

        // Current slice that we are merging into.
        let mut current = 0;
        for other in 1..slices.len() {
            let other_slice = slices[other];
            let current_slice = &mut slices[current];

            // Merge into current if overlapping at the end.
            if current_slice.overlap_end(&other_slice) {
                current_slice.merge_end(&other_slice);
            } else {
                current += 1;
                slices[current] = other_slice;
            }
        }
        slices.resize_with(slices.len().min(current + 1), || panic!("new size should be smaller or equal after merging"));

        slices
    }

    fn length(slices: &Vec<Slice>) -> i64 {
        slices.iter().map(|it| it.len()).sum()
    }
}

#[derive(Debug)]
struct Sensor {
    position: Position,
    distance: i64,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Slice {
    start: i64,
    end: i64,
}

impl Slice {
    fn new(start: i64, end: i64) -> Self {
        Self {
            start,
            end,
        }
    }

    fn len(&self) -> i64 {
        self.end - self.start
    }

    fn contains(&self, value: i64) -> bool {
        self.start <= value && value <= self.end
    }

    fn overlap_end(&self, other: &Self) -> bool {
        other.start <= self.end
    }

    fn merge_end(&mut self, other: &Self) {
        self.end = self.end.max(other.end);
    }
}

impl Ord for Slice {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.cmp(&other.start)
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
        fn find_coordinate(line: &str, start_delimiter: char, end_delimiter: Option<char>) -> (&str, &str) {
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