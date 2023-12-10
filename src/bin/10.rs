use std::fs;

fn main() {
    let input = fs::read_to_string("./inputs/10").expect("Input should exist");

    part_one(&input);
}

fn part_one(input: &str) {
    let mut steps: usize = 1;
    let maze = Maze::new(input);
    let (mut dir, mut tile) = maze.next(None, &maze.start);
    while tile.value != 'S' {
        (dir, tile) = maze.next(Some(dir), tile);
        steps += 1;
    }

    let farthest_point = steps / 2;

    println!("Part 1: {farthest_point}")
}

#[derive(Debug)]
struct Maze {
    start: Tile,
    tiles: Vec<Vec<Tile>>,
}

#[derive(Clone, Debug)]
struct Tile {
    row: usize,
    col: usize,
    value: char,
}

#[derive(Clone, Debug, PartialEq)]
enum Direction {
    West,
    North,
    East,
    South,
}

impl Maze {
    fn new(input: &str) -> Self {
        let mut tiles: Vec<Vec<Tile>> = Vec::new();
        let mut start = Tile {
            row: 0,
            col: 0,
            value: 'S',
        };
        for (row, line) in input.lines().enumerate() {
            for (col, value) in line.chars().enumerate() {
                if tiles.len() <= col {
                    tiles.push(vec![]);
                }
                if value == 'S' {
                    start = Tile { row, col, value };
                }
                tiles[row].push(Tile { row, col, value });
            }
        }

        Maze { start, tiles }
    }

    fn next(&self, prev: Option<Direction>, curr: &Tile) -> (Direction, &Tile) {
        if prev.is_none() {
            // Starting point, manually set the next tile because why not
            return (Direction::North, &self.tiles[curr.row + 1][curr.col]);
        } else {
            let prev_direction = prev.unwrap();
            let curr_directions = curr.directions();
            let next_direction = curr_directions
                .iter()
                .find(|&d| d != &prev_direction)
                .unwrap();

            match next_direction {
                Direction::West => (Direction::East, &self.tiles[curr.row][curr.col - 1]),
                Direction::North => (Direction::South, &self.tiles[curr.row - 1][curr.col]),
                Direction::East => (Direction::West, &self.tiles[curr.row][curr.col + 1]),
                Direction::South => (Direction::North, &self.tiles[curr.row + 1][curr.col]),
            }
        }
    }
}

impl Tile {
    fn directions(&self) -> Vec<Direction> {
        match self.value {
            '|' => vec![Direction::North, Direction::South],
            '-' => vec![Direction::West, Direction::East],
            'L' => vec![Direction::North, Direction::East],
            'J' => vec![Direction::West, Direction::North],
            '7' => vec![Direction::West, Direction::South],
            'F' => vec![Direction::East, Direction::South],
            'S' => vec![Direction::North, Direction::South],
            _ => vec![],
        }
    }
}
