use crate::input;
use std::str::FromStr;

pub fn part1() -> String {
    // solution_part1(String::from("test"))
    solution_part1(String::from("input"))
}

pub fn part2() -> String {
    solution_part2(String::from("input"))
}

fn solution_part1(file: String) -> String {
    let data = input::get_input(2, &file);
    let rounds = hand_pairs(&data);
    score(&rounds).to_string()
}

fn solution_part2(file: String) -> String {
    let data = input::get_input(2, &file);
    let rounds = hand_result_pairs(&data);
    score(&rounds).to_string()
}

fn score(rounds: &Vec<impl HasScore>) -> u32 {
    rounds.iter().map(|round| round.score()).sum()
}

fn hand_pairs(data: &str) -> Vec<impl HasScore> {
    data.lines()
        .map(|line| {
            let hands: Vec<&str> = line.split(" ").collect();
            (
                Hand::from_str(hands[0]).unwrap(),
                Hand::from_str(hands[1]).unwrap(),
            )
        })
        .collect()
}

fn hand_result_pairs(data: &str) -> Vec<impl HasScore> {
    data.lines()
        .map(|line| {
            let strs: Vec<&str> = line.split(" ").collect();
            (
                Hand::from_str(strs[0]).unwrap(),
                GameResult::from_str(strs[1]).unwrap(),
            )
        })
        .collect()
}

trait HasScore {
    fn score(&self) -> u32;
}

#[derive(Debug, PartialEq)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}
#[derive(Debug)]
struct UnknownHandError;

impl FromStr for Hand {
    type Err = UnknownHandError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Hand::Rock),
            "B" => Ok(Hand::Paper),
            "C" => Ok(Hand::Scissors),
            "X" => Ok(Hand::Rock),
            "Y" => Ok(Hand::Paper),
            "Z" => Ok(Hand::Scissors),
            _ => Err(UnknownHandError),
        }
    }
}

impl HasScore for Hand {
    fn score(&self) -> u32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
}

#[derive(Debug)]
enum GameResult {
    Win,
    Draw,
    Lose,
}

#[derive(Debug)]
struct UnknownResultError;

impl FromStr for GameResult {
    type Err = UnknownResultError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(GameResult::Lose),
            "Y" => Ok(GameResult::Draw),
            "Z" => Ok(GameResult::Win),
            _ => Err(UnknownResultError),
        }
    }
}

impl HasScore for GameResult {
    fn score(&self) -> u32 {
        match self {
            GameResult::Lose => 0,
            GameResult::Draw => 3,
            GameResult::Win => 6,
        }
    }
}

impl HasScore for (Hand, Hand) {
    fn score(&self) -> u32 {
        let result = match self {
            (a, b) if a == b => GameResult::Draw,
            (Hand::Rock, Hand::Scissors) => GameResult::Lose,
            (Hand::Scissors, Hand::Paper) => GameResult::Lose,
            (Hand::Paper, Hand::Rock) => GameResult::Lose,
            (_, _) => GameResult::Win,
        };
        result.score() + self.1.score()
    }
}
impl HasScore for (Hand, GameResult) {
    fn score(&self) -> u32 {
        let hand = hand_from_expected_result((&self.0, &self.1));
        hand.score() + self.1.score()
    }
}

fn hand_from_expected_result(game: (&Hand, &GameResult)) -> Hand {
    match game {
        (Hand::Rock, GameResult::Draw) => Hand::Rock,
        (Hand::Rock, GameResult::Lose) => Hand::Scissors,
        (Hand::Rock, GameResult::Win) => Hand::Paper,
        (Hand::Paper, GameResult::Draw) => Hand::Paper,
        (Hand::Paper, GameResult::Lose) => Hand::Rock,
        (Hand::Paper, GameResult::Win) => Hand::Scissors,
        (Hand::Scissors, GameResult::Draw) => Hand::Scissors,
        (Hand::Scissors, GameResult::Lose) => Hand::Paper,
        (Hand::Scissors, GameResult::Win) => Hand::Rock,
    }
}
