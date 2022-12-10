use util::{FromLine, FromLines, read};

mod util;

fn main() {
    // Read data
    let Data(rounds) = read("inputs/day2.txt");

    // Part 1
    let score: u64 = rounds.iter().map(|it| it.score_for_play()).sum();
    println!("Part 1 : {}", score);

    // Part 2
    let score: u64 = rounds.iter().map(|it| it.score_for_outcome()).sum();
    println!("Part 2 : {}", score);
}

#[derive(Debug)]
struct Data(Vec<Round>);

impl FromLines for Data {
    fn from_lines(lines: &[&str]) -> Self {
        let rounds = lines.iter().map(line_to!(Round)).collect();

        Self(rounds)
    }
}

#[derive(Debug)]
struct Round {
    opponent: Choice,
    desired_play: Choice,
    desired_outcome: Outcome,
}

impl Round {
    fn score_for_play(&self) -> u64 {
        let player_play = self.desired_play;
        let opponent_play = self.opponent;
        let outcome = player_play.outcome_for(opponent_play);
        outcome.score() + player_play.score()
    }
    fn score_for_outcome(&self) -> u64 {
        let outcome = self.desired_outcome;
        let opponent_play = self.opponent;
        let player_play = opponent_play.for_outcome(outcome);
        outcome.score() + player_play.score()
    }
}

impl FromLine for Round {
    fn from_line(line: &str) -> Self {
        let mut parts = line.split(' ');
        let lhs = parts.next().expect("round should have a left-hand side part");
        let rhs = parts.next().expect("round should have a right-hand side part");

        Self {
            desired_play: Choice::from_line(rhs),
            opponent: Choice::from_line(lhs),
            desired_outcome: Outcome::from_line(rhs),
        }
    }
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