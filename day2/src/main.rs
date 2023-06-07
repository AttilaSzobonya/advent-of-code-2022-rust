use modular::Modular;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let reg_pairs = Regex::new(r"^(\w)\s+(\w)$").unwrap();

    let input = File::open("input.txt").unwrap();
    let lines = io::BufReader::new(input).lines();

    println!(
        "{:?}",
        lines
            .into_iter()
            // All the lines parsed with regex and the letters stored in a tuple
            .map(|l| {
                let line_string = &l.unwrap();
                let capture = reg_pairs.captures(line_string).unwrap();
                (capture[1].to_string(), capture[2].to_string())
            })
            // The tuple strings are interpreted
            .map(|(opponent, me)| (
                match opponent.as_str() {
                    "A" => 0.to_modulo(3), // Rock
                    "B" => 1.to_modulo(3), // Paper
                    "C" => 2.to_modulo(3), // Sissors
                    _ => panic!("This element shouldn't exist"),
                },
                match me.as_str() {
                    "X" => 0.to_modulo(3), // Rock
                    "Y" => 1.to_modulo(3), // Paper
                    "Z" => 2.to_modulo(3), // Sissors
                    _ => panic!("This element shouldn't exist"),
                }
            ))
            // Calculate the point of each round
            .map(|(opponent, me)| match (me - opponent).remainder() {
                1 => 6, // This is when we win
                0 => 3, // This is draw
                2 => 0, // This is losing
                _ => panic!("This modulo doesn't exsist"),
            } + me.remainder()
                + 1)
            .sum::<i32>()
    );

    let input = File::open("input.txt").unwrap();
    let lines = io::BufReader::new(input).lines();

    println!(
        "{:?}",
        lines
            .into_iter()
            .map(|l| {
                let line_string = &l.unwrap();
                let capture = reg_pairs.captures(line_string).unwrap();
                (capture[1].to_string(), capture[2].to_string())
            })
            .map(|(opponent, expected)| (
                match opponent.as_str() {
                    "A" => 0.to_modulo(3), // Rock
                    "B" => 1.to_modulo(3), // Paper
                    "C" => 2.to_modulo(3), // Sissors
                    _ => panic!("This element shouldn't exist"),
                },
                match expected.as_str() {
                    "X" => 2.to_modulo(3), // We lose
                    "Y" => 0.to_modulo(3), // Draw
                    "Z" => 1.to_modulo(3), // We win
                    _ => panic!("This element shouldn't exist"),
                }
            ))
            .map(|(opponent, expected)| (opponent, opponent + expected))
            // Calculate the point of each round
            .map(|(opponent, me)| match (me - opponent).remainder() {
                1 => 6, // This is when we win
                0 => 3, // This is draw
                2 => 0, // This is losing
                _ => panic!("This modulo doesn't exsist"),
            } + me.remainder() + 1)
            .sum::<i32>()
    );
}
