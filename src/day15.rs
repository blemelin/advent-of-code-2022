use std::thread;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicI64, Ordering};

use util::{FromLine, FromLines, read, run, Vec2};

mod util;

// Optimisation opportunity : find all intersections of the sensors (it's diamond shaped).
// For each of them, check if the position just bellow is inside one of the sensors ranges.
// If not, it's the positions we are looking for.

const PART_1_HEIGHT: i64 = 2_000_000;
// const PART_1_HEIGHT: i64 = 10;
const PART_2_HEIGHT: i64 = 4_000_000;
// const PART_2_HEIGHT: i64 = 20;
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
        // Extract intervals.
        let intervals = self.report.slice(PART_1_HEIGHT);

        // Merge intervals.
        let intervals = Report::merge(intervals);

        // Counts positions covered by interval.
        Report::length(&intervals)
    }

    fn part_2(&self) -> i64 {
        // This is quite a long process (400 ms), but it can be parallelized (down to 10 ms).
        let thread_count = thread::available_parallelism().map(|it| it.get()).unwrap_or(1);

        // Channel to receive the answer.
        let found = Arc::new(AtomicBool::new(false));
        let value = Arc::new(AtomicI64::new(0));

        // Create threads.
        thread::scope(|s| {
            let thread_height = (PART_2_HEIGHT as usize / thread_count) as i64;
            for i in 0..thread_count {
                let found = found.clone();
                let value = value.clone();
                s.spawn(move || {
                    let start = i as i64 * thread_height;
                    let end = start + thread_height;

                    for y in start..end {
                        // Has a thread found it yet ?
                        if found.load(Ordering::Relaxed) { break; }

                        // Extract intervals.
                        let intervals = self.report.slice(y);
                        // Merge intervals.
                        let intervals = Report::merge(intervals);

                        // If there is a hole, there will be two intervals after merging the others.
                        if intervals.len() >= 2 {
                            // We found the hole. Stop everything!
                            found.store(true, Ordering::Relaxed);

                            // The hole is at the end of the first interval.
                            let x = intervals[0].end + 1;

                            // Submit the result.
                            value.store(x * PART_2_MULTIPLIER + y, Ordering::Relaxed);
                            break;
                        }
                    }
                });
            }
        });

        value.load(Ordering::Relaxed)
    }
}

type Position = Vec2<i64>;

#[derive(Debug)]
struct Report {
    sensors: Vec<Sensor>,
}

impl Report {
    fn slice(&self, height: i64) -> Vec<Interval> {
        self.sensors
            .iter()
            .filter_map(|sensor| {
                let x = sensor.position.x();
                let y = sensor.position.y();
                let distance = sensor.distance;

                // First, get distance from the center of the sensor the the line.
                // Then, remove distance from the beacon. This is the overlap radius.
                let overlap = distance - (height - y).abs();
                if overlap >= 0 {
                    Some(Interval::new(x - overlap, x + overlap))
                } else {
                    None
                }
            })
            .collect()
    }

    fn merge(mut intervals: Vec<Interval>) -> Vec<Interval> {
        // Sort by start value.
        intervals.sort_by_key(|it| it.start);

        // Fold intervals into each other, starting at the first one.
        let mut current = 0;
        for other in 1..intervals.len() {
            let other_interval = intervals[other];
            let current_interval = &mut intervals[current];

            if current_interval.overlap_end(&other_interval) {
                current_interval.merge_end(&other_interval);
            } else {
                current += 1;
                intervals[current] = other_interval;
            }
        }

        // Remove intervals after current. They were all merged.
        intervals.resize_with(intervals.len().min(current + 1), || panic!("new size should be smaller or equal after merging"));
        intervals
    }

    fn length(slices: &Vec<Interval>) -> i64 {
        // Sum of the lengths is equal to the total length covered.
        slices.iter().map(|it| it.len()).sum()
    }
}

#[derive(Debug)]
struct Sensor {
    position: Position,
    distance: i64,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct Interval {
    start: i64,
    end: i64,
}

impl Interval {
    fn new(start: i64, end: i64) -> Self {
        Self {
            start,
            end,
        }
    }

    fn len(&self) -> i64 {
        self.end - self.start
    }

    #[allow(unused)]
    fn overlap(&self, other: &Self) -> bool {
        self.overlap_start(&other) || self.overlap_end(&other)
    }

    #[allow(unused)]
    fn overlap_start(&self, other: &Self) -> bool {
        self.start <= other.end
    }

    fn overlap_end(&self, other: &Self) -> bool {
        other.start <= self.end
    }

    #[allow(unused)]
    fn merge(&mut self, other: &Self) {
        self.merge_start(other);
        self.merge_end(other);
    }

    #[allow(unused)]
    fn merge_start(&mut self, other: &Self) {
        self.start = self.start.min(other.start);
    }

    fn merge_end(&mut self, other: &Self) {
        self.end = self.end.max(other.end);
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