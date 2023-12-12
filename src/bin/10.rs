use std::fs;

fn main() {
    let input = fs::read_to_string("./inputs/10").expect("Input should exist");

    part_one(&input);
    part_two(&input);
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

fn part_two(input: &str) {
    let maze = Maze::new(input);
    let (mut dir, mut tile) = maze.next(None, &maze.start);
    let mut edges = vec![tile];
    while tile.value != 'S' {
        (dir, tile) = maze.next(Some(dir), tile);
        edges.push(tile);
    }

    let mut count = 0;
    for row in &maze.tiles {
        for t in row {
            if maze.is_tile_in_loop(t, &edges) {
                count += 1;
            }
        }
    }

    println!("Part 2: {count}")
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

    fn is_tile_in_loop(&self, tile: &Tile, edges: &Vec<&Tile>) -> bool {
        if tile.is_edge(edges) {
            false
        } else {
            let mut is_in = false;
            let mut previous_intersection_direction = None;
            for row_tile in &self.tiles[tile.row] {
                if tile.is_same(&row_tile) {
                    break;
                } else if row_tile.is_edge(edges) {
                    let tile_directions = row_tile.directions();
                    if tile_directions.contains(&Direction::North)
                        && tile_directions.contains(&Direction::South)
                    {
                        is_in = !is_in;
                    } else if tile_directions.contains(&Direction::North) {
                        if let Some(Direction::South) = previous_intersection_direction {
                            is_in = !is_in;
                        } else {
                            previous_intersection_direction = Some(Direction::North);
                        }
                    } else if tile_directions.contains(&Direction::South) {
                        if let Some(Direction::North) = previous_intersection_direction {
                            is_in = !is_in;
                        } else {
                            previous_intersection_direction = Some(Direction::South);
                        }
                    }
                }
            }
            is_in
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

    fn is_edge(&self, edges: &Vec<&Tile>) -> bool {
        edges.iter().any(|&e| e.is_same(self))
    }

    fn is_same(&self, tile: &Tile) -> bool {
        tile.row == self.row && tile.col == self.col
    }
}
