use std::fs;

fn main() {
    let input = fs::read_to_string("./inputs/09").expect("Input should exist");

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &str) {
    let report = Report::new(input);
    let mut count = 0;
    for history in &report.history {
        count += report.extrapolate(history);
    }
    println!("Part 1: {count}")
}

fn part_two(input: &str) {
    let report = Report::new(input);
    let mut count = 0;
    for history in &report.history {
        let mut clone = history.clone();
        clone.reverse();
        count += report.extrapolate(&clone);
    }
    println!("Part 2: {count}")
}

struct Report {
    history: Vec<Vec<i32>>,
}

impl Report {
    fn new(input: &str) -> Self {
        let mut history = Vec::new();
        for line in input.lines() {
            let numbers: Vec<i32> = line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect();
            history.push(numbers);
        }

        Report { history }
    }

    fn extrapolate(&self, history: &Vec<i32>) -> i32 {
        let mut sequence: Vec<i32> = history.clone();
        let mut last_values: Vec<i32> = vec![*sequence.last().unwrap()];
        while sequence.iter().find(|&n| *n != 0).is_some() {
            sequence = self.next_sequence(&sequence);
            last_values.push(*sequence.last().unwrap());
        }
        last_values.iter().sum()
    }

    fn next_sequence(&self, numbers: &Vec<i32>) -> Vec<i32> {
        let mut next = Vec::new();
        for n in numbers.windows(2) {
            next.push(n[1] - n[0]);
        }
        next
    }
}
