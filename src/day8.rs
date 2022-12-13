use std::iter::successors;

use util::{FromChar, FromLines, read, Vec2};

mod util;

fn main() {
    let input: Input = read("inputs/day8.txt");
    println!("Part 1 : {}", input.part_1());
    println!("Part 2 : {}", input.part_2());
}

#[derive(Debug)]
struct Input {
    forest: Forest,
}

impl Input {
    fn part_1(&self) -> usize {
        self.forest.count_visible()
    }

    fn part_2(&self) -> usize {
        self.forest.best_scenic_score().unwrap_or(0)
    }
}

type Position = Vec2<usize>;
type Direction = Vec2<isize>;

#[derive(Debug)]
struct Forest {
    trees: Vec<Vec<Tree>>,
    width: usize,
    height: usize,
}

impl Forest {
    fn tree(&self, position: Position) -> Tree {
        self.trees[position.y()][position.x()]
    }

    fn trees_in(&self, position: Position, direction: Direction) -> impl Iterator<Item=Tree> + '_ {
        let is_in_bounds = |position: &Position| position.x() < self.width && position.y() < self.height;

        successors((position + direction).filter(is_in_bounds), move |position| {
            (*position + direction).filter(is_in_bounds)
        }).map(|position| {
            self.tree(position)
        })
    }

    fn is_tree_visible(&self, position: Position) -> bool {
        let tree = self.tree(position);

        // Top
        for other_tree in self.trees_in(position, vec2!(0, -1)) {
            if other_tree >= tree { return false; }
        }
        // Bottom
        for other_tree in self.trees_in(position, vec2!(0, 1)) {
            if other_tree >= tree { return false; }
        }
        // Left
        for other_tree in self.trees_in(position, vec2!(-1, 0)) {
            if other_tree >= tree { return false; }
        }
        // Right
        for other_tree in self.trees_in(position, vec2!(1, 0)) {
            if other_tree >= tree { return false; }
        }

        true
    }

    fn count_visible(&self) -> usize {
        // All trees around the forest are visible. No need to count them.
        let mut count = self.width * 2 + self.height * 2 - 4;

        // Count other trees.
        for x in 1..self.width - 1 {
            for y in 1..self.height - 1 {
                if self.is_tree_visible(vec2!(x, y)) { count += 1; }
            }
        }
        count
    }

    fn tree_scenic_score(&self, position: Position) -> usize {
        let tree = self.tree(position);

        // Top
        let mut top_score = 0;
        for other_tree in self.trees_in(position, vec2!(0, -1)) {
            top_score += 1;
            if other_tree >= tree { break; }
        }
        // Bottom
        let mut bottom_score = 0;
        for other_tree in self.trees_in(position, vec2!(0, 1)) {
            bottom_score += 1;
            if other_tree >= tree { break; }
        }
        // Left
        let mut left_score = 0;
        for other_tree in self.trees_in(position, vec2!(-1, 0)) {
            left_score += 1;
            if other_tree >= tree { break; }
        }
        // Right
        let mut right_score = 0;
        for other_tree in self.trees_in(position, vec2!(1, 0)) {
            right_score += 1;
            if other_tree >= tree { break; }
        }

        top_score * bottom_score * left_score * right_score
    }

    fn best_scenic_score(&self) -> Option<usize> {
        let mut best = None;
        for x in 0..self.width {
            for y in 0..self.height {
                let current = Some(self.tree_scenic_score(vec2!(x, y)));
                if current > best { best = current; }
            }
        }
        best
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
struct Tree(u8);

impl FromLines for Input {
    fn from_lines(lines: &[&str]) -> Self {
        let forest = Forest::from_lines(lines);

        Self {
            forest
        }
    }
}

impl FromLines for Forest {
    fn from_lines(lines: &[&str]) -> Self {
        let width = lines.get(0).map(|it| it.len()).unwrap_or(0);
        let height = lines.len();
        let mut trees = vec![vec![Tree(0); width]; height];

        for (y, row) in lines.iter().enumerate() {
            for (x, tree) in row.chars().enumerate() {
                trees[x][y] = Tree::from_char(tree);
            }
        }

        Self {
            trees,
            width,
            height,
        }
    }
}

impl FromChar for Tree {
    fn from_char(char: char) -> Self {
        let value = char.to_digit(10).expect("tree value should between 0 and 9") as u8;

        Self(value)
    }
}