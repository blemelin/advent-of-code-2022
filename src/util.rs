use std::any::type_name;
use std::fmt::Debug;
use std::fs;
use std::ops::{Add, AddAssign};
use std::path::Path;
use std::str::FromStr;

pub fn read<T, P>(path: P) -> T
    where T: FromLines,
          P: AsRef<Path> {
    let file = fs::read_to_string(path).expect("input should be readable");
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
        T::from_str(line).expect(&format!("line should be a valid {}", type_name::<T>()))
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

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
#[allow(unused)]
pub struct Vec2<T>(T, T);

#[allow(unused)]
impl<T> Vec2<T>
    where T: Copy {
    pub const fn new(x: T, y: T) -> Self {
        Self(x, y)
    }

    pub const fn x(&self) -> T {
        self.0
    }

    pub const fn y(&self) -> T {
        self.1
    }
}

impl Add for Vec2<usize> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Add for Vec2<isize> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Add<Vec2<isize>> for Vec2<usize> {
    type Output = Option<Self>;

    fn add(self, rhs: Vec2<isize>) -> Self::Output {
        match (checked_add_signed(self.0, rhs.0), checked_add_signed(self.1, rhs.1)) {
            (Some(x), Some(y)) => Some(Self(x, y)),
            _ => None
        }
    }
}

impl Add<Vec2<usize>> for Vec2<isize> {
    type Output = Option<Self>;

    fn add(self, rhs: Vec2<usize>) -> Self::Output {
        match (checked_add_unsigned(self.0, rhs.0), checked_add_unsigned(self.1, rhs.1)) {
            (Some(x), Some(y)) => Some(Self(x, y)),
            _ => None
        }
    }
}

impl AddAssign for Vec2<usize> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl AddAssign for Vec2<isize> {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

#[macro_export]
macro_rules! vec2 {
    ($x:expr, $y:expr) => {
        Vec2::new($x, $y)
    }
}

fn checked_add_signed(lhs: usize, rhs: isize) -> Option<usize> {
    if rhs >= 0 {
        lhs.checked_add(rhs as usize)
    } else {
        lhs.checked_sub(rhs.unsigned_abs())
    }
}

fn checked_add_unsigned(lhs: isize, rhs: usize) -> Option<isize> {
    let rhs_div = (rhs / 2) as isize;
    let rhs_rem = (rhs % 2) as isize;

    lhs.checked_add(rhs_div)
        .and_then(|lhs| lhs.checked_add(rhs_div))
        .and_then(|lhs| lhs.checked_add(rhs_rem))
}
