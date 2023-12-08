use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("./inputs/08").expect("Input should exist");

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &str) {
    let network = Network::new(input);
    let steps = network.navigate("AAA", "ZZZ");

    println!("Part 1: {steps}")
}

fn part_two(input: &str) {
    let network = Network::new(input);
    let start_nodes: Vec<String> = network
        .nodes
        .keys()
        .filter(|&k| k.ends_with("A"))
        .map(|s| s.to_string())
        .collect();

    let mut steps_vec = Vec::new();
    for node in start_nodes {
        let steps = network.navigate(&node, "Z");
        steps_vec.push(steps);
    }

    let result = lcmm(steps_vec);

    println!("Part 2: {result}")
}

fn lcmm(numbers: Vec<usize>) -> usize {
    if numbers.len() == 2 {
        return lcm(numbers[0], numbers[1]);
    }

    return lcm(numbers[0], lcmm(numbers[1..].to_vec()));
}

fn lcm(a: usize, b: usize) -> usize {
    let max = a.max(b);
    let min = a.min(b);

    (a * b) / gcd(max, min)
}

fn gcd(max: usize, min: usize) -> usize {
    if max % min == 0 {
        return min;
    }
    return gcd(min, max % min);
}

struct Network {
    directions: Vec<char>,
    nodes: HashMap<String, (String, String)>,
}

impl Network {
    fn new(input: &str) -> Self {
        let mut directions = Vec::new();
        let mut nodes = HashMap::new();
        for (i, line) in input.lines().enumerate() {
            if i == 0 {
                directions = line.chars().collect();
            } else if line.is_empty() {
                continue;
            } else {
                let (start, end) = line.split_once(" = ").unwrap();
                let end = end.replace(&['(', ')'][..], "");
                let (left, right) = end.split_once(", ").unwrap();
                nodes.insert(start.to_string(), (left.to_string(), right.to_string()));
            }
        }

        Network { directions, nodes }
    }

    fn next(&self, steps: usize, pos: &str) -> &str {
        let direction = self.directions[steps % self.directions.len()];
        let (left, right) = self.nodes.get(pos).unwrap();
        if direction == 'L' {
            return left;
        } else {
            return right;
        }
    }

    fn navigate(&self, start: &str, end: &str) -> usize {
        let mut steps = 0;
        let mut pos = start;
        while !pos.ends_with(end) {
            pos = self.next(steps, pos);
            steps += 1;
        }
        steps
    }
}
