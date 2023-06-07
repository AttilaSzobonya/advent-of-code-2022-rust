use lazy_static::lazy_static;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn new(x: i32, y: i32) -> Self {
        Coordinate { x, y }
    }

    fn distance(&self, other: &Coordinate) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn initialize(read_buffer: io::BufReader<File>) -> (Vec<(Coordinate, i32)>, i32, i32) {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"^Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)$"
        )
        .unwrap();
    }

    let mut map = Vec::new();
    let mut min_x = 0;
    let mut max_x = 0;

    for line in read_buffer.lines() {
        let line = line.unwrap();
        let caps = RE.captures(&line).unwrap();

        let sensor_x = caps[1].parse::<i32>().unwrap();
        let sensor_y = caps[2].parse::<i32>().unwrap();
        let sensor = Coordinate::new(sensor_x, sensor_y);

        let beacon_x = caps[3].parse::<i32>().unwrap();
        let beacon_y = caps[4].parse::<i32>().unwrap();
        let beacon = Coordinate::new(beacon_x, beacon_y);

        let distance_of_sensor_to_beacon = sensor.distance(&beacon);

        min_x = std::cmp::min(std::cmp::min(min_x, sensor_x), beacon_x);
        max_x = std::cmp::max(std::cmp::max(max_x, sensor_x), beacon_x);

        map.push((sensor, distance_of_sensor_to_beacon));
    }

    (map, min_x, max_x)
}

fn main() {
    let (map, min_x, max_x) = initialize(io::BufReader::new(File::open("input.txt").unwrap()));

    let max_distance = *map.iter().map(|(_, distance)| distance).max().unwrap(); // this gives a safe estimateion of the field to check

    let mut not_possible_positions = 0;
    for x in (min_x - max_distance)..=(max_x + max_distance) {
        if map.iter().any(|(sensor, distance)| {
            let beacon = Coordinate::new(x, 2000000);
            sensor.distance(&beacon) <= *distance
        }) {
            not_possible_positions += 1;
        }
    }

    not_possible_positions -= 1; // as there is one beacon in the line
    println!(
        "Not possible positions: {} (substacted 1 as there is one beacon in the line)",
        not_possible_positions
    );
    assert!(not_possible_positions == 5607466);

    (0..=4000000).into_par_iter().for_each(|x| {
        let mut beacon = Coordinate::new(x, 0);

        for y in 0..=4000000 {
            beacon.y = y;

            let mut possible = true;
            for ix in 0..map.len() {
                let (sensor, distance) = &map[ix];

                if sensor.distance(&beacon) <= *distance {
                    possible = false;
                    break;
                }
            }

            if possible {
                println!("Found possible position: ({}, {})", x, y);

                let tuning_frequency = x as i128 * 4000000 + y as i128;
                println!("Tuning frequency: '{}'", tuning_frequency);
                assert!(tuning_frequency == 12543202766584);
            }
        }
    });
}
