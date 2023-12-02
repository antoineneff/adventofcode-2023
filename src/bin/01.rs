use std::collections::HashMap;
use std::fs;

const STRING_NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    let input = fs::read_to_string("./inputs/01").expect("Input should exist");

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &String) {
    let mut count: u32 = 0;
    for line in input.lines() {
        let first = first_num_char(&line);
        let last = first_num_char(&reverse_string(&line));

        let str_from_chars = first.to_string() + &last.to_string();
        let number: u32 = str_from_chars.parse().unwrap();

        count += number
    }
    println!("Part 1: {count}")
}

fn part_two(input: &String) {
    let mut count: u32 = 0;
    let numbers_map = create_map(false);
    let numbers_map_reversed = create_map(true);

    for line in input.lines() {
        let first = first_num_or_string_char(&line, &numbers_map);
        let last = first_num_or_string_char(&reverse_string(&line), &numbers_map_reversed);

        let str_from_chars = first.to_string() + &last.to_string();
        let number: u32 = str_from_chars.parse().unwrap();

        count += number
    }
    println!("Part 2: {count}")
}

fn reverse_string(text: &str) -> String {
    let mut result = String::new();
    for c in text.chars() {
        result.insert(0, c);
    }
    result
}

fn first_num_char(str: &str) -> char {
    let mut first = '0';
    for char in str.chars() {
        if char.is_numeric() {
            first = char;
            break;
        }
    }
    first
}

fn create_map(is_reversed: bool) -> HashMap<String, char> {
    let mut map: HashMap<String, char> = HashMap::new();
    for (i, str) in STRING_NUMBERS.iter().enumerate() {
        if is_reversed {
            map.insert(
                reverse_string(&str),
                char::from_digit((i + 1).try_into().unwrap(), 10).unwrap(),
            );
        } else {
            map.insert(
                str.to_string(),
                char::from_digit((i + 1).try_into().unwrap(), 10).unwrap(),
            );
        }
    }
    map
}

fn first_num_or_string_char(str: &str, numbers_map: &HashMap<String, char>) -> char {
    let mut first = '0';
    let mut buffer = str;
    while first == '0' {
        let c = buffer.chars().next().unwrap();
        if c.is_numeric() {
            first = c;
        } else {
            for string_number in numbers_map.keys() {
                if buffer.starts_with(string_number) {
                    first = numbers_map.get(string_number).unwrap().clone();
                    break;
                }
            }
            if buffer.len() > 0 {
                buffer = &buffer[1..];
            }
        }
    }
    first
}
