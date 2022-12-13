use util::{FromLine, FromLines, read, run};

mod util;

fn main() {
    let (t0, input) = run(|| read::<Input, _>("inputs/day2.txt"));
    let (t1, p1) = run(|| input.part_1());
    let (t2, p2) = run(|| input.part_2());

    println!("Part 1 : {}", p1);
    println!("Part 2 : {}", p2);
    println!("Time : {} ns", (t0 + t1 + t2).as_nanos());
}

#[derive(Debug)]
struct Input {
    rounds: Vec<Round>,
}

impl Input {
    fn part_1(&self) -> u64 {
        self.rounds
            .iter()
            .map(|it| {
                let player = it.player;
                let opponent = it.opponent;
                let outcome = player.outcome_for(opponent);
                outcome.score() + player.score()
            })
            .sum()
    }

    fn part_2(&self) -> u64 {
        self.rounds
            .iter()
            .map(|it| {
                let outcome = it.outcome;
                let opponent = it.opponent;
                let player = opponent.for_outcome(outcome);
                outcome.score() + player.score()
            })
            .sum()
    }
}

#[derive(Debug)]
struct Round {
    opponent: Choice,
    player: Choice,
    outcome: Outcome,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Choice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Choice {
    fn wins_against(&self) -> Self {
        match self {
            Self::Rock => Self::Scissors,
            Self::Paper => Self::Rock,
            Self::Scissors => Self::Paper,
        }
    }

    fn loses_against(&self) -> Self {
        match self {
            Self::Rock => Self::Paper,
            Self::Paper => Self::Scissors,
            Self::Scissors => Self::Rock,
        }
    }

    fn outcome_for(&self, other: Self) -> Outcome {
        if *self == other {
            Outcome::Draw
        } else if self.wins_against() == other {
            Outcome::Win
        } else {
            Outcome::Loss
        }
    }

    fn for_outcome(&self, outcome: Outcome) -> Self {
        match outcome {
            Outcome::Win => self.loses_against(),
            Outcome::Loss => self.wins_against(),
            Outcome::Draw => *self,
        }
    }

    fn score(&self) -> u64 {
        *self as u64
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Outcome {
    Win = 6,
    Loss = 0,
    Draw = 3,
}

impl Outcome {
    fn score(&self) -> u64 {
        *self as u64
    }
}

impl FromLines for Input {
    fn from_lines(lines: &[&str]) -> Self {
        let rounds = lines.iter().map(line_to!(Round)).collect();

        Self {
            rounds
        }
    }
}

impl FromLine for Round {
    fn from_line(line: &str) -> Self {
        let (lhs, rhs) = line.split_once(' ').expect("round should have a left and a right part");

        Self {
            player: Choice::from_line(rhs),
            opponent: Choice::from_line(lhs),
            outcome: Outcome::from_line(rhs),
        }
    }
}

impl FromLine for Choice {
    fn from_line(line: &str) -> Self {
        match line {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("\"{line}\" is not va valid choice")
        }
    }
}

impl FromLine for Outcome {
    fn from_line(line: &str) -> Self {
        match line {
            "X" => Self::Loss,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("\"{line}\" is not a valid outcome")
        }
    }
}