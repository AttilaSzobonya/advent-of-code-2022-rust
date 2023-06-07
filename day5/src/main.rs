use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::vec;

fn main() {
    let reg_pairs = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    let input = File::open("input.txt").unwrap();
    let lines = io::BufReader::new(input).lines().into_iter();

    let mut crates = vec![
        vec!['W', 'B', 'D', 'N', 'C', 'F', 'J'],
        vec!['P', 'Z', 'V', 'Q', 'L', 'S', 'T'],
        vec!['P', 'Z', 'B', 'G', 'J', 'T'],
        vec!['D', 'T', 'L', 'J', 'Z', 'B', 'H', 'C'],
        vec!['G', 'V', 'B', 'J', 'S'],
        vec!['P', 'S', 'Q'],
        vec!['B', 'V', 'D', 'F', 'L', 'M', 'P', 'N'],
        vec!['P', 'S', 'M', 'F', 'B', 'D', 'L', 'R'],
        vec!['V', 'D', 'T', 'R'],
    ];

    for line in lines {
        let line = line.unwrap();

        let captures = reg_pairs.captures(line.as_str()).unwrap();

        move_items_9000(
            &mut crates,
            captures[1].to_string().parse::<u8>().unwrap(),
            captures[2].to_string().parse::<usize>().unwrap() - 1,
            captures[3].to_string().parse::<usize>().unwrap() - 1,
        );
    }

    for cr in crates {
        print!("{:?}", cr.last().unwrap());
    }
    println!();

    let input = File::open("input.txt").unwrap();
    let lines = io::BufReader::new(input).lines().into_iter();

    let mut crates = vec![
        vec!['W', 'B', 'D', 'N', 'C', 'F', 'J'],
        vec!['P', 'Z', 'V', 'Q', 'L', 'S', 'T'],
        vec!['P', 'Z', 'B', 'G', 'J', 'T'],
        vec!['D', 'T', 'L', 'J', 'Z', 'B', 'H', 'C'],
        vec!['G', 'V', 'B', 'J', 'S'],
        vec!['P', 'S', 'Q'],
        vec!['B', 'V', 'D', 'F', 'L', 'M', 'P', 'N'],
        vec!['P', 'S', 'M', 'F', 'B', 'D', 'L', 'R'],
        vec!['V', 'D', 'T', 'R'],
    ];

    for line in lines {
        let line = line.unwrap();

        let captures = reg_pairs.captures(line.as_str()).unwrap();

        move_items_9001(
            &mut crates,
            captures[1].to_string().parse::<u8>().unwrap(),
            captures[2].to_string().parse::<usize>().unwrap() - 1,
            captures[3].to_string().parse::<usize>().unwrap() - 1,
        );
    }

    for cr in crates {
        print!("{:?}", cr.last().unwrap());
    }
    println!();
}

fn move_items_9000(vector: &mut Vec<Vec<char>>, number: u8, from: usize, to: usize) {
    for _ in 0..number {
        let pop = vector[from].pop().unwrap();
        vector[to].push(pop);
    }
}

fn move_items_9001(vector: &mut Vec<Vec<char>>, number: u8, from: usize, to: usize) {
    let mut move_vec: Vec<char> = vec![];

    for _ in 0..number {
        let pop = vector[from].pop().unwrap();
        move_vec.insert(0, pop);
    }

    vector[to].append(&mut move_vec);
}
