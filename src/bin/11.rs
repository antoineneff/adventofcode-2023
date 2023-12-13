use std::{fs, time::Instant};

fn main() {
    let input = fs::read_to_string("./inputs/11").expect("Input should exist");

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &str) {
    let instant = Instant::now();
    let mut map = Map::new(input);
    map.expand(2);
    let mut count = 0;
    for pair in map.pairs() {
        count += shorter_path(pair);
    }

    println!("Part 1: {count} in {:?}", instant.elapsed())
}

fn part_two(input: &str) {
    let instant = Instant::now();
    let mut map = Map::new(input);
    map.expand(1_000_000);
    let mut count = 0;
    for pair in map.pairs() {
        count += shorter_path(pair);
    }

    println!("Part 2: {count} in {:?}", instant.elapsed())
}

struct Map {
    galaxies: Vec<(usize, usize)>,
    empty_rows: Vec<usize>,
    empty_cols: Vec<usize>,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut galaxies = Vec::new();
        let (mut min_col, mut max_col) = (usize::MAX, usize::MIN);
        let mut empty_rows = Vec::new();
        for (row, line) in input.lines().enumerate() {
            let mut is_empty_row = true;
            for (col, char) in line.chars().enumerate() {
                if char == '#' {
                    is_empty_row = false;
                    galaxies.push((row, col));
                    if col > max_col {
                        max_col = col;
                    }
                    if col < min_col {
                        min_col = col;
                    }
                }
            }
            if is_empty_row {
                empty_rows.push(row);
            }
        }

        let mut empty_cols = Vec::new();
        for i in min_col + 1..max_col {
            if !galaxies.iter().any(|g| g.1 == i) {
                empty_cols.push(i);
            }
        }

        Map {
            galaxies,
            empty_rows,
            empty_cols,
        }
    }

    fn expand(&mut self, expand_factor: usize) {
        for (i, galaxy) in self.galaxies.clone().iter().enumerate() {
            let mut new_row = galaxy.0;
            let mut new_col = galaxy.1;

            for empty_row in &self.empty_rows {
                if empty_row < &galaxy.0 {
                    new_row += expand_factor - 1;
                }
            }

            for empty_col in &self.empty_cols {
                if empty_col < &galaxy.1 {
                    new_col += expand_factor - 1;
                }
            }

            self.galaxies[i] = (new_row, new_col);
        }
    }

    fn pairs(&self) -> Vec<(&(usize, usize), &(usize, usize))> {
        let mut pairs = Vec::new();
        for (i, galaxy) in self.galaxies.iter().enumerate() {
            for (j, galaxy2) in self.galaxies.iter().enumerate() {
                if i >= j {
                    continue;
                }
                pairs.push((galaxy, galaxy2))
            }
        }
        pairs
    }
}

fn shorter_path(pair: (&(usize, usize), &(usize, usize))) -> usize {
    pair.0 .0.abs_diff(pair.1 .0) + pair.0 .1.abs_diff(pair.1 .1)
}
