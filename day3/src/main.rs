use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let input = File::open("input.txt").unwrap();
    let lines = io::BufReader::new(input).lines();

    println!(
        "{:?}",
        lines
            .into_iter()
            .map(|line| {
                let line = line.unwrap();
                (
                    line[..line.len() / 2].to_string(),
                    line[line.len() / 2..].to_string(),
                )
            })
            .map(|(ruck1, ruck2)| find_common_char(ruck1, ruck2)
                .expect("There are no common elements in these rucksacks"))
            .map(|common| match common {
                65..=90 => common - 38,
                97..=122 => common - 96,
                _ => panic!("Invalid character"),
            } as i32)
            .sum::<i32>()
    );

    let input = File::open("input.txt").unwrap();
    let lines = io::BufReader::new(input).lines();

    println!(
        "{:?}",
        lines
            .into_iter()
            .map(|line| line.unwrap().bytes().collect_vec())
            .collect_vec()
            .chunks(3)
            .map(|chunk| find_common_char_general(chunk).unwrap().into_iter().unique().collect::<Vec<_>>()[0])
            .map(|common| match common {
                65..=90 => common - 38,
                97..=122 => common - 96,
                _ => panic!("Invalid character"),
            } as i32)
            .sum::<i32>()
    );
}

fn find_common_char(op1: String, op2: String) -> Option<u8> {
    let op1 = op1.as_bytes();
    let op2 = op2.as_bytes();

    for current_op1 in op1 {
        for current_op2 in op2 {
            if current_op1 == current_op2 {
                return Some(*current_op1);
            }
        }
    }

    return None;
}

fn find_common_char_general<T>(op: &[Vec<T>]) -> Option<Vec<T>>
where
    T: std::clone::Clone,
    T: std::cmp::PartialEq,
{
    let mut current_common = op[0].to_vec();
    let mut next_common: Vec<T>;

    for enumerable in op.into_iter().skip(1) {
        next_common = Vec::new();

        for element in &current_common {
            if enumerable.iter().contains(&element) {
                next_common.push(element.clone());
            }
        }

        current_common = next_common.clone();
    }

    if current_common.len() > 0 {
        Some(current_common)
    } else {
        None
    }
}
