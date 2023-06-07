use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DijkstraState {
    distance: u32,
    position: Coord,
}

fn main() {
    let input = File::open("input.txt").unwrap();
    let lines = io::BufReader::new(input).lines();

    let map = lines
        .into_iter()
        .map(|line| line.unwrap().as_bytes().to_vec())
        .collect_vec();

    let starting_position = find_in_map(&map, b'S').expect("No starting position found");
    let target_position = find_in_map(&map, b'E').expect("No target position found");

    println!(
        "Shortest distance from 'S': '{:?}'",
        find_distance(&map, starting_position, target_position)
    );

    let mut shortest_distance: u32 = u32::max_value();

    for iy in 0..map.len() {
        for ix in 0..map[iy].len() {
            if map[iy][ix] == b'a' {
                if let Some(distance) = find_distance(&map, Coord { x: ix, y: iy }, target_position)
                {
                    if distance < shortest_distance {
                        shortest_distance = distance;
                    }
                }
            }
        }
    }

    println!("Shortest distance from any 'a': '{:?}'", shortest_distance);
}

fn find_distance(
    map: &Vec<Vec<u8>>,
    starting_position: Coord,
    target_position: Coord,
) -> Option<u32> {
    let mut dijkstra_states: Vec<DijkstraState> = vec![DijkstraState {
        distance: 0,
        position: starting_position,
    }];

    let mut already_visited: Vec<Coord> = vec![];

    loop {
        let nearest_position = dijkstra_states.pop()?;

        if nearest_position.position.x > 0 {
            let possible_step = Coord {
                x: nearest_position.position.x - 1,
                y: nearest_position.position.y,
            };

            if can_step(
                &map,
                &already_visited,
                &nearest_position.position,
                &possible_step,
            ) {
                if possible_step == target_position {
                    return Some(nearest_position.distance + 1);
                }

                dijkstra_states.insert(
                    0,
                    DijkstraState {
                        distance: nearest_position.distance + 1,
                        position: possible_step,
                    },
                );

                already_visited.push(possible_step);
            }
        }

        if nearest_position.position.x < map[nearest_position.position.y].len() - 1 {
            let possible_step = Coord {
                x: nearest_position.position.x + 1,
                y: nearest_position.position.y,
            };

            if can_step(
                &map,
                &already_visited,
                &nearest_position.position,
                &possible_step,
            ) {
                if possible_step == target_position {
                    return Some(nearest_position.distance + 1);
                }

                dijkstra_states.insert(
                    0,
                    DijkstraState {
                        distance: nearest_position.distance + 1,
                        position: possible_step,
                    },
                );

                already_visited.push(possible_step);
            }
        }

        if nearest_position.position.y > 0 {
            let possible_step = Coord {
                x: nearest_position.position.x,
                y: nearest_position.position.y - 1,
            };

            if can_step(
                &map,
                &already_visited,
                &nearest_position.position,
                &possible_step,
            ) {
                if possible_step == target_position {
                    return Some(nearest_position.distance + 1);
                }

                dijkstra_states.insert(
                    0,
                    DijkstraState {
                        distance: nearest_position.distance + 1,
                        position: possible_step,
                    },
                );

                already_visited.push(possible_step);
            }
        }

        if nearest_position.position.y < map.len() - 1 {
            let possible_step = Coord {
                x: nearest_position.position.x,
                y: nearest_position.position.y + 1,
            };

            if can_step(
                &map,
                &already_visited,
                &nearest_position.position,
                &possible_step,
            ) {
                if possible_step == target_position {
                    return Some(nearest_position.distance + 1);
                }

                dijkstra_states.insert(
                    0,
                    DijkstraState {
                        distance: nearest_position.distance + 1,
                        position: possible_step,
                    },
                );

                already_visited.push(possible_step);
            }
        }
    }
}

fn find_in_map(map: &Vec<Vec<u8>>, char_to_find: u8) -> Option<Coord> {
    for iy in 0..map.len() {
        for ix in 0..map[iy].len() {
            if map[iy][ix] == char_to_find {
                return Some(Coord { x: ix, y: iy });
            }
        }
    }

    None
}

fn get_map_char(map: &Vec<Vec<u8>>, coord: &Coord) -> u8 {
    let possible = map[coord.y][coord.x];

    if possible == b'S' {
        return b'a';
    }

    possible
}

fn can_step(map: &Vec<Vec<u8>>, previous: &Vec<Coord>, current: &Coord, to: &Coord) -> bool {
    if !previous.contains(to) {
        let distance = get_map_char(&map, to).checked_sub(get_map_char(&map, current));

        if distance.is_none() || distance.unwrap() <= 1 {
            return true;
        }
    }

    false
}
