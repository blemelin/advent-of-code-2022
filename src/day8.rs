use util::{Bounds, FromChar, FromLines, read, Vec2};

mod util;

fn main() {
    // Read data
    let Data(forest) = read("inputs/day8.txt");

    // Part 1
    let result = forest.count_visible();
    println!("Part 1 : {}", result);

    // Part 2
    let result = forest.best_scenic_score();
    println!("Part 2 : {}", result);
}

#[derive(Debug)]
struct Data(Forest);

impl FromLines for Data {
    fn from_lines(lines: &[&str]) -> Self {
        let forest = Forest::from_lines(lines);

        Self(forest)
    }
}

#[derive(Debug)]
struct Forest {
    // Shape : [Y][X].
    trees: Vec<Vec<Tree>>,
    width: usize,
    height: usize,
}

impl Forest {
    fn tree(&self, position: Vec2) -> Tree {
        self.trees[position.y() as usize][position.x() as usize]
    }

    fn bounds(&self) -> Bounds {
        bounds!(0, self.width as isize - 1, 0, self.height as isize - 1)
    }

    fn is_visible(&self, position: Vec2) -> bool {
        let is_visible_top = self.is_visible_in(position, vec2!(0, -1));
        let is_visible_bottom = self.is_visible_in(position, vec2!(0, 1));
        let is_visible_left = self.is_visible_in(position, vec2!(-1, 0));
        let is_visible_right = self.is_visible_in(position, vec2!(1, 0));

        is_visible_top || is_visible_bottom || is_visible_left || is_visible_right
    }

    fn is_visible_in(&self, mut position: Vec2, direction: Vec2) -> bool {
        let current = self.tree(position);
        let bounds = self.bounds();

        position += direction;
        while bounds.contains(position) {
            let other = self.tree(position);
            if other >= current { return false; }

            position += direction;
        }
        true
    }

    fn count_visible(&self) -> usize {
        let mut count = self.width * 2 + self.height * 2 - 4;
        let width = self.width as isize;
        let height = self.height as isize;
        for x in 1..width - 1 {
            for y in 1..height - 1 {
                if self.is_visible(vec2!(x, y)) { count += 1; }
            }
        }
        count
    }

    fn scenic_score(&self, position: Vec2) -> usize {
        let score_top = self.scenic_score_in(position, vec2!(0, -1));
        let score_bottom = self.scenic_score_in(position, vec2!(0, 1));
        let score_left = self.scenic_score_in(position, vec2!(-1, 0));
        let score_right = self.scenic_score_in(position, vec2!(1, 0));

        score_top * score_bottom * score_left * score_right
    }

    fn scenic_score_in(&self, mut position: Vec2, direction: Vec2) -> usize {
        let current = self.tree(position);
        let bounds = self.bounds();

        let mut score = 0;
        position += direction;
        while bounds.contains(position) {
            score += 1;
            let other = self.tree(position);
            if other >= current { break; }

            position += direction;
        }
        score
    }

    fn best_scenic_score(&self) -> usize {
        let width = self.width as isize;
        let height = self.height as isize;

        let mut best = 0;
        for x in 0..width {
            for y in 0..height {
                let current = self.scenic_score(vec2!(x, y));
                if current > best { best = current; }
            }
        }
        best
    }
}

impl FromLines for Forest {
    fn from_lines(lines: &[&str]) -> Self {
        let trees: Vec<Vec<Tree>> = lines.iter().map(|it| it.chars().map(char_to!(Tree)).collect()).collect();
        let width = trees.len();
        let height = trees.get(0).map(|it| it.len()).unwrap_or(0);

        Self {
            trees,
            width,
            height,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
struct Tree(u8);

impl FromChar for Tree {
    fn from_char(char: char) -> Self {
        let value = char.to_digit(10).expect("tree value should between 0 and 9") as u8;

        Self(value)
    }
}