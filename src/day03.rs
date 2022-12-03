use crate::input;
use std::collections::HashSet;

pub fn part1() -> String {
    // solution_part1(String::from("test"))
    solution_part1(String::from("input"))
}

pub fn part2() -> String {
    solution_part2(String::from("input"))
}

fn solution_part1(file: String) -> String {
    let data = input::get_input(3, &file);
    data.lines()
        .map(|line| {
            let set_left: HashSet<char> = line_to_set(&line[0..line.len() / 2]);
            let set_right: HashSet<char> = line_to_set(&line[line.len() / 2..]);
            score(set_left.intersection(&set_right).next().unwrap())
        })
        .sum::<u32>()
        .to_string()
}

fn solution_part2(file: String) -> String {
    let data = input::get_input(3, &file);
    let lines: Vec<&str> = data.lines().collect();
    lines
        .chunks(3)
        .map(|chunk| {
            score(
                chunk
                    .iter()
                    .map(|line| line_to_set(line))
                    .reduce(|l, r| l.intersection(&r).copied().collect())
                    .unwrap()
                    .iter()
                    .next()
                    .unwrap(),
            )
        })
        .sum::<u32>()
        .to_string()
}

fn score(ch: &char) -> u32 {
    match ch {
        'a'..='z' => *ch as u32 - 'a' as u32 + 1,
        'A'..='Z' => *ch as u32 - 'A' as u32 + 27,
        _ => 0,
    }
}

fn line_to_set(line: &str) -> HashSet<char> {
    line.chars().collect::<HashSet<char>>()
}
