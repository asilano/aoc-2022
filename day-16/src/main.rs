use std::{fs, collections::{HashMap, VecDeque}, cmp::max};
use regex::Regex;

fn load_data() -> String {
    fs::read_to_string("data/data.txt").expect("Should have read")
}

struct Node {
    label: String,
    pressure: u32,
    adjacency: Vec<String>
}
type Network = HashMap<String, Node>;

fn parse_data(data: &String) -> Network {
    let mut network = Network::new();

    let rex = Regex::new(r"^Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? ([\w, ]+)$").unwrap();
    // First pass - add the nodes
    for line in data.lines() {
        let caps = rex.captures(line).unwrap_or_else(|| { println!("{}", line); panic!("") });
        let label = caps.get(1).unwrap().as_str();
        let pressure = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
        let to_labels = caps.get(3).unwrap().as_str().split(", ");

        network.insert(label.to_string(), Node {
            label: label.to_string(),
            pressure,
            adjacency: to_labels.map(|l| l.to_string()).collect()
        });
    }

    network
}

fn part_one(network: &Network) -> u32 {
    let mut best_pressure = 0u32;
    let mut current_pressure = 0u32;
    let mut valves_on = Vec::<String>::new();

    visit(network, "AA".to_string(), &mut valves_on, 30, 0, &mut current_pressure, &mut best_pressure);
    best_pressure
}

fn visit(network: &Network, label: String, valves_on: &mut Vec<String>, timeout: u32, tick: u32, current_pressure: &mut u32, best_pressure: &mut u32) {
    if tick >= timeout {
        return;
    }

    let here = network.get(&label).unwrap();
    *current_pressure += here.pressure * (timeout - tick);
    valves_on.push(label.clone());
    if current_pressure > best_pressure {
        println!("{:?}", valves_on);
    }
    *best_pressure = max(*best_pressure, *current_pressure);
    
    let mut dists = HashMap::<String, u32>::new();
    dists.insert(label.clone(), 1);
    let mut searchfront = VecDeque::<(String, u32)>::new();
    searchfront.push_back((label.clone(), 1));

    while !searchfront.is_empty() {
        let (examine, dist) = searchfront.pop_front().unwrap();
        let examine_node = network.get(&examine).unwrap();
        examine_node.adjacency.iter().for_each(|adj| {
            if !dists.contains_key(adj) {
                dists.insert(adj.clone(), dist + 1);
                searchfront.push_back((adj.clone(), dist));
            }
        });
    }
   
    let valves_clone = valves_on.clone();
    network.keys().filter(|adj| !valves_clone.contains(adj)).for_each(|adj| {
        let dist = dists.get(adj).unwrap();
        visit(network, adj.clone(), valves_on, timeout, tick + *dist, current_pressure, best_pressure); 
    });

    *current_pressure -= here.pressure * (timeout - tick);
    valves_on.pop();
}

fn main() {
    let data = load_data();
    let network = parse_data(&data);

    let pressure = part_one(&network);
    println!("Part one: {}", pressure);
}
