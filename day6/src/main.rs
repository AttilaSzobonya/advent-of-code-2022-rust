use std::fs::File;
use std::io::{self, BufRead};
use std::vec;

fn main() {
    let input = File::open("input.txt").unwrap();
    let lines = io::BufReader::new(input).lines().into_iter();

    let mut chars: Vec<char> = vec![];
    let mut ix: usize = 0;

    for line in lines {
        for ch in line.expect("Unable to read line").chars() {
            ix += 1;
            chars.push(ch);

            if chars.len() > 14 {
                chars.remove(0);
            }

            if chars.len() == 14 {
                if is_unique(&chars) {
                    println!("{:?}", ix);
                    println!("{:?}", chars);
                    return;
                }
            }
        }
    }
}

fn is_unique<T: PartialEq>(vector: &Vec<T>) -> bool {
    for i in 0..(vector.len() - 1) {
        for j in i + 1..vector.len() {
            if vector[i] == vector[j] {
                return false;
            }
        }
    }

    return true;
}
