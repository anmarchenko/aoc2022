use crate::input;

pub fn part1() -> String {
    solution_part1(String::from("input"))
}

pub fn part2() -> String {
    solution_part2(String::from("input"))
}

fn solution_part1(file: String) -> String {
    let data = input::get_input(1, &file);
    let cals = calories(&data);

    cals.get(0).unwrap().to_string()
}

fn solution_part2(file: String) -> String {
    let data = input::get_input(1, &file);
    let mut cals = calories(&data);

    cals.drain(0..=2).sum::<i32>().to_string()
}

fn calories(data: &str) -> Vec<i32> {
    let mut elves: Vec<i32> = vec![0];
    for line in data.lines() {
        if line.is_empty() {
            elves.push(0)
        } else {
            let cal = line.parse::<i32>().unwrap();
            *elves.last_mut().unwrap() += cal
        }
    }
    elves.sort_by(|a, b| b.cmp(a));
    elves
}
