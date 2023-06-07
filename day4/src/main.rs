use itertools::Itertools;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Range;

fn main() {
    let reg_pairs = Regex::new(r"^(\d+)\-(\d+),(\d+)\-(\d+)$").unwrap();

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
                capture
                    .iter()
                    .skip(1)
                    .map(|c| c.unwrap().as_str().parse::<i32>().unwrap())
                    .collect_vec()
            })
            .filter(|v| (v[2] >= v[0] && v[3] <= v[1]) || (v[0] >= v[2] && v[1] <= v[3]))
            .collect_vec()
            .len()
    );

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
                let values = capture
                    .iter()
                    .skip(1)
                    .map(|c| c.unwrap().as_str().parse::<i32>().unwrap())
                    .collect_vec();

                if values[0] <= values[2] {
                    ((values[0], values[1]), (values[2], values[3]))
                } else {
                    ((values[2], values[3]), (values[0], values[1]))
                }
            })
            .filter(|v| v.0.1 >= v.1.0)
            .collect_vec()
            .len()
    );
}
