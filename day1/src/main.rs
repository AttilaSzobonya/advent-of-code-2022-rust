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
            .group_by(|elt| *elt.as_ref().unwrap() != "")
            .into_iter()
            .filter(|(key, _)| *key)
            .enumerate()
            .map(|(ix, (_, val))| (ix, val.map(|f| f.unwrap().parse::<i32>().unwrap())))
            .map(|(ix, it)| (ix, it.sum::<i32>()))
            .minmax_by(|o1, o2| o1.1.cmp(&o2.1))
    );

    let input = File::open("input.txt").unwrap();
    let lines = io::BufReader::new(input).lines();

    println!(
        "{:?}",
        lines
            .into_iter()
            .group_by(|elt| *elt.as_ref().unwrap() != "")
            .into_iter()
            .filter(|(key, _)| *key)
            .map(|(_, val)| val.map(|f| f.unwrap().parse::<i32>().unwrap()))
            .map(|it| it.sum::<i32>())
            .sorted()
            .rev()
            .take(3)
            .sum::<i32>()
    );
}
