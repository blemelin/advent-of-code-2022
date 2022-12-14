use util::{FromLine, FromLines, read, run};

use crate::util::Vec2;

mod util;

fn main() {
    let (t0, input) = run(|| read::<Input, _>("inputs/day14.txt"));
    let (t1, p1) = run(|| input.part_1());
    let (t2, p2) = run(|| input.part_2());

    println!("Part 1 : {}", p1);
    println!("Part 2 : {}", p2);
    println!("Time : {} ns", (t0 + t1 + t2).as_nanos());
}

#[derive(Debug)]
struct Input {
    simulation: SimulationGrid,
}

impl Input {
    fn part_1(&self) -> u64 {
        let mut simulation = self.simulation.clone();

        println!("Before : ");
        for y in simulation.min_y..=simulation.max_y {
            for x in simulation.min_x..=simulation.max_x {
                match simulation.cell(&vec2!(x,y)) {
                    SimulationCell::Empty => print!("."),
                    SimulationCell::Sand => print!("S"),
                    SimulationCell::Rock => print!("R"),
                }
            }
            println!();
        }

        let mut count = 0;
        while simulation.add_sand(&vec2!(500, 0)) {
            count += 1;
        }

        println!("After : ");
        for y in simulation.min_y..=simulation.max_y {
            for x in simulation.min_x..=simulation.max_x {
                match simulation.cell(&vec2!(x,y)) {
                    SimulationCell::Empty => print!("."),
                    SimulationCell::Sand => print!("S"),
                    SimulationCell::Rock => print!("R"),
                }
            }
            println!();
        }


        count
    }

    fn part_2(&self) -> u64 {
        0
    }
}

type Position = Vec2<usize>;

#[derive(Debug, Clone)]
struct SimulationGrid {
    // Shape [Y][X]
    grid: Vec<Vec<SimulationCell>>,
    min_x: usize,
    max_x: usize,
    min_y: usize,
    max_y: usize,
    floor_y: Option<usize>,
}

impl SimulationGrid {
    fn new(mut min_x: usize, mut max_x: usize, min_y: usize, mut max_y: usize) -> Self {
        // Grid must be larger to accommodate the sand.
        //  - Two more spaces at the bottom.
        //  - One more space to the left
        //  - One more to the right.
        println!("{}x{}", min_x, max_x);
        min_x -= 1;
        max_x += 1;
        println!("{}x{}", min_x, max_x);
        //max_y += 2;


        let width = max_x - min_x + 1;
        let height = max_y - min_y + 1;

        Self {
            grid: vec![vec![SimulationCell::Empty; width]; height],
            min_x,
            max_x,
            min_y,
            max_y,
            floor_y: None,
        }
    }

    fn cell(&self, position: &Position) -> &SimulationCell {
        //println!("Pos : {:?}", position);
        //println!("MinX : {:?}", self.min_x);
        //println!("MinY : {:?}", self.min_y);
        let x = position.x() - self.min_x;
        let y = position.y() - self.min_y;
        &self.grid[y][x]
    }

    fn cell_mut(&mut self, position: &Position) -> &mut SimulationCell {
        let x = position.x() - self.min_x - 1;
        let y = position.y() - self.min_y - 1;
        &mut self.grid[y][x]
    }

    fn is_in_bounds(&self, position: &Position) -> bool {
        position.x() >= self.min_x - 1 &&
            position.x() <= self.max_x + 1 &&
            position.y() >= self.min_y &&
            position.y() <= self.max_y
    }

    fn add_rock_formation(&mut self, formation: &RockFormation) {
        for position in formation.positions() {
            *self.cell_mut(&position) = SimulationCell::Rock;
        }
    }

