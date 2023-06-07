use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::vec;

#[derive(Debug, Clone)]
enum Instruction {
    Noop,
    Addx(i32),
}

fn read_instructions() -> impl Iterator<Item = Instruction> {
    lazy_static! {
        static ref REG_STEP: Regex = Regex::new(r"^(\S+)(?:\s(-?\d+))?$").unwrap();
    }

    let input = File::open("input.txt").unwrap();
    io::BufReader::new(input)
        .lines()
        .into_iter()
        .map(|l| l.unwrap())
        .flat_map(|l| {
            let captures = REG_STEP.captures(&l).unwrap();

            if &captures[1] == "noop" {
                vec![Instruction::Noop]
            } else if &captures[1] == "addx" {
                vec![
                    Instruction::Noop,
                    Instruction::Addx(captures[2].parse::<i32>().unwrap()),
                ]
            } else {
                panic!("Unknown instruction")
            }
        })
}

fn main() {
    let mut x = 1;
    let mut cycle = 0;
    let mut signal_strength_sum = 0;

    for instruction in read_instructions() {
        cycle += 1;

        if (cycle - 20) % 40 == 0 {
            signal_strength_sum += cycle * x;
        }

        match instruction {
            Instruction::Noop => {}
            Instruction::Addx(op) => {
                x += op;
            }
        }

        let horizontal_pixel = cycle % 40;

        if horizontal_pixel == 0 {
            println!();
        }

        if (x - 1..=x + 1).contains(&horizontal_pixel) {
            print!("#");
        } else {
            print!(" ");
        }
    }

    println!("Signal strength sum: {}", signal_strength_sum);
}
