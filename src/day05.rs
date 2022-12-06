use crate::input;
use regex::Regex;

pub fn part1() -> String {
    // solution_part1(String::from("test"))
    solution_part1(String::from("input"))
}

pub fn part2() -> String {
    solution_part2(String::from("input"))
}

fn solution_part1(file: String) -> String {
    let data = input::get_input(5, &file);
    let (stacks_data, operations_data) = split_input(&data);

    let mut stacks = parse_stacks(stacks_data);

    let operations = parse_operations(operations_data);

    for op in operations {
        apply_operation(&op, &mut stacks);
    }

    stacks.iter().map(|s| s.peek()).collect()
}

fn solution_part2(file: String) -> String {
    let data = input::get_input(5, &file);
    let (stacks_data, operations_data) = split_input(&data);

    let mut stacks = parse_stacks(stacks_data);

    let operations = parse_operations(operations_data);
    for op in operations {
        apply_operation_9001(&op, &mut stacks);
    }

    stacks.iter().map(|s| s.peek()).collect()
}

fn split_input(data: &str) -> (Vec<&str>, Vec<&str>) {
    let mut data_split = data.split("\n\n");
    let stacks_data: Vec<&str> = data_split.next().unwrap().split("\n").collect();
    let operations: Vec<&str> = data_split.next().unwrap().split("\n").collect();
    (stacks_data, operations)
}

fn parse_stacks(data: Vec<&str>) -> Vec<Stack> {
    let mut iter = data.iter().rev();
    let mut stacks: Vec<Stack> = iter
        .next()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|_| Stack::new())
        .collect();

    for line in iter {
        let chars: Vec<char> = line.chars().collect();
        for (index, stack) in stacks.iter_mut().enumerate() {
            if let Some(ch) = chars.get(index * 4 + 1) {
                if *ch != ' ' {
                    stack.push(*ch);
                }
            }
        }
    }
    stacks
}

fn parse_operations(data: Vec<&str>) -> Vec<Operation> {
    data.iter().map(|line| Operation::new(line)).collect()
}

fn apply_operation(op: &Operation, stacks: &mut Vec<Stack>) {
    for _ in 0..op.count {
        let ch = stacks[op.from as usize - 1].pop();
        stacks[op.to as usize - 1].push(ch);
    }
}

fn apply_operation_9001(op: &Operation, stacks: &mut Vec<Stack>) {
    let chars = stacks[op.from as usize - 1].pop_count(op.count);
    for char in chars {
        stacks[op.to as usize - 1].push(char)
    }
}

#[derive(Debug)]
struct Stack {
    elements: Vec<char>,
}

impl Stack {
    fn new() -> Self {
        Stack {
            elements: Vec::new(),
        }
    }

    fn push(&mut self, ch: char) {
        self.elements.push(ch)
    }

    fn pop(&mut self) -> char {
        self.elements.pop().unwrap()
    }

    fn pop_count(&mut self, count: u32) -> Vec<char> {
        let l = self.elements.len();
        self.elements.split_off(l - count as usize)
    }

    fn peek(&self) -> char {
        *self.elements.last().unwrap()
    }
}

#[derive(Debug)]
struct Operation {
    count: u32,
    from: u32,
    to: u32,
}

impl Operation {
    fn new(s: &str) -> Self {
        let digits_regex: Regex = Regex::new(r"[0-9]+").unwrap();
        let numbers: Vec<u32> = digits_regex
            .find_iter(s)
            .map(|snum| snum.as_str().parse::<u32>().unwrap())
            .collect();

        Operation {
            count: numbers[0],
            from: numbers[1],
            to: numbers[2],
        }
    }
}
