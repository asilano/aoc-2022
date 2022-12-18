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
type DistTable = HashMap<String, HashMap<String, u32>>;

fn parse_data(data: &String) -> (Network, DistTable) {
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

    let mut dist_table = DistTable::new();
    for label in network.keys() {
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
	                searchfront.push_back((adj.clone(), dist + 1));
	            }
	        });
	    }
        dist_table.insert(label.clone(), dists);
    }
   
    (network, dist_table)
}

fn part_one(network: &Network, dist_table: &DistTable) -> u32 {
    let mut best_pressure = 0u32;
    let mut current_pressure = 0u32;
    let mut valves_on = Vec::<String>::new();

    visit(network, dist_table, "AA".to_string(), &mut valves_on, 30, 0, &mut current_pressure, &mut best_pressure);
    best_pressure
}

fn visit(network: &Network, dist_table: &DistTable, label: String, valves_on: &mut Vec<String>, timeout: u32, tick: u32, current_pressure: &mut u32, best_pressure: &mut u32) {
    if tick >= timeout {
        return;
    }

    let here = network.get(&label).unwrap();
    *current_pressure += here.pressure * (timeout - tick);
    valves_on.push(label.clone());
    if current_pressure > best_pressure {
        //println!("{:?}", valves_on);
    }
    *best_pressure = max(*best_pressure, *current_pressure);
    
    let dists = dist_table.get(&label).unwrap();
    let valves_clone = valves_on.clone();
    network.iter().filter(|(lbl, adj)| adj.pressure > 0 && !valves_clone.contains(lbl)).for_each(|(lbl, _)| {
        let dist = dists.get(lbl).unwrap();
        visit(network, dist_table, lbl.clone(), valves_on, timeout, tick + *dist, current_pressure, best_pressure); 
    });

    *current_pressure -= here.pressure * (timeout - tick);
    valves_on.pop();
}

fn part_two(network: &Network, dist_table: &DistTable) -> u32 {
    let mut best_pressure = 0u32;
    let mut current_pressure = 0u32;
    let mut valves_on = vec!["AA".to_string()];

    let queue = (("AA".to_string(), 0, "Me".to_string()), ("AA".to_string(), 0, "Ele".to_string()));

    visit_queue(network, dist_table, queue, &mut valves_on, 26, &mut current_pressure, &mut best_pressure);
    best_pressure
}

fn visit_queue(network: &Network, dist_table: &DistTable, queue: ((String, u32, String), (String, u32, String)), valves_on: &mut Vec<String>, timeout: u32, current_pressure: &mut u32, best_pressure: &mut u32) {
    if queue.0.1 >= timeout && queue.1.1 >= timeout {
        return;
    }
    if *current_pressure + network.iter().filter(|(lbl, _)| !valves_on.contains(lbl)).map(|(_, node)| node.pressure * (timeout - queue.0.1)).sum::<u32>() < *best_pressure {
        return;
    }
    let (move_now, move_later) = queue;
    let (label, tick, who) = move_now;
    let here = network.get(&label).unwrap();
    valves_on.push(label.clone());
    *current_pressure += here.pressure * (timeout - tick);
    if current_pressure > best_pressure { println!("...{}", current_pressure); }
    *best_pressure = max(*best_pressure, *current_pressure);
    
    let dists = dist_table.get(&label).unwrap();
    let valves_clone = valves_on.clone();
    network.iter().filter(|(lbl, adj)| adj.pressure > 0 && !valves_clone.contains(lbl) && **lbl != move_later.0).for_each(|(lbl, _)| {
        let dist = dists.get(lbl).unwrap();
        let nows_next_tick = tick + dist;
        if nows_next_tick <= timeout {
            let nows_next = (lbl.clone(), nows_next_tick, who.clone());
            let next_queue = if nows_next_tick < move_later.1 {            
                (nows_next, move_later.clone())
            } else {
                (move_later.clone(), nows_next)
            };
            visit_queue(network, dist_table, next_queue, valves_on, timeout, current_pressure, best_pressure); 
        }
    });
    let next_queue = (move_later.clone(), ("XXX".to_string(), timeout + 10, who));
    visit_queue(network, dist_table, next_queue, valves_on, timeout, current_pressure, best_pressure); 
   
    *current_pressure -= here.pressure * (timeout - tick);
    valves_on.pop();
}

fn main() {
    let data = load_data();
    let (network, dist_table) = parse_data(&data);

    let pressure = part_one(&network, &dist_table);
    println!("Part one: {}", pressure);
    let pressure = part_two(&network, &dist_table);
    println!("Part two: {}", pressure);
}
