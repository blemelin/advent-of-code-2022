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
    fn execute(&self) -> (usize, usize) {
        let path_map = self.heightmap.path_map(self.end);

        // Part 1
        let path_to_top_len = path_map
            .path(self.start)
            .map(|it| it.len())
            .unwrap_or(0);

        // Part 2
        let lowest = self.heightmap.lowest();
        let path_to_bottom_len = self.heightmap
            .positions()
            .filter(|it| self.heightmap.node(*it) == lowest)
            .filter_map(|it| path_map.path(it).map(|it| it.len()))
            .min()
            .unwrap_or(0);

        (path_to_top_len, path_to_bottom_len)
    }
}

type Position = Vec2<usize>;
type Height = u64;

#[derive(Debug)]
struct Heightmap {
    nodes: Vec<Vec<Height>>,
    width: usize,
    height: usize,
}

impl Heightmap {
    fn node(&self, position: Position) -> Height {
        self.nodes[position.y()][position.x()]
    }

    fn nodes(&self) -> impl Iterator<Item=Height> + '_ {
        self.nodes.iter().map(|it| it.iter().cloned()).flatten()
    }

    fn positions(&self) -> impl Iterator<Item=Position> + '_ {
        (0..self.width).into_iter().map(|x| (0..self.height).into_iter().map(move |y| vec2!(x, y))).flatten()
    }

    fn lowest(&self) -> Height {
        self.nodes().min().unwrap_or(0)
    }

    fn neighbours(&self, position: Position) -> Vec<Position> {
        let height = self.node(position);
        let mut neighbours = Vec::with_capacity(4);

        // Top
        if position.y() > 0 {
            let other_position = vec2!(position.x(), position.y() - 1);
            let other_height = self.node(other_position);
            if other_height >= height || height.checked_sub(other_height).filter(|it| *it == 1).is_some(){
                neighbours.push(other_position);
            }
        }

        // Bottom
        if position.y() < self.height - 1 {
            let other_position = vec2!(position.x(), position.y() + 1);
            let other_height = self.node(other_position);
            if other_height >= height || height.checked_sub(other_height).filter(|it| *it == 1).is_some(){
                neighbours.push(other_position);
            }
        }

        // Left
        if position.x() > 0 {
            let other_position = vec2!(position.x() - 1, position.y());
            let other_height = self.node(other_position);
            if other_height >= height || height.checked_sub(other_height).filter(|it| *it == 1).is_some(){
                neighbours.push(other_position);
            }
        }

        // Right
        if position.x() < self.width - 1 {
            let other_position = vec2!(position.x() + 1, position.y());
            let other_height = self.node(other_position);
            if other_height >= height || height.checked_sub(other_height).filter(|it| *it == 1).is_some(){
                neighbours.push(other_position);
            }
        }

        neighbours
    }

    fn path_map(&self, end: Position) -> PathMap {
        // Unvisited nodes.
        let mut unvisited: Vec<Position> = self.positions().collect();

        // Distance from end position to every other node. Default to infinity.
        let mut distances: HashMap<Position, u64> = unvisited.iter().map(|it| (*it, u64::MAX)).collect();
        *distances.get_mut(&end).expect("end should exist in nodes") = 0;

        // Previous nodes.
        let mut previous: HashMap<Position, Option<Position>> = unvisited.iter().map(|it| (*it, None)).collect();

        // As long as there is unvisited nodes.
        'top: while !unvisited.is_empty() {

            // Find closest unvisited nodes.
            if let Some((i, current, distance)) = unvisited.iter()
                .enumerate()
                .map(|(i, position)| (i, *position, *distances.get(position).expect("distances should have all nodes")))
                .min_by_key(|(_, _, distance)| *distance)
                .filter(|(_, _, distance)| *distance < u64::MAX) {

                // Found a reachable node. Mark as visited.
                unvisited.remove(i);

                // For all unvisited neighbours of this node.
                for neighbour in self.neighbours(current).iter().filter(|it| unvisited.contains(it)) {
                    // Current know distance to this node.
                    let neighbour_distance = distances.get_mut(&neighbour).expect("distances should have all nodes");

                    // Distance from this node to this neighbour is always one more.
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

        PathMap {
            previous,
            end,
        }
    }
}

#[derive(Debug)]
struct Path(Vec<Position>);

impl Path {
    fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug)]
struct PathMap {
    previous: HashMap<Position, Option<Position>>,
    end: Position,
}

impl PathMap {
    fn path(&self, start: Position) -> Option<Path> {
        let mut path = Vec::new();

        let mut current = self.previous.get(&start).and_then(|it| *it);
        while let Some(step) = current {
            // Add step to path
            path.push(step);

            // Check if target reached.
            if current == Some(self.end) {
                return Some(Path(path));
            }

            // Go next step.
            current = self.previous.get(&step).and_then(|it| *it);
        }

        None
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
                nodes,
                width,
                height,
            },
            start: start.expect("input should have a start position"),
            end: end.expect("input should have a end position"),
        }
    }
}