use std::fmt;
use std::iter::successors;

use util::{FromLine, FromLines, read, run, Vec2};

mod util;

// Optimisation opportunity : the grid is way too big for the problem. It's possible to shrink it.
// The required height is equal to the maximum rock Y position. The required width is equal
// to height + height - 1. The problem here lies in the offset that this creates, and I've spent
// already too much time into this.

const SAND_SOURCE: Position = vec2!(500, 0);
const SIMULATION_WIDTH: usize = SAND_SOURCE.x() * 2 + 1;
const SIMULATION_HEIGHT: usize = SIMULATION_WIDTH;

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
    rock_formations: Vec<RockFormation>,
}

impl Input {
    fn part_1(&self) -> usize {
        let mut simulation = Simulation::<SIMULATION_WIDTH, SIMULATION_HEIGHT>::new();

        // Add rock formations
        for formation in &self.rock_formations {
            for position in formation.positions() {
                *simulation.cell_mut(&position) = Cell::Rock;
            }
        }

        // Run simulation
        simulation.flow(&SAND_SOURCE)
    }

    fn part_2(&self) -> usize {
        let mut simulation = Simulation::<SIMULATION_WIDTH, SIMULATION_HEIGHT>::new();

        // Add rock formations
        for formation in &self.rock_formations {
            for position in formation.positions() {
                *simulation.cell_mut(&position) = Cell::Rock;
            }
        }

        // Add floor
        let floor_height = self.rock_formations.iter().map(|it| it.max_height()).max().unwrap_or(0) + 2;
        for x in 0..SIMULATION_WIDTH {
            *simulation.cell_mut(&vec2!(x,floor_height)) = Cell::Rock;
        }

        // Run simulation
        simulation.fill(&SAND_SOURCE)
    }

    // Export the simulation to a file.
    #[allow(unused)]
    fn export_simulation<const W: usize, const H: usize>(path: &str, simulation: &Simulation<W, H>) {
        std::fs::write(path, format!("{}", &simulation)).unwrap();
    }
}

type Position = Vec2<usize>;
type Direction = Vec2<isize>;

#[derive(Debug)]
struct Simulation<const W: usize, const H: usize> {
    grid: [[Cell; W]; H],
}

impl<const W: usize, const H: usize> Simulation<W, H> {
    fn new() -> Self {
        Self {
            grid: [[Cell::Empty; W]; H]
        }
    }

    fn cell(&self, position: &Position) -> &Cell {
        &self.grid[position.y()][position.x()]
    }

    fn cell_mut(&mut self, position: &Position) -> &mut Cell {
        &mut self.grid[position.y()][position.x()]
    }

    fn is_in_bounds(&self, position: &Position) -> bool {
        position.x() < W && position.y() < H
    }

    fn flow(&mut self, start: &Position) -> usize {
        // Count of sand blocks added.
        let mut count = 0;

        'flow: loop {
            // Current position
            let mut current_pos = *start;

            // Abort if current position is occupied. Sand cannot settle anymore.
            if *self.cell(&current_pos) != Cell::Empty { break 'flow; }

            // Directions where the sand can flow, by priority. Down, down left and down right.
            const DIRECTIONS: [Direction; 3] = [vec2!(0isize, 1isize), vec2!(-1isize, 1isize), vec2!(1isize, 1isize)];

            'step: loop {
                // Fall.
                'direction: for direction in &DIRECTIONS {
                    let next_pos = current_pos + *direction;
                    let next_cell = next_pos.filter(|it| self.is_in_bounds(it)).map(|it| self.cell(&it));

                    match next_cell {
                        // Empty. Fall.
                        Some(Cell::Empty) => {
                            current_pos = next_pos.expect("position should exist since cell exist");
                            continue 'step;
                        }
                        // Blocked. Check next direction.
                        Some(Cell::Rock) | Some(Cell::Sand) => {
                            continue 'direction;
                        }
                        // Out of bounds. Can't settle. Abort.
                        None => {
                            break 'flow;
                        }
                    }
                }

                // Can't fall anymore. Settle.
                *self.cell_mut(&current_pos) = Cell::Sand;
                count += 1;
                continue 'flow;
            }
        }
        count
    }

    fn fill(&mut self, start: &Position) -> usize {
        // Put the first sand block.
        *self.cell_mut(&start) = Cell::Sand;

        // Count of sand blocks added. First one is already done.
        let mut count = 1;

        // Make the sand flow, line by line.
        // Start at the line where the starting sand block is.
        let mut depth = 0;
        for y in start.y()..H - 1 {
            // Last count.
            let last_count = count;

            // For each sand block in current line, try to put a other sand block in each direction.
            // Simulation will end up in a pyramid shape. So no need to simulate the whole line.
            for x in (start.x() - depth)..=(start.x() + depth) {
                if *self.cell(&vec2!(x,y)) == Cell::Sand {
                    let other_y = y + 1;
                    for other_x in [x, x - 1, x + 1] {
                        let position = vec2!(other_x, other_y);
                        if *self.cell(&position) == Cell::Empty {
                            *self.cell_mut(&position) = Cell::Sand;
                            count += 1;
                        }
                    }
                }
            }

            // Abort if no sand block was added.
            if last_count == count { break; }

            // Increase depth.
            depth += 1;
        }
        count
    }
}

impl<const W: usize, const H: usize> fmt::Display for Simulation<W, H> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..W {
            for x in 0..H {
                let position = vec2!(x, y);
                if position == SAND_SOURCE {
                    write!(f, "+")?;
                } else {
                    write!(f, "{}", self.cell(&position))?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Cell {
    Empty,
    Rock,
    Sand,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Cell::Empty => write!(f, "."),
            Cell::Sand => write!(f, "o"),
            Cell::Rock => write!(f, "#"),
        }
    }
}

#[derive(Debug)]
struct RockFormation(Vec<Position>);

impl RockFormation {
    fn positions(&self) -> impl Iterator<Item=Position> + '_ {
        self.0.iter().zip(self.0[1..].iter()).map(|(start, end)| {
            let direction = {
                let x = if start.x() < end.x() { 1isize } else if start.x() > end.x() { -1isize } else { 0 };
                let y = if start.y() < end.y() { 1isize } else if start.y() > end.y() { -1isize } else { 0 };
                vec2!(x, y)
            };

            successors(Some(*start), move |position| {
                if position != end {
                    Some((*position + direction).expect("rock position should be positive at all times"))
                } else {
                    None
                }
            })
        }).flatten()
    }

    fn max_height(&self) -> usize {
        self.positions().map(|it| it.y()).max().unwrap_or(0)
    }
}

impl FromLines for Input {
    fn from_lines(lines: &[&str]) -> Self {
        let rock_formations = lines.iter().map(line_to!(RockFormation)).collect();

        Self {
            rock_formations
        }
    }
}

impl FromLine for RockFormation {
    fn from_line(line: &str) -> Self {
        let points: Vec<Position> = line.split(" -> ").map(line_to!(Vec2<usize>)).collect();
        if points.len() < 2 { panic!("rock formation should have at least two points"); }

        Self(points)
    }
}
