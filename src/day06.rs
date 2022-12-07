use std::str::Chars;

use crate::input;
use itertools::Itertools;

pub fn part1() -> String {
    // solution_part1(String::from("test"))
    solution_part1(String::from("input"))
}

pub fn part2() -> String {
    solution_part2(String::from("input"))
}

fn solution_part1(file: String) -> String {
    let data = input::get_input(6, &file);
    for (index, (a, b, c, d)) in data.chars().tuple_windows().enumerate() {
        if a != b && b != c && c != d && a != c && a != d && b != d {
            return (index + 4).to_string();
        }
    }
    String::from("no answer")
}

fn solution_part2(file: String) -> String {
    let data = input::get_input(6, &file);
    String::from("XXX")
}
