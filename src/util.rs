use std::fmt::Debug;
use std::fs;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::path::Path;
use std::str::FromStr;

pub fn read<T, P>(path: P) -> T
    where T: FromLines,
          P: AsRef<Path> {
    let file = fs::read_to_string(path).expect("file should be readable");
    let lines: Vec<&str> = file.lines().collect();
    T::from_lines(&lines)
}

pub trait FromChar {
    fn from_char(char: char) -> Self;
}

pub trait FromLine {
    fn from_line(line: &str) -> Self;
}

pub trait FromLines {
    fn from_lines(lines: &[&str]) -> Self;
}

impl<T> FromLine for T
    where T: FromStr,
          <T as FromStr>::Err: Debug {
    fn from_line(line: &str) -> Self {
        T::from_str(line).expect("line should have valid format")
    }
}

#[macro_export]
macro_rules! on_empty_line {
    () => {
        |it| it.is_empty()
    }
}

#[macro_export]
macro_rules! char_to {
    ($type:ty) => {
        |it| <$type>::from_char(it)
    }
}


#[macro_export]
macro_rules! line_to {
    ($type:ty) => {
        |it| <$type>::from_line(it)
    }
}

#[macro_export]
macro_rules! lines_to {
    ($type:ty) => {
        |it| <$type>::from_lines(it)
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Copy, Clone)]
#[allow(unused)]
pub struct Vec2(isize, isize);

#[allow(unused)]
impl Vec2 {
    pub const fn new(x: isize, y: isize) -> Self {
        Self(x, y)
    }

    pub const fn x(&self) -> isize {
        self.0
    }

    pub const fn y(&self) -> isize {
        self.1
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl SubAssign for Vec2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

#[macro_export]
macro_rules! vec2 {
    ($x:expr, $y:expr) => {
        Vec2::new($x, $y)
    }
}


#[derive(Debug)]
#[allow(unused)]
pub struct Bounds {
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
}

#[allow(unused)]
impl Bounds {
    pub const fn new(min_x: isize, max_x: isize, min_y: isize, max_y: isize) -> Self {
        Self {
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }

    pub const fn contains(&self, position: Vec2) -> bool {
        position.0 >= self.min_x &&
            position.0 <= self.max_x &&
            position.1 >= self.min_y &&
            position.1 <= self.max_y
    }
}

#[macro_export]
macro_rules! bounds {
    ($min_x:expr, $max_x:expr, $min_y:expr, $max_y:expr) => {
        Bounds::new($min_x, $max_x, $min_y, $max_y)
    }
}
