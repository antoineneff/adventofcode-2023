use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("./inputs/03").expect("Input should exist");

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &String) {
    let mut count = 0;
    let schema = Schematic::new(&input);
    let part_numbers = schema.part_numbers();
    for part_number in part_numbers {
        if part_number.has_adjacent_symbol() {
            count += part_number.number;
        }
    }

    println!("Part 1: {count}")
}

fn part_two(input: &String) {
    let mut count = 0;
    let schema = Schematic::new(&input);
    for (_, n) in schema.gears() {
        if n.len() > 1 {
            count += n[0] * n[1];
        }
    }

    println!("Part 2: {count}")
}

struct Schematic {
    representation: Vec<Vec<char>>,
}

impl Schematic {
    fn new(input: &String) -> Self {
        let mut representation = Vec::new();
        for line in input.lines() {
            representation.push(line.chars().collect())
        }
        Schematic { representation }
    }

    fn get_adjacent_chars(
        &self,
        line_idx: usize,
        left_idx: usize,
        right_idx: usize,
    ) -> Vec<SchemaChar> {
        let mut adjacents: Vec<SchemaChar> = Vec::new();
        // Top side
        if line_idx > 0 {
            for i in left_idx..=right_idx {
                adjacents.push(SchemaChar {
                    x: i,
                    y: line_idx - 1,
                    char: self.representation[line_idx - 1][i],
                });
            }
        }
        // Left side
        adjacents.push(SchemaChar {
            x: left_idx,
            y: line_idx,
            char: self.representation[line_idx][left_idx],
        });
        // Right side
        adjacents.push(SchemaChar {
            x: right_idx,
            y: line_idx,
            char: self.representation[line_idx][right_idx],
        });
        // Bottom side
        if line_idx < self.representation.len() - 1 {
            for i in left_idx..=right_idx {
                adjacents.push(SchemaChar {
                    x: i,
                    y: line_idx + 1,
                    char: self.representation[line_idx + 1][i],
                });
            }
        }
        adjacents
    }

    fn part_numbers(&self) -> Vec<PartNumber> {
        let mut part_numbers = Vec::new();
        for (line_idx, line) in self.representation.iter().enumerate() {
            let mut number_buffer = String::new();
            for (char_idx, char) in line.iter().enumerate() {
                if char.is_ascii_digit() {
                    number_buffer.push(*char);
                } else if !number_buffer.is_empty() {
                    let left_idx = {
                        if char_idx == number_buffer.len() {
                            0
                        } else {
                            char_idx - (number_buffer.len() + 1)
                        }
                    };
                    let adjacent_chars = self.get_adjacent_chars(line_idx, left_idx, char_idx);

                    part_numbers.push(PartNumber {
                        number: number_buffer.parse().unwrap(),
                        adjacent_chars,
                    });
                    number_buffer.clear();
                }
            }
            if !number_buffer.is_empty() {
                let adjacent_chars = self.get_adjacent_chars(
                    line_idx,
                    line.len() - 1 - number_buffer.len(),
                    line.len() - 1,
                );

                part_numbers.push(PartNumber {
                    number: number_buffer.parse().unwrap(),
                    adjacent_chars,
                });
            }
        }
        part_numbers
    }

    fn gears(&self) -> HashMap<String, Vec<u32>> {
        let mut gears_map: HashMap<String, Vec<u32>> = HashMap::new();
        for part_number in self.part_numbers() {
            for adjacent_char in part_number.adjacent_chars {
                if adjacent_char.char == '*' {
                    let key = format!("{}:{}", adjacent_char.x, adjacent_char.y);
                    let new_value = vec![part_number.number];
                    if gears_map.contains_key(&key) {
                        let current_value = gears_map.get(&key).unwrap();
                        gears_map.insert(key, [new_value, current_value.to_vec()].concat());
                    } else {
                        gears_map.insert(key, new_value);
                    }
                }
            }
        }
        gears_map
    }
}

struct SchemaChar {
    x: usize,
    y: usize,
    char: char,
}

struct PartNumber {
    number: u32,
    adjacent_chars: Vec<SchemaChar>,
}

impl PartNumber {
    fn has_adjacent_symbol(&self) -> bool {
        for adjacent in &self.adjacent_chars {
            if adjacent.char != '.' && !adjacent.char.is_ascii_digit() {
                return true;
            }
        }
        false
    }
}
