use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, Read};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Tile {
    Air,
    Rock,
    Sand,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Coordinate {
    x: usize,
    y: usize,
}

fn str_to_coordinate(s: &str) -> Coordinate {
    let mut split = s.split(",");

    let x = split.next().unwrap().parse::<usize>().unwrap();
    let y = split.next().unwrap().parse::<usize>().unwrap();

    Coordinate { x, y }
}

fn initialize_with_void(mut read_buffer: io::BufReader<File>) -> (Vec<Vec<Tile>>, usize) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\d+,\d+").unwrap();
    }

    let mut map: Vec<Vec<Tile>> = Vec::new();

    for _ in 0..1000 {
        let mut row: Vec<Tile> = Vec::new();
        for _ in 0..1000 {
            row.push(Tile::Air);
        }
        map.push(row);
    }

    let mut max_depth = 0;

    for line in read_buffer.by_ref().lines() {
        let line = line.unwrap();

        let mut previous: Option<Coordinate> = None;
        for m in RE.find_iter(&line) {
            let current = str_to_coordinate(m.as_str());

            if let Some(previous) = previous {
                /* This only works because either x or y is equal */
                let x1 = std::cmp::min(previous.x, current.x);
                let x2 = std::cmp::max(previous.x, current.x);
                let y1 = std::cmp::min(previous.y, current.y);
                let y2 = std::cmp::max(previous.y, current.y);

                if y2 > max_depth {
                    max_depth = y2;
                }

                for x in x1..=x2 {
                    for y in y1..=y2 {
                        map[y][x] = Tile::Rock;
                    }
                }
            }

            previous = Some(current);
        }
    }

    (map, max_depth)
}

fn initialize_with_floor(read_buffer: io::BufReader<File>) -> Vec<Vec<Tile>> {
    let (mut map, max_depth) = initialize_with_void(read_buffer);

    let floor = max_depth + 2;

    for x in 0..map[floor].len() {
        map[floor][x] = Tile::Rock;
    }

    map
}

fn simulate_sand_fall(map: &mut Vec<Vec<Tile>>) -> i32 {
    let mut number_of_sands = 0;

    'main: loop {
        let mut current_new_sand = Coordinate { x: 500, y: 0 };

        'sand: loop {
            if current_new_sand.y + 1 == map.len() {
                break 'main number_of_sands;
            }

            if map[current_new_sand.y + 1][current_new_sand.x] == Tile::Air {
                map[current_new_sand.y][current_new_sand.x] = Tile::Air;
                current_new_sand.y += 1;
                map[current_new_sand.y][current_new_sand.x] = Tile::Sand;
            } else if map[current_new_sand.y + 1][current_new_sand.x - 1] == Tile::Air {
                map[current_new_sand.y][current_new_sand.x] = Tile::Air;
                current_new_sand.x -= 1;
                current_new_sand.y += 1;
                map[current_new_sand.y][current_new_sand.x] = Tile::Sand;
            } else if map[current_new_sand.y + 1][current_new_sand.x + 1] == Tile::Air {
                map[current_new_sand.y][current_new_sand.x] = Tile::Air;
                current_new_sand.x += 1;
                current_new_sand.y += 1;
                map[current_new_sand.y][current_new_sand.x] = Tile::Sand;
            } else {
                break 'sand;
            }
        }

        if current_new_sand.x == 500 && current_new_sand.y == 0 {
            break 'main number_of_sands + 1;
        }

        number_of_sands += 1;
    }
}

fn main() {
    let (mut map, _) = 
        initialize_with_void(io::BufReader::new(File::open("input.txt").unwrap()));
    println!("Number of sands with abyss: {}", simulate_sand_fall(&mut map));

    let mut map = 
        initialize_with_floor(io::BufReader::new(File::open("input.txt").unwrap()));
    println!("Number of sands with floor: {}", simulate_sand_fall(&mut map));
}
