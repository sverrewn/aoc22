use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;

#[derive(Copy, Clone)]
enum Choice {
    Rock,
    Paper,
    Scissor,
}

#[derive(Copy, Clone)]
enum Outcome {
    Loss,
    Draw,
    Win,
}

fn main() {
    println!("{}", find_score());
    println!("{}", find_score_given_outcome());
}

fn find_score() -> i32 {
    let file = File::open("inp.txt").unwrap();
    let reader = BufReader::new(file);

    reader.lines().fold(0, |acc, line| {
        let a: Vec<Choice> = line.unwrap().split(' ').map(|a| Choice::from(a)).collect();
        acc + a[1].fight(&a[0]) + a[1]
    })
}

fn find_score_given_outcome() -> i32 {
    let file = File::open("inp.txt").unwrap();
    let reader = BufReader::new(file);

    reader.lines().fold(0, |acc, line| {
        let line = line.unwrap();
        let l: Vec<&str> = line.split(' ').collect();
        let c = Choice::from(l[0]);
        let o = Outcome::from(l[1]);
        acc + o + o.which_choice(&c)
    })
}

impl Choice {
    fn fight(&self, other: &Choice) -> Outcome {
        match self {
            Choice::Rock => {
                match other {
                    Choice::Rock => Outcome::Draw,
                    Choice::Paper => Outcome::Loss,
                    Choice::Scissor => Outcome::Win,
                }
            }
            Choice::Paper => {
                match other {
                    Choice::Rock => Outcome::Win,
                    Choice::Paper => Outcome::Draw,
                    Choice::Scissor => Outcome::Loss,
                }
            }
            Choice::Scissor => {
                match other {
                    Choice::Rock => Outcome::Loss,
                    Choice::Paper => Outcome::Win,
                    Choice::Scissor => Outcome::Draw,
                }
            }
        }
    }
}

impl From<&str> for Choice {
    fn from(str: &str) -> Self {
        match str {
            "A" | "X" => Choice::Rock,
            "B" | "Y" => Choice::Paper,
            "C" | "Z" => Choice::Scissor,
            _ => panic!("Unexpected character for RPS"),
        }
    }
}

impl Add<Choice> for i32 {
    type Output = i32;

    fn add(self, rhs: Choice) -> i32 {
        match rhs {
            Choice::Rock => self + 1,
            Choice::Paper => self + 2,
            Choice::Scissor => self + 3,
        }
    }
}

impl Outcome {
    fn which_choice(&self, other: &Choice) -> Choice {
        match self {
            Outcome::Loss => match other {
                Choice::Rock => Choice::Scissor,
                Choice::Paper => Choice::Rock,
                Choice::Scissor => Choice::Paper,
            },
            Outcome::Draw => *other,
            Outcome::Win => match other {
                Choice::Rock => Choice::Paper,
                Choice::Paper => Choice::Scissor,
                Choice::Scissor => Choice::Rock,
            }
        }
    }
}

impl From<&str> for Outcome {
    fn from(str: &str) -> Self {
        match str {
            "X" => Outcome::Loss,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Not a valid outcome"),
        }
    }
}
impl Add<Outcome> for i32 {
    type Output = i32;

    fn add(self, rhs: Outcome) -> i32 {
        match rhs {
            Outcome::Loss => self,
            Outcome::Draw => self + 3,
            Outcome::Win => self + 6,
        }
    }
}