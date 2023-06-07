use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, Read};
use std::vec;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Element {
    Number(u8),
    Vector(Vec<Element>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Order {
    RightOrder,
    WrongOrder,
    Continue,
}

fn read_line_from_buf<R: Read>(reader: &mut io::BufReader<R>) -> Option<Vec<String>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\[|\]|\d+").unwrap();
    }

    let mut buf = String::new();
    let read_bytes = reader.read_line(&mut buf).unwrap();

    if read_bytes == 0 {
        return None;
    }

    Some(
        RE.find_iter(&buf)
            .map(|m| m.as_str().to_owned())
            .collect_vec(),
    )
}

fn read_structure_recursive(to_process: &[String]) -> (Vec<Element>, usize) {
    let mut result: Vec<Element> = vec![];

    let mut ix: usize = 0;
    loop {
        match to_process[ix].as_str() {
            "[" => {
                let (sub_result, processed_items) = read_structure_recursive(&to_process[ix + 1..]);

                ix += processed_items + 1;
                result.push(Element::Vector(sub_result));
            }
            "]" => {
                ix += 1;
                break;
            }
            b => {
                ix += 1;
                result.push(Element::Number(b.parse::<u8>().unwrap()));
            }
        }
    }

    (result, ix)
}

fn compare_elements(a: &Element, b: &Element) -> Order {
    match (a, b) {
        (Element::Number(a), Element::Number(b)) => match a.cmp(b) {
            std::cmp::Ordering::Equal => Order::Continue,
            std::cmp::Ordering::Greater => Order::WrongOrder,
            std::cmp::Ordering::Less => Order::RightOrder,
        },
        (Element::Vector(a), Element::Vector(b)) => {
            match a
                .iter()
                .zip_longest(b.iter())
                .map(|e| match e {
                    itertools::EitherOrBoth::Both(a, b) => compare_elements(a, b),
                    itertools::EitherOrBoth::Left(_) => Order::WrongOrder,
                    itertools::EitherOrBoth::Right(_) => Order::RightOrder,
                })
                .find(|o| *o != Order::Continue)
            {
                Some(o) => o,
                None => Order::Continue,
            }
        }
        (Element::Number(a), b) => compare_elements(&Element::Vector(vec![Element::Number(*a)]), b),
        (a, Element::Number(b)) => compare_elements(a, &Element::Vector(vec![Element::Number(*b)])),
    }
}

fn main() {
    let input = File::open("input.txt").unwrap();
    let mut read_buffer = io::BufReader::new(input);

    let mut parsed_lines: Vec<Element> = vec![];
    let mut sum_of_indices: usize = 0;
    let mut ix: usize = 0;

    while let Some(line) = read_line_from_buf(&mut read_buffer) {
        if line.len() == 0 {
            continue;
        }

        match parsed_lines.len() % 2 == 1 {
            false => parsed_lines.push(Element::Vector(read_structure_recursive(&line[1..]).0)),
            true => {
                ix += 1;

                let current = Element::Vector(read_structure_recursive(&line[1..]).0);

                sum_of_indices += match compare_elements(parsed_lines.last().unwrap(), &current) {
                    Order::RightOrder => ix,
                    Order::WrongOrder => 0,
                    Order::Continue => panic!("Unexpected continue"),
                };

                parsed_lines.push(current);
            }
        }
    }

    println!("Sum of indices: {}", sum_of_indices);

    assert_eq!(sum_of_indices, 6423);

    parsed_lines.sort_by(|a, b| match compare_elements(a, b) {
        Order::RightOrder => std::cmp::Ordering::Less,
        Order::WrongOrder => std::cmp::Ordering::Greater,
        Order::Continue => std::cmp::Ordering::Equal,
    });

    let find1 = Element::Vector(vec![Element::Vector(vec![Element::Number(2)])]);
    let find2 = Element::Vector(vec![Element::Vector(vec![Element::Number(6)])]);

    let index_of_first = parsed_lines.iter().position(|e| *e == find1);
    let index_of_second = parsed_lines.iter().position(|e| *e == find2);

    println!("Index of [[2]]: {:?}", index_of_first);
    println!("Index of [[6]]: {:?}", index_of_second);

    println!(
        "Product: {}",
        (index_of_first.unwrap() + 1) * (index_of_second.unwrap() + 1)
    );
}
