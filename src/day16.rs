use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use util::{FromLine, FromLines, read, run};

mod util;

const TIME_ALLOWED: u64 = 30;
const TIME_PER_ACTION: u64 = 1;

fn main() {
    let (t0, input) = run(|| read::<Input, _>("inputs/day16.txt"));
    let (t1, p1) = run(|| input.part_1());
    let (t2, p2) = run(|| input.part_2());

    println!("Part 1 : {}", p1);
    println!("Part 2 : {}", p2);
    println!("Time : {} ns", (t0 + t1 + t2).as_nanos());
}

#[derive(Debug)]
struct Input {
    system: PressureSystem,
}

impl Input {
    fn part_1(&self) -> u64 {
        self.system.search(Id(['A', 'A']));
        0
    }

    fn part_2(&self) -> u64 {
        0
    }
}

impl FromLines for Input {
    fn from_lines(lines: &[&str]) -> Self {
        let system = PressureSystem::from_lines(lines);

        Self {
            system
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Id([char; 2]);

#[derive(Debug)]
struct PressureSystem {
    valves: HashMap<Id, Valve>,
}

impl PressureSystem {
    fn search(&self, end: Id) -> PathSearch {
        #[derive(Debug, Eq, PartialEq)]
        struct Node(Id, u64);

        // Reverse order. Smaller cost gets prioritised first.
        impl Ord for Node {
            fn cmp(&self, other: &Self) -> Ordering {
                other.1.cmp(&self.1)
            }
        }

        impl PartialOrd for Node {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        // All valves.
        let positions: Vec<Id> = self.valves.keys().cloned().collect();

        // Visited valves.
        let mut visited: HashSet<Id> = HashSet::with_capacity(positions.len());

        // Valves to visit.
        let mut visit_queue: BinaryHeap<Node> = BinaryHeap::with_capacity(positions.len());
        visit_queue.push(Node(end, 0));

        // Distance from end to every other valves. Defaults to infinity.
        // Distance to end is 0. If end doesn't exist in distances, nothing happens.
        let mut distances: HashMap<Id, u64> = positions.iter().map(|it| (*it, u64::MAX)).collect();
        distances.entry(end).and_modify(|distance| *distance = 0);

        // Map a valve to the next valve to go to get closer to the end. Default to None.
        let mut previous: HashMap<Id, Option<Id>> = positions.iter().map(|it| (*it, None)).collect();

        // As long as there is valves to visit.
        while let Some(Node(current, distance)) = visit_queue.pop() {
            // For all neighbours of this position.
            for neighbour in &self.valves.get(&current).expect("node should exist").tunnels {
                // Mark position as visited, or skip if already visited.
                if visited.insert(*neighbour) {
                    // Distance from current to this neighbour is current distance plus one.
                    let new_distance = distance + 1;

                    // Visit this neighbour neighbour's next iteration.
                    visit_queue.push(Node(*neighbour, new_distance));

                    // Update known distance to this position, if smaller.
                    let neighbour_distance = distances.get_mut(&neighbour).expect("distances should have all nodes");
                    if new_distance < *neighbour_distance {
                        let neighbour_previous = previous.get_mut(&neighbour).expect("previous should have all nodes");

                        *neighbour_distance = new_distance;
                        *neighbour_previous = Some(current);
                    }
                }
            }
        }

        PathSearch
    }
}

#[derive(Debug)]
struct PathSearch;

#[derive(Debug)]
struct Valve {
    id: Id,
    flow_rate: u64,
    tunnels: Vec<Id>,
}

impl FromLine for Id {
    fn from_line(line: &str) -> Self {
        let mut parts = line.chars();
        Self([
            parts.next().expect("id should have 2 characters"),
            parts.next().expect("id should have 2 characters")
        ])
    }
}

impl FromLines for PressureSystem {
    fn from_lines(lines: &[&str]) -> Self {
        let valves = lines.iter().map(line_to!(Valve)).map(|it| (it.id, it)).collect();

        Self {
            valves
        }
    }
}

impl FromLine for Valve {
    fn from_line(line: &str) -> Self {
        let mut parts = line.split(' ');
        let id = parts.nth(1).expect("valve should have an id");
        let id = Id::from_line(id);

        let flow_rate = parts.nth(2).expect("valve should have a flow_rate");
        let flow_rate = u64::from_line(&flow_rate[5..flow_rate.len() - 1]);

        let tunnels = parts.skip(4);
        let tunnels = tunnels.map(line_to!(Id)).collect();

        Self {
            id,
            flow_rate,
            tunnels,
        }
    }
}