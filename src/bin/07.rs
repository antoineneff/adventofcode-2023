use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("./inputs/07").expect("Input should exist");

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &str) {
    let mut game = Game::new(input, Version::One);

    println!("Part 1: {}", game.winnings())
}

fn part_two(input: &str) {
    let mut game = Game::new(input, Version::Two);

    println!("Part 2: {}", game.winnings())
}

const CARDS_STRENGTH_1: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];
const CARDS_STRENGTH_2: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

#[derive(Debug, Clone)]
enum Version {
    One,
    Two,
}

#[derive(Debug)]
struct Game {
    hands: Vec<Hand>,
}

#[derive(Debug)]
struct Hand {
    cards: Vec<char>,
    bid: usize,
    version: Version,
}

impl Game {
    fn new(input: &str, version: Version) -> Self {
        let mut hands = Vec::new();
        for line in input.lines() {
            let (cards_str, bid_str) = line.split_once(" ").unwrap();
            let cards = cards_str.chars().collect();
            let bid: usize = bid_str.parse().unwrap();
            hands.push(Hand {
                cards,
                bid,
                version: version.clone(),
            })
        }

        Game { hands }
    }

    fn winnings(&mut self) -> usize {
        let mut count = 0;
        self.hands.sort();
        for (i, hand) in self.hands.iter().enumerate() {
            let rank = i + 1;
            count += rank * hand.bid;
        }
        count
    }
}

impl Hand {
    fn strength(&self) -> u8 {
        let mut occurences: HashMap<char, u8> = HashMap::new();
        for card in &self.cards {
            let current_value = occurences.get(&card).unwrap_or(&0);
            occurences.insert(*card, current_value + 1);
        }

        let mut jokers_count = 0;
        if let Version::Two = self.version {
            jokers_count = occurences.remove(&'J').unwrap_or(0);
            let highest_count_key = occurences.iter().max_by(|a, b| a.1.cmp(b.1));
            if let Some((k, v)) = highest_count_key {
                occurences.insert(*k, v + jokers_count);
            }
        }

        if occurences.values().any(|&x| x == 5) || jokers_count == 5 {
            7
        } else if occurences.values().any(|&x| x == 4) {
            6
        } else if occurences.values().any(|&x| x == 3) && occurences.values().any(|&x| x == 2) {
            5
        } else if occurences.values().any(|&x| x == 3) {
            4
        } else if occurences.values().any(|&x| x == 2) && occurences.values().count() == 3 {
            3
        } else if occurences.values().any(|&x| x == 2) {
            2
        } else {
            1
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.strength().cmp(&other.strength())
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.strength() == other.strength() {
            for i in 0..self.cards.len() {
                let strength_list = match self.version {
                    Version::One => CARDS_STRENGTH_1,
                    Version::Two => CARDS_STRENGTH_2,
                };
                let self_card_strength = strength_list
                    .iter()
                    .position(|e| e == &self.cards[i])
                    .unwrap();
                let other_card_strength = strength_list
                    .iter()
                    .position(|e| e == &other.cards[i])
                    .unwrap();

                match self_card_strength.cmp(&other_card_strength) {
                    Ordering::Less => return Some(Ordering::Less),
                    Ordering::Equal => continue,
                    Ordering::Greater => return Some(Ordering::Greater),
                };
            }
            None
        } else {
            Some(self.strength().cmp(&other.strength()))
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.strength() == other.strength()
    }
}

impl Eq for Hand {}
