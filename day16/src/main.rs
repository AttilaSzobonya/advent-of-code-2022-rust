use itertools::Itertools;
use lazy_static::lazy_static;
use petgraph::visit::IntoNodeReferences;
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Read, Seek};

use petgraph::graph::{NodeIndex, UnGraph};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Valve {
    name: String,
    rate: u32,
}

fn initialize(mut read_buffer: io::BufReader<File>) -> (UnGraph<Valve, ()>, NodeIndex) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^Valve ([A-Z]{2}) has flow rate=(\d+)").unwrap();
        static ref RE2: Regex = Regex::new(r"[A-Z]{2}").unwrap();
    }

    let mut valves: UnGraph<Valve, ()> = UnGraph::new_undirected();
    let mut valve_map: HashMap<String, NodeIndex> = HashMap::new();

    for line in read_buffer.by_ref().lines() {
        let line = line.unwrap();
        let caps = RE.captures(&line).unwrap();
        let valve_name = caps.get(1).unwrap().as_str();
        let valve_rate = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();

        let valve = Valve {
            name: valve_name.to_owned(),
            rate: valve_rate,
        };

        let valve_node_index = valves.add_node(valve);
        valve_map.insert(valve_name.to_owned(), valve_node_index);
    }

    let first_valve = *valve_map.get("AA").unwrap();

    read_buffer.seek(io::SeekFrom::Start(0)).unwrap();

    for line in read_buffer.by_ref().lines() {
        let line = line.unwrap();
        let mut caps = RE2.captures_iter(&line);

        let valve_name: &str = caps.next().unwrap().get(0).unwrap().as_str();
        let source_valve = valve_map.get(valve_name).unwrap();

        for cap in caps {
            let tunnel_name = cap.get(0).unwrap().as_str();

            valves.update_edge(*source_valve, *valve_map.get(tunnel_name).unwrap(), ());
        }
    }

    (valves, first_valve)
}

fn traverse(
    graph: &UnGraph<Valve, ()>,
    current_valve_ix: NodeIndex,
    rounds: u32,
    allowed_valves: &[NodeIndex],
    previous_valve: Option<NodeIndex>,
) -> u32 {
    if rounds <= 1 {
        return 0;
    }

    let current_valve = graph.node_weight(current_valve_ix).unwrap();

    let is_valve_opened = current_valve.rate > 0 && allowed_valves.contains(&current_valve_ix);

    let total_pressure_release = if is_valve_opened {
        current_valve.rate * (rounds - 1)
    } else {
        0
    };

    let rounds = if is_valve_opened {
        rounds - 2
    } else {
        rounds - 1
    };

    let allowed_valves: Vec<NodeIndex> = if is_valve_opened {
        allowed_valves
            .iter()
            .copied()
            .filter(|x| *x != current_valve_ix)
            .collect_vec()
    } else {
        allowed_valves.iter().copied().collect_vec()
    };

    let mut max_total = 0;

    for tunnel in graph.neighbors(current_valve_ix) {
        // When we didn't open the valve, we don't want to go back to the previous valve
        // that would be a waste of time and would make the problem space explode
        if !is_valve_opened && Some(tunnel) == previous_valve {
            continue;
        }

        let current_total = traverse(
            graph,
            tunnel,
            rounds,
            &allowed_valves,
            Some(current_valve_ix),
        );

        if current_total > max_total {
            max_total = current_total;
        }
    }

    total_pressure_release + max_total
}

fn main() {
    let source_file = "input.txt";

    let (grpah, first_valve) = initialize(io::BufReader::new(File::open(source_file).unwrap()));

    let allowed_valves = grpah
        .node_references()
        .filter(|(_, v)| v.rate > 0)
        .map(|(ix, _)| ix)
        .collect_vec();

    let total_pressure_release = traverse(&grpah, first_valve, 30, &allowed_valves, None);

    //assert_eq!(total_pressure_release, 1947);

    println!("Total pressure released: {}", total_pressure_release);

    let minutes = 26;
    let total_mask = 2u32.pow(allowed_valves.len() as u32) - 1;

    let max_total_release = (1..total_mask)
        .into_par_iter()
        .map(|mask| {
            let mut first_group: Vec<NodeIndex> = Vec::new();
            let mut second_group: Vec<NodeIndex> = Vec::new();

            for (ix, item) in allowed_valves.iter().enumerate() {
                if mask & (1 << ix) != 0 {
                    first_group.push(*item);
                } else {
                    second_group.push(*item);
                }
            }

            let total_pressure_release_first =
                traverse(&grpah, first_valve, minutes, &first_group, None);
            let total_pressure_release_second =
                traverse(&grpah, first_valve, minutes, &second_group, None);

            total_pressure_release_first + total_pressure_release_second
        })
        .max();

    println!(
        "Max total pressure released: {}",
        max_total_release.unwrap()
    );
}
