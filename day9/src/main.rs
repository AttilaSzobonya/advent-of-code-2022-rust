use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::vec;

#[derive(Debug, Clone)]
enum Step {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq)]
struct Coord {
    x: i32,
    y: i32,
}

fn read_steps() -> impl Iterator<Item = Step> {
    lazy_static! {
        static ref REG_STEP: Regex = Regex::new(r"^([UDRL])\s(\d+)$").unwrap();
    }

    let input = File::open("input.txt").unwrap();
    io::BufReader::new(input)
        .lines()
        .into_iter()
        .map(|l| l.unwrap())
        .flat_map(|l| {
            let captures = REG_STEP.captures(&l).unwrap();
            let number_of_steps = captures[2].parse::<usize>().unwrap();

            std::iter::repeat(match &captures[1] {
                "U" => Step::Up,
                "D" => Step::Down,
                "L" => Step::Left,
                "R" => Step::Right,
                _ => panic!("Unknown direction"),
            })
            .take(number_of_steps)
        })
}

fn move_snake(snake_length: usize, steps: impl Iterator<Item = Step>) -> Vec<Coord> {
    let mut snake = std::iter::repeat(Coord { x: 0, y: 0 })
        .take(snake_length)
        .collect_vec();

    let mut tail_visited_list: Vec<Coord> = vec![];

    for step in steps {
        match step {
            Step::Up => snake[0].y += 1,
            Step::Down => snake[0].y -= 1,
            Step::Right => snake[0].x += 1,
            Step::Left => snake[0].x -= 1,
        };

        for ix in 1..snake_length {
            let part_distance = (snake[ix - 1].x - snake[ix].x, snake[ix - 1].y - snake[ix].y);

            match part_distance {
                (2, 0) => snake[ix].x += 1,
                (0, 2) => snake[ix].y += 1,
                (-2, 0) => snake[ix].x -= 1,
                (0, -2) => snake[ix].y -= 1,
                (2, 1) | (1, 2) | (2, 2) => {
                    snake[ix].x += 1;
                    snake[ix].y += 1
                }
                (-2, -1) | (-1, -2) | (-2, -2) => {
                    snake[ix].x -= 1;
                    snake[ix].y -= 1
                }
                (-2, 1) | (-1, 2) | (-2, 2)=> {
                    snake[ix].x -= 1;
                    snake[ix].y += 1
                }
                (2, -1) | (1, -2) | (2, -2)=> {
                    snake[ix].x += 1;
                    snake[ix].y -= 1
                }
                _ => {}
            }
        }

        if !tail_visited_list.contains(&snake[snake_length - 1]) {
            tail_visited_list.insert(0, snake[snake_length - 1].clone());
        }
    }

    tail_visited_list
}

fn main() {
    println!("{:?}", move_snake(2, read_steps()).len());
    println!("{:?}", move_snake(10, read_steps()).len());
}
