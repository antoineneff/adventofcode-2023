use std::fs;

fn main() {
    let input = fs::read_to_string("./inputs/05").expect("Input should exist");

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &str) {
    let almanac = Almanac::new(input);
    let mut smallest = usize::MAX;
    for seed in almanac.seeds {
        let mut temp_value = seed;
        for map in &almanac.maps {
            for range in &map.ranges {
                let destination = range.destination(&temp_value);
                if destination != temp_value {
                    temp_value = destination;
                    break;
                }
            }
        }
        if temp_value < smallest {
            smallest = temp_value;
        }
    }

    println!("Part 1: {smallest}")
}

fn part_two(input: &str) {
    let almanac = Almanac::new(input);
    let mut smallest = usize::MAX;
    for (start, end) in almanac.seed_ranges() {
        println!("Computing range [{start}..{end}[");
        for i in start..end {
            let mut temp_value = i;
            for map in &almanac.maps {
                for range in &map.ranges {
                    let destination = range.destination(&temp_value);
                    if destination != temp_value {
                        temp_value = destination;
                        break;
                    }
                }
            }
            if temp_value < smallest {
                smallest = temp_value;
            }
        }
    }

    println!("Part 2: {smallest}")
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<Map>,
}

#[derive(Debug)]
struct Map {
    source: String,
    destination: String,
    ranges: Vec<MapRange>,
}

#[derive(Debug)]
struct MapRange {
    source_start: usize,
    destination_start: usize,
    length: usize,
}

impl Almanac {
    fn new(input: &str) -> Self {
        let mut seeds: Vec<usize> = Vec::new();
        let mut maps: Vec<Map> = Vec::new();
        let mut source_temp = "";
        let mut destination_temp = "";
        let mut ranges_temp: Vec<MapRange> = Vec::new();

        for line in input.lines() {
            if line.is_empty() {
                if ranges_temp.len() > 0 {
                    maps.push(Map {
                        source: source_temp.to_string(),
                        destination: destination_temp.to_string(),
                        ranges: ranges_temp.drain(..).collect(),
                    });
                }
                continue;
            } else if line.starts_with("seeds:") {
                let (_, numbers) = line.split_once(": ").unwrap();
                seeds = numbers.split(" ").map(|n| n.parse().unwrap()).collect();
            } else if line.contains("map:") {
                let (mapping, _) = line.split_once(" ").unwrap();
                (source_temp, destination_temp) = mapping.split_once("-to-").unwrap();
            } else {
                let numbers: Vec<usize> = line.split(" ").map(|n| n.parse().unwrap()).collect();
                ranges_temp.push(MapRange {
                    source_start: numbers[1],
                    destination_start: numbers[0],
                    length: numbers[2],
                });
            }
        }
        if ranges_temp.len() > 0 {
            maps.push(Map {
                source: source_temp.to_string(),
                destination: destination_temp.to_string(),
                ranges: ranges_temp.drain(..).collect(),
            });
        }

        Almanac { seeds, maps }
    }

    fn seed_ranges(&self) -> Vec<(usize, usize)> {
        let mut seed_ranges = Vec::new();
        for i in 0..self.seeds.len() / 2 {
            let start = self.seeds[i * 2];
            let end = self.seeds[i * 2] + self.seeds[i * 2 + 1];
            seed_ranges.push((start, end));
        }
        seed_ranges
    }
}

impl MapRange {
    fn destination(&self, source: &usize) -> usize {
        if source >= &self.source_start && source < &(self.source_start + self.length) {
            let diff = source - self.source_start;
            self.destination_start + diff
        } else {
            *source
        }
    }
}