    fn add_sand(&mut self, position: &Position) -> bool {
        // println!("Add sand");
        let mut current = *position;
        loop {
            let down_pos = current + vec2!(0isize, 1isize);
            let down_left_pos = current + vec2!(-1isize, 1isize);
            let down_right_pos = current + vec2!(1isize, 1isize);

            let down = down_pos.filter(|it| self.is_in_bounds(it)).map(|it| self.cell(&it));
            let down_left = down_left_pos.filter(|it| self.is_in_bounds(it)).map(|it| self.cell(&it));
            let down_right = down_right_pos.filter(|it| self.is_in_bounds(it)).map(|it| self.cell(&it));

            // println!("Currently at : {:?}", current);
            match down {
                Some(SimulationCell::Empty) => {
                    // Falling.
                    // println!("Fall down : {:?}", down_pos);
                    current = down_pos.expect("position should exist in simulation");
                    continue;
                }
                Some(SimulationCell::Rock) | Some(SimulationCell::Sand) => {
                    // Blocked.
                    // println!("Blocked down : {:?}", down_pos);
                    match down_left {
                        Some(SimulationCell::Empty) => {
                            // Falling.
                            // println!("Fall down left : {:?}", down_left_pos);
                            current = down_left_pos.expect("position should exist in simulation");
                            continue;
                        }
                        Some(SimulationCell::Rock) | Some(SimulationCell::Sand) => {
                            // Blocked.
                            // println!("Blocked down left : {:?}", down_left_pos);
                            match down_right {
                                Some(SimulationCell::Empty) => {
                                    // Falling.
                                    // println!("Fall down right : {:?}", down_right_pos);
                                    current = down_right_pos.expect("position should exist in simulation");
                                    continue;
                                }
                                Some(SimulationCell::Rock) | Some(SimulationCell::Sand) => {
                                    // Blocked. Cannot move anymore.
                                    // println!("Blocked down right : {:?}", down_right_pos);
                                    // println!("Settles at : {:?}", current);
                                    *self.cell_mut(&current) = SimulationCell::Sand;
                                    return true;
                                }
                                None => { panic!("sand should never land out of bounds to the right"); }
                            }
                        }
                        None => { panic!("sand should never land out of bounds to the left"); }
                    }
                }
                None => {
                    // Out of bounds.
                    return false;
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
enum SimulationCell {
    Empty,
    Rock,
    Sand,
}

#[derive(Debug)]
struct RockFormation(Vec<Position>);

impl RockFormation {
    fn positions(&self) -> impl Iterator<Item=&Position> + '_ {
        self.0.iter()
    }

    fn min_x(&self) -> usize {
        self.positions().map(|it| it.x()).min().unwrap_or(0)
    }

    fn max_x(&self) -> usize {
        self.positions().map(|it| it.x()).max().unwrap_or(0)
    }

    #[allow(unused)]
    fn min_y(&self) -> usize {
        self.positions().map(|it| it.y()).min().unwrap_or(0)
    }

    fn max_y(&self) -> usize {
        self.positions().map(|it| it.y()).max().unwrap_or(0)
    }
}

impl FromLines for Input {
    fn from_lines(lines: &[&str]) -> Self {
        let simulation = SimulationGrid::from_lines(lines);

        Self {
            simulation
        }
    }
}

impl FromLines for SimulationGrid {
    fn from_lines(lines: &[&str]) -> Self {
        let formations: Vec<RockFormation> = lines.iter().map(line_to!(RockFormation)).collect();
        let mut min_x = formations.iter().map(|it| it.min_x()).min().unwrap_or(0);
        let mut max_x = formations.iter().map(|it| it.max_x()).max().unwrap_or(0);
        let mut min_y = 0; // Always 0.
        let mut max_y = formations.iter().map(|it| it.max_y()).max().unwrap_or(0);

        let mut simulation = Self::new(min_x, max_x, min_y, max_y);
        for formation in formations {
            simulation.add_rock_formation(&formation);
        }

        simulation
    }
}

impl FromLine for RockFormation {
    fn from_line(line: &str) -> Self {
        // Read rock formations as lines.
        let lines: Vec<Vec2<usize>> = line.split(" -> ").map(|it| {
            let (lhs, rhs) = it.split_once(',').expect("point should have two coordinates");
            vec2!(usize::from_line(lhs), usize::from_line(rhs))
        }).collect();

        // Each formation must have two points.
        if lines.len() < 2 { panic!("rock formation should have at least two points"); }

        // Convert each rock formation line to a set of points.
        let mut points = Vec::new();
        for (start, end) in lines.iter().zip(lines[1..].iter()).map(|(start, end)| (*start, *end)) {
            let direction = {
                let x = if start.x() < end.x() { 1isize } else if start.x() > end.x() { -1isize } else { 0 };
                let y = if start.y() < end.y() { 1isize } else if start.y() > end.y() { -1isize } else { 0 };
                vec2!(x, y)
            };
            let mut current = start;
            loop {
                points.push(vec2!(current.x() as usize, current.y() as usize));
                if current == end { break; }
                current = (current + direction).expect("rock position should be positive at all times");
            }
        }

        Self(points)
    }
}
