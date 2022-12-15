use std::fmt;
use std::iter::successors;

use util::{FromLine, FromLines, read, run, Vec2};

mod util;

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
    fn part_1(&self) -> u64 {
        let mut simulation = Simulation::<SIMULATION_WIDTH, SIMULATION_HEIGHT>::new();

        // Add rock formations
        for formation in &self.rock_formations {
            for position in formation.positions() {
                *simulation.cell_mut(&position) = Cell::Rock;
            }
        }

        // Run simulation
        let mut count = 0;
        while simulation.add_sand(&SAND_SOURCE) {
            count += 1;
        }
        count
    }

    fn part_2(&self) -> u64 {
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
        let mut count = 0;
        while simulation.add_sand(&SAND_SOURCE) {
            count += 1;
        }
        count
    }

    // Export the simulation to a file.
    #[allow(unused)]
    fn export_simulation<const W: usize, const H: usize>(path: &str, simulation: &Simulation<W, H>) {
        std::fs::write(path, format!("{}", &simulation)).unwrap();
    }
}

type Position = Vec2<usize>;
type Direction = Vec2<isize>;

#[derive(Debug, Clone)]
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

    fn add_sand(&mut self, position: &Position) -> bool {
        // Abort if position occupied. Sand cannot flow and settle.
        if *self.cell(position) != Cell::Empty { return false; }

        // Current position
        let mut current_pos = *position;

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
                    // Out of bounds. Can settle. Abort.
                    None => {
                        return false;
                    }
                }
            }

            // Can't fall anymore. Settle.
            *self.cell_mut(&current_pos) = Cell::Sand;
            return true;
        }
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
