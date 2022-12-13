use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::time::Instant;

use util::{FromLines, read, Vec2};

mod util;

fn main() {
    let input: Input = read("inputs/day12.txt");
    let (part_1, part_2) = input.execute();
    println!("Part 1 : {}", part_1);
    println!("Part 2 : {}", part_2);
}

#[derive(Debug)]
struct Input {
    heightmap: Heightmap,
    start: Position,
    end: Position,
}

impl Input {
    fn execute(&self) -> (u64, u64) {
        let start = Instant::now();
        let path_search = self.heightmap.search(self.end);
        let delta = Instant::now() - start;
        println!("Search took {}s", delta.as_secs_f64());

        // Part 1
        let distance_to_top = path_search
            .distance(self.start)
            .unwrap_or(0);

        // Part 2
        let lowest = self.heightmap.lowest();
        let shortest_distance_to_top = self.heightmap
            .positions()
            .filter(|it| self.heightmap.height(*it) == lowest)
            .filter_map(|it| path_search.distance(it))
            .min()
            .unwrap_or(0);

        (distance_to_top, shortest_distance_to_top)
    }
}

type Position = Vec2<usize>;
type Height = u64;

#[derive(Debug)]
struct Heightmap {
    heights: Vec<Vec<Height>>,
    width: usize,
    height: usize,
}

impl Heightmap {
    fn height(&self, position: Position) -> Height {
        self.heights[position.y()][position.x()]
    }

    fn heights(&self) -> impl Iterator<Item=Height> + '_ {
        self.heights.iter().map(|it| it.iter().cloned()).flatten()
    }

    fn lowest(&self) -> Height {
        self.heights().min().unwrap_or(0)
    }

    fn positions(&self) -> impl Iterator<Item=Position> + '_ {
        (0..self.width).into_iter().map(|x| (0..self.height).into_iter().map(move |y| vec2!(x, y))).flatten()
    }

    fn neighbours(&self, position: Position) -> impl Iterator<Item=Position> + '_ {
        let height = self.height(position);
        let mut neighbours = Vec::with_capacity(4);

        // Top
        if position.y() > 0 {
            let other_position = vec2!(position.x(), position.y() - 1);
            let other_height = self.height(other_position);
            if height <= other_height || height == other_height + 1 {
                neighbours.push(other_position);
            }
        }

        // Bottom
        if position.y() < self.height - 1 {
            let other_position = vec2!(position.x(), position.y() + 1);
            let other_height = self.height(other_position);
            if height <= other_height || height == other_height + 1 {
                neighbours.push(other_position);
            }
        }

        // Left
        if position.x() > 0 {
            let other_position = vec2!(position.x() - 1, position.y());
            let other_height = self.height(other_position);
            if height <= other_height || height == other_height + 1 {
                neighbours.push(other_position);
            }
        }

        // Right
        if position.x() < self.width - 1 {
            let other_position = vec2!(position.x() + 1, position.y());
            let other_height = self.height(other_position);
            if height <= other_height || height == other_height + 1 {
                neighbours.push(other_position);
            }
        }

        neighbours.into_iter()
    }

    fn search(&self, end: Position) -> PathSearch {
        #[derive(Debug, Eq, PartialEq)]
        struct Node(Position, u64);

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

        // All positions.
        let positions: Vec<Position> = self.positions().collect();

        // Visited positions.
        let mut visited: HashSet<Position> = HashSet::with_capacity(self.height * self.width);

        // Positions to visit.
        let mut visit_queue: BinaryHeap<Node> = BinaryHeap::with_capacity(self.height * self.width);
        visit_queue.push(Node(end, 0));

        // Distance from end to every other positions. Defaults to infinity.
        // Distance to end is 0. If end doesn't exist in distances, nothing happens.
        let mut distances: HashMap<Position, u64> = positions.iter().map(|it| (*it, u64::MAX)).collect();
        distances.entry(end).and_modify(|distance| *distance = 0);

        // Map a position to the next position to go to get closer to the end. Default to None.
        let mut previous: HashMap<Position, Option<Position>> = positions.iter().map(|it| (*it, None)).collect();

        // As long as there is positions to visit.
        while let Some(Node(current, distance)) = visit_queue.pop() {
            // For all neighbours of this position.
            for neighbour in self.neighbours(current) {
                // Mark position as visited, or skip if already visited.
                if visited.insert(neighbour) {
                    // Distance from current to this neighbour is current distance plus one.
                    let new_distance = distance + 1;

                    // Visit this neighbour neighbour's next iteration.
                    visit_queue.push(Node(neighbour, new_distance));

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

        PathSearch {
            distances,
            previous,
            end,
        }
    }
}

#[derive(Debug)]
struct PathSearch {
    distances: HashMap<Position, u64>,
    previous: HashMap<Position, Option<Position>>,
    end: Position,
}

impl PathSearch {
    #[allow(unused)]
    fn path(&self, start: Position) -> Option<Vec<Position>> {
        let mut path = Vec::new();

        let mut current = self.previous.get(&start).and_then(|it| *it);
        while let Some(step) = current {
            // Add step to path
            path.push(step);

            // Check if target reached.
            if current == Some(self.end) {
                return Some(path);
            }

            // Go next step.
            current = self.previous.get(&step).and_then(|it| *it);
        }

        None
    }

    fn distance(&self, start: Position) -> Option<u64> {
        self.distances.get(&start).cloned()
    }
}

impl FromLines for Input {
    fn from_lines(lines: &[&str]) -> Self {
        let width = lines.get(0).map(|it| it.len()).unwrap_or(0);
        let height = lines.len();
        let mut nodes = vec![vec![0; width]; height];

        let mut start = None;
        let mut end = None;

        for (y, row) in lines.iter().enumerate() {
            for (x, node) in row.chars().enumerate() {
                if node == 'S' {
                    nodes[y][x] = 0;
                    start = Some(vec2!(x, y));
                } else if node == 'E' {
                    nodes[y][x] = (b'z' - b'a') as u64;
                    end = Some(vec2!(x, y));
                } else {
                    nodes[y][x] = (node as u8 - b'a') as u64;
                }
            }
        }

        Self {
            heightmap: Heightmap {
                heights: nodes,
                width,
                height,
            },
            start: start.expect("input should have a start position"),
            end: end.expect("input should have a end position"),
        }
    }
}