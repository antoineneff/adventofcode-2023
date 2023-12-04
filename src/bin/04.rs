use std::{
    collections::{HashSet, VecDeque},
    fs,
};

fn main() {
    let input = fs::read_to_string("./inputs/04").expect("Input should exist");

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &str) {
    let mut count = 0;
    for line in input.lines() {
        let card = ScratchCard::new(line);
        count += card.points();
    }

    println!("Part 1: {count}")
}

fn part_two(input: &str) {
    let mut count = 0;
    let mut copies: VecDeque<u32> = VecDeque::new();

    for line in input.lines() {
        let card = ScratchCard::new(line);

        let mut current_copies = 0;
        if copies.len() > 0 {
            current_copies = copies.pop_front().unwrap();
        }
        count += current_copies + 1;

        let number_of_matches = card.number_of_matches();
        for i in 0..number_of_matches {
            if copies.len() < i + 1 {
                copies.push_back(1 + current_copies);
            } else {
                copies[i] += 1 + current_copies;
            }
        }
    }

    println!("Part 2: {count}")
}

struct ScratchCard {
    winning_numbers: HashSet<u32>,
    choosen_numbers: HashSet<u32>,
}

impl ScratchCard {
    fn new(line: &str) -> ScratchCard {
        let (_, numbers) = line.split_once(": ").unwrap();
        let (left, right) = numbers.split_once(" | ").unwrap();
        let winning_numbers = left
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<HashSet<u32>>();
        let choosen_numbers = right
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<HashSet<u32>>();

        ScratchCard {
            winning_numbers,
            choosen_numbers,
        }
    }

    fn number_of_matches(&self) -> usize {
        let matches: HashSet<&u32> = self
            .winning_numbers
            .intersection(&self.choosen_numbers)
            .collect();

        matches.len()
    }

    fn points(&self) -> usize {
        if self.number_of_matches() == 0 {
            0
        } else {
            let base: usize = 2;
            let power = self.number_of_matches() - 1;
            base.pow(power.try_into().unwrap())
        }
    }
}
