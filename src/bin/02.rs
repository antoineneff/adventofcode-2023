use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("./inputs/02").expect("Input should exist");

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &String) {
    let mut count = 0;
    for line in input.lines() {
        let game = Game::new(line);
        if game.is_possible() {
            count += game.id;
        }
    }

    println!("Part 1: {count}")
}

fn part_two(input: &String) {
    let mut count = 0;
    for line in input.lines() {
        let game = Game::new(line);
        count += game.power();
    }

    println!("Part 2: {count}")
}

struct Game {
    id: u32,
    pairs: Vec<(String, u32)>,
    limits: Vec<(String, u32)>,
}

impl Game {
    fn new(line: &str) -> Self {
        let (game, sets) = line.split_once(": ").unwrap();
        let (_, game_id) = game.split_once(" ").unwrap();

        let pairs: Vec<(&str, &str)> = sets
            .split(&[';', ','])
            .map(|x| x.trim().split_once(" ").unwrap())
            .collect();

        Game {
            id: game_id.parse().unwrap(),
            pairs: pairs
                .iter()
                .map(|(x, y)| (y.to_string(), x.parse::<u32>().unwrap()))
                .collect(),
            limits: vec![
                (String::from("red"), 12),
                (String::from("green"), 13),
                (String::from("blue"), 14),
            ],
        }
    }

    fn is_possible(&self) -> bool {
        for (color, number) in &self.pairs {
            for (limit_color, limit_number) in &self.limits {
                if color.eq(limit_color) && number > limit_number {
                    return false;
                }
            }
        }
        true
    }

    fn power(&self) -> u32 {
        let mut max_values: HashMap<&String, &u32> = HashMap::new();
        for (color, number) in &self.pairs {
            if !max_values.contains_key(color) {
                max_values.insert(color, number);
            } else {
                let current_max = max_values.get(color).unwrap();
                if number > current_max {
                    max_values.insert(color, number);
                }
            }
        }
        let mut total = 1;
        for value in max_values.values() {
            total *= *value;
        }
        total
    }
}
