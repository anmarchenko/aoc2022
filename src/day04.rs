use crate::input;

pub fn part1() -> String {
    // solution_part1(String::from("test"))
    solution_part1(String::from("input"))
}

pub fn part2() -> String {
    solution_part2(String::from("input"))
}

fn solution_part1(file: String) -> String {
    let data = input::get_input(4, &file);
    let elven_pairs = parse_input(&data);
    elven_pairs
        .iter()
        .filter(|e_pair| e_pair.contains())
        .count()
        .to_string()
}

fn solution_part2(file: String) -> String {
    let data = input::get_input(4, &file);
    let elven_pairs = parse_input(&data);
    elven_pairs
        .iter()
        .filter(|e_pair| e_pair.overlaps())
        .count()
        .to_string()
}

#[derive(Debug)]
struct Interval {
    left: u32,
    right: u32,
}

impl Interval {
    fn new(s: &str) -> Self {
        let pair: Vec<&str> = s.split("-").collect();
        Interval {
            left: pair[0].parse().unwrap(),
            right: pair[1].parse().unwrap(),
        }
    }

    fn contains(&self, other: &Interval) -> bool {
        self.left <= other.left && self.right >= other.right
    }

    fn overlaps(&self, other: &Interval) -> bool {
        self.left <= other.right && self.right >= other.left
    }
}

struct ElvenPair {
    first: Interval,
    second: Interval,
}

impl ElvenPair {
    fn new(s: &str) -> Self {
        let pair: Vec<&str> = s.split(",").collect();
        ElvenPair {
            first: Interval::new(pair[0]),
            second: Interval::new(pair[1]),
        }
    }

    fn contains(&self) -> bool {
        self.first.contains(&self.second) || self.second.contains(&self.first)
    }

    fn overlaps(&self) -> bool {
        self.first.overlaps(&self.second)
    }
}

fn parse_input(input: &str) -> Vec<ElvenPair> {
    input.split("\n").map(|s| ElvenPair::new(s)).collect()
}
