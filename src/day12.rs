use std::collections::HashMap;

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
        let path_search = self.heightmap.search(self.end);

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
        // Unvisited positions.
        // Note : Seems fastest to use a Vec than a HashSet, probably because the data is not very big.
        let mut unvisited: Vec<Position> = self.positions().collect();

        // Distance from end to every other positions. Defaults to infinity.
        let mut distances: HashMap<Position, u64> = unvisited.iter().map(|it| (*it, u64::MAX)).collect();

        // Distance to end is 0. If end doesn't exist in distances, nothing happens.
        distances.entry(end).and_modify(|distance| *distance = 0);

        // Map a position to the next position to go to get closer to the end. Default to None.
        let mut previous: HashMap<Position, Option<Position>> = unvisited.iter().map(|it| (*it, None)).collect();

        // As long as there is unvisited positions.
        'top: while !unvisited.is_empty() {

            // Find closest unvisited position.
            // If we get infinity, that means the other positions are unreachable.
            if let Some((i, current, distance)) = unvisited.iter()
                .enumerate()
                .map(|(i, position)| (i, *position, *distances.get(position).expect("distances should have all nodes")))
                .min_by_key(|(_, _, distance)| *distance)
                .filter(|(_, _, distance)| *distance < u64::MAX) {

                // Found a reachable position. Mark as visited.
                unvisited.swap_remove(i);

                // For all unvisited neighbours of this position.
                for neighbour in self.neighbours(current) {
                    // Current known distance to this position.
                    let neighbour_distance = distances.get_mut(&neighbour).expect("distances should have all nodes");

                    // Distance from this position to this neighbour is current distance plus one.
                    let new_distance = distance + 1;

                    if new_distance < *neighbour_distance {
                        let neighbour_previous = previous.get_mut(&neighbour).expect("previous should have all nodes");

                        *neighbour_distance = new_distance;
                        *neighbour_previous = Some(current);
                    }
                }
            } else {
                // No more reachable nodes.
                break 'top;
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