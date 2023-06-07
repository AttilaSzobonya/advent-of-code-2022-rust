use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Read, Seek};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Valve {
    name: String,
    rate: u32,
    tunnels: Vec<Box<Valve>>,
}

fn initialize<'a>(mut read_buffer: io::BufReader<File>) -> (HashMap<String, Box<Valve>>, String) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^Valve ([A-Z]{2}) has flow rate=(\d+)").unwrap();
        static ref RE2: Regex = Regex::new(r"[A-Z]{2}").unwrap();
    }

    let mut valves: HashMap<String, Box<Valve>> = HashMap::new();
    let mut first_valve: Option<String> = None;

    for line in read_buffer.by_ref().lines() {
        let line = line.unwrap();
        let caps = RE.captures(&line).unwrap();
        let valve_name = caps.get(1).unwrap().as_str();
        let valve_rate = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();

        let valve = Valve {
            name: valve_name.to_owned(),
            rate: valve_rate,
            tunnels: Vec::new(),
        };

        valves.insert(valve_name.to_owned(), Box::new(valve));

        if first_valve.is_none() {
            first_valve = Some(valve_name.to_owned());
        }
    }

    read_buffer.seek(io::SeekFrom::Start(0)).unwrap();

    for line in read_buffer.by_ref().lines() {
        let line = line.unwrap();
        let mut caps = RE2.captures_iter(&line);
        let valve_name = caps.nth(0).unwrap().get(0).unwrap().as_str();

        let mut targets: Vec<Box<Valve>> = Vec::new();
        for cap in caps {
            let tunnel_name = cap.get(0).unwrap().as_str();
            let tunnel = valves.get(tunnel_name).unwrap().as_ref();
            targets.push(Box(tunnel));
        }

        valves.get_mut(valve_name).unwrap().tunnels = targets;
    }

    (valves, first_valve.unwrap())
}

fn traverse<'a>(valve: &'a Valve, rounds: u32) -> u32 {
    println!("Traversing {:?}", valve);

    let valve_opened = valve.rate > 0;

    let total = if valve_opened {
        valve.rate * (rounds - 1)
    } else {
        0
    };

    let mut max_total = 0;
    for tunnel in valve.tunnels.iter() {
        let current_total = traverse(tunnel, if valve_opened { rounds - 2 } else { rounds - 1 });

        if current_total > max_total {
            max_total = current_total;
        }
    }

    total + max_total
}

fn main() {
    let (tunnels, first_valve) =
        initialize(io::BufReader::new(File::open("input_demo.txt").unwrap()));

    println!(
        "Total pressure released: {}",
        traverse(tunnels.get(&first_valve).unwrap(), 30)
    );
}
