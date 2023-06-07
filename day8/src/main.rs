use std::fs::File;
use std::io::{self, BufRead};

use itertools::Itertools;

fn main() {
    let input = File::open("input.txt").unwrap();
    let trees = io::BufReader::new(input)
        .lines()
        .into_iter()
        .map(|l| l.unwrap())
        .map(|l| l.bytes().collect_vec())
        .collect_vec();

    let mut visible_trees = 0;

    for ix_line in 0..trees.len() {
        for ix_col in 0..trees[ix_line].len() {
            if check_if_visible(&trees, ix_line, ix_col) {
                visible_trees += 1;
            }
        }
    }

    println!("{}", visible_trees);

    let input = File::open("input.txt").unwrap();
    let trees = io::BufReader::new(input)
        .lines()
        .into_iter()
        .map(|l| l.unwrap())
        .map(|l| l.bytes().collect_vec())
        .collect_vec();

    let max_scenic_value = trees.iter().enumerate().map(|(iy, l)| {
        l.iter()
            .enumerate()
            .map(|(ix, _)| calculate_scenic_score(&trees, iy, ix))
            .max().unwrap()
    }).max().unwrap();

    println!("{:?}", max_scenic_value);
}

fn check_if_visible(trees: &Vec<Vec<u8>>, start_y: usize, start_x: usize) -> bool {
    if start_x == 0 {
        return true;
    }

    if start_y == 0 {
        return true;
    }

    if start_y == trees.len() - 1 {
        return true;
    }

    if start_x == trees[start_y].len() - 1 {
        return true;
    }

    let start_tree_height = trees[start_y][start_x];
    let mut left_alive = true;
    let mut right_alive = true;
    let mut up_alive = true;
    let mut down_alive = true;
    let mut distance = 1;

    while left_alive | right_alive | up_alive | down_alive {
        if left_alive {
            if let Some(x) = start_x.checked_sub(distance) {
                if trees[start_y][x] >= start_tree_height {
                    left_alive = false;
                }
            } else {
                return true;
            }
        }

        if right_alive {
            if let Some(right) = trees[start_y].get(start_x + distance) {
                if *right >= start_tree_height {
                    right_alive = false;
                }
            } else {
                return true;
            }
        }

        if up_alive {
            if let Some(y) = start_y.checked_sub(distance) {
                if trees[y][start_x] >= start_tree_height {
                    up_alive = false;
                }
            } else {
                return true;
            }
        }

        if down_alive {
            if let Some(down) = trees.get(start_y + distance) {
                if (*down)[start_x] >= start_tree_height {
                    down_alive = false;
                }
            } else {
                return true;
            }
        }

        distance += 1;
    }

    return false;
}

fn calculate_scenic_score(trees: &Vec<Vec<u8>>, start_y: usize, start_x: usize) -> i32 {
    if start_x == 0 {
        return 0;
    }

    if start_y == 0 {
        return 0;
    }

    if start_y == trees.len() - 1 {
        return 0;
    }

    if start_x == trees[start_y].len() - 1 {
        return 0;
    }

    let start_tree_height = trees[start_y][start_x];
    let mut left_alive = true;
    let mut right_alive = true;
    let mut up_alive = true;
    let mut down_alive = true;

    let mut left_visibility = 0;
    let mut right_visibility = 0;
    let mut up_visibility = 0;
    let mut down_visibility = 0;

    let mut distance = 1;

    while left_alive | right_alive | up_alive | down_alive {
        if left_alive {
            
            if let Some(x) = start_x.checked_sub(distance) {
                left_visibility += 1;

                if trees[start_y][x] >= start_tree_height {
                    left_alive = false;
                }
            } else {
                left_alive = false;
            }
        }

        if right_alive {
            if let Some(right) = trees[start_y].get(start_x + distance) {
                right_visibility += 1;

                if *right >= start_tree_height {
                    right_alive = false;
                }
            } else {
                right_alive = false;
            }
        }

        if up_alive {
            if let Some(y) = start_y.checked_sub(distance) {
                up_visibility += 1;

                if trees[y][start_x] >= start_tree_height {
                    up_alive = false;
                }
            } else {
                up_alive = false;
            }
        }

        if down_alive {
            if let Some(down) = trees.get(start_y + distance) {
                down_visibility += 1;

                if (*down)[start_x] >= start_tree_height {
                    down_alive = false;
                }
            } else {
                down_alive = false;
            }
        }

        distance += 1;
    }

    return left_visibility * right_visibility * up_visibility * down_visibility;
}
