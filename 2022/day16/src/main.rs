use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::hash::Hasher;
use std::io::{prelude::*, BufReader};
use core::hash::Hash;

#[derive(Debug, Clone)]
struct Valve {
    flow_rate: i32,
    tunnels: Vec<String>,
}

impl Valve {
    fn from(text: &str) -> (String, Self) {
        let mut parts = text.split(";");
        let mut first = parts.next().unwrap().split(",");
        let name = first.next().unwrap().to_string();
        let flow_rate = first.next().unwrap().parse::<i32>().unwrap();

        let tunnels = parts
            .next()
            .unwrap()
            .split(",")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        (name, Self { flow_rate, tunnels })
    }
}

#[derive(Clone, Debug)]
struct Sequence {
    score: i32,
    time: i32,
    open_valves: Vec<String>,
    pos: String,
}

impl Sequence {
    fn new(start: &str) -> Self {
        Self {
            score: 0,
            time: 1,
            open_valves: Vec::new(),
            pos: start.to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct SequencePair {
    score: i32,
    time_you: i32,
    time_ele: i32,
    open_valves: Vec<String>,
    pos_you: String,
    pos_ele: String
}

impl Hash for SequencePair {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.score.hash(state);
        
        let time = self.time_you + self.time_ele;
        time.hash(state);

        let mut pos = format!("{}{}", self.pos_you, self.pos_ele).chars().collect::<Vec<char>>();
        pos.sort();
        pos.hash(state);

        self.open_valves.hash(state);
    }
}

impl SequencePair {
    fn new(start: &str) -> Self {
        Self {
            score: 0,
            time_you: 1,
            time_ele: 1,
            open_valves: Vec::new(),
            pos_you: start.to_string(),
            pos_ele: start.to_string()
        }
    }

    fn get_hash(&self) -> u64 {
        let mut h = DefaultHasher::new();
        self.hash(&mut h);
        h.finish()
    }
}

fn shortest_path(from: &String, valves: &HashMap<String, Valve>) -> HashMap<String, i32> {
    let mut costs: HashMap<String, i32> = valves.keys().map(|n| (n.clone(), i32::MAX)).collect();
    let mut open_nodes: HashSet<String> = HashSet::new();

    open_nodes.insert(from.clone());

    for node in &open_nodes {
        costs.insert(node.clone(), 0);
    }

    while !open_nodes.is_empty() {
        let node = open_nodes
            .iter()
            .min_by(|&n, &m| costs[n].cmp(&costs[m]))
            .unwrap()
            .clone();
        open_nodes.remove(&node);

        let cost = costs[&node];

        for nn in &valves[&node].tunnels {
            if cost + 1 < costs[nn] {
                open_nodes.insert(nn.clone());
                costs.insert(nn.clone(), cost + 1);
            }
        }
    }

    return costs;
}

fn part1(valves: &HashMap<String, Valve>) {
    let mut best_score = 0;
    let mut sequences: Vec<Sequence> = Vec::new();

    let mut shortest_path_cache: HashMap<String, HashMap<String, i32>> = HashMap::new();

    sequences.push(Sequence::new("AA"));

    while let Some(seq) = sequences.pop() {
        if !shortest_path_cache.contains_key(&seq.pos) {
            shortest_path_cache.insert(seq.pos.clone(), shortest_path(&seq.pos, &valves));
        }

        let paths = &shortest_path_cache[&seq.pos];

        for (next, dist) in paths {
            let valve = &valves[next];

            if valve.flow_rate == 0 {
                continue;
            }

            if seq.open_valves.contains(&next) {
                continue;
            }

            if seq.time + dist > 29 {
                continue;
            }

            let mut next_seq = seq.clone();
            next_seq.time += dist + 1;
            next_seq.open_valves.push(next.clone());
            next_seq.open_valves.sort();
            next_seq.pos = next.clone();
            next_seq.score += valve.flow_rate * (30 - next_seq.time + 1);

            if next_seq.score > best_score {
                best_score = next_seq.score;
            }

            sequences.push(next_seq);
        }
    }

    println!("Part 1: {}", best_score);
}

fn part2(valves: &HashMap<String, Valve>) {
    let mut best_score = 0;
    let mut sequences: Vec<SequencePair> = Vec::new();

    let mut shortest_path_cache: HashMap<String, HashMap<String, i32>> = HashMap::new();

    let mut previous : HashSet<u64> = HashSet::new();

    sequences.push(SequencePair::new("AA"));

    while let Some(seq) = sequences.pop() {

        previous.insert(seq.get_hash());

        if !shortest_path_cache.contains_key(&seq.pos_you) {
            shortest_path_cache.insert(seq.pos_you.clone(), shortest_path(&seq.pos_you, &valves));
        }

        let paths = &shortest_path_cache[&seq.pos_you];

        for (next, dist) in paths {
            let valve = &valves[next];

            if valve.flow_rate == 0 {
                continue;
            }

            if seq.open_valves.contains(next) {
                continue;
            }

            if seq.time_you + dist > 26 {
                continue;
            }

            let mut next_seq = seq.clone();
            next_seq.time_you += dist + 1;
            next_seq.open_valves.push(next.clone());
            next_seq.pos_you = next.clone();
            next_seq.score += valve.flow_rate * (26 - next_seq.time_you + 1);

            if previous.contains(&next_seq.get_hash()) {
                println!("Removed symmetry ({})", previous.len());
                continue;
            }

            if next_seq.score > best_score {
                best_score = next_seq.score;
            }

            sequences.push(next_seq);
        }

        if seq.pos_you == seq.pos_ele {
            continue;
        }

        if !shortest_path_cache.contains_key(&seq.pos_ele) {
            shortest_path_cache.insert(seq.pos_ele.clone(), shortest_path(&seq.pos_ele, &valves));
        }

        let paths = &shortest_path_cache[&seq.pos_ele];

        for (next, dist) in paths {
            let valve = &valves[next];

            if valve.flow_rate == 0 {
                continue;
            }

            if seq.open_valves.contains(next) {
                continue;
            }

            if seq.time_ele + dist > 26 {
                continue;
            }

            let mut next_seq = seq.clone();
            next_seq.time_ele += dist + 1;
            next_seq.open_valves.push(next.clone());
            next_seq.open_valves.sort();
            next_seq.pos_ele = next.clone();
            next_seq.score += valve.flow_rate * (26 - next_seq.time_ele + 1);

            if previous.contains(&next_seq.get_hash()) {
                println!("Removed symmetry ({})", previous.len());
                continue;
            }

            if next_seq.score > best_score {
                best_score = next_seq.score;
            }

            sequences.push(next_seq);
        }
    }

    println!("Part 2: {}", best_score);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let reader = BufReader::new(File::open(&args[1]).unwrap());

    let valves = reader
        .lines()
        .map(|l| Valve::from(&l.unwrap()))
        .collect::<HashMap<String, Valve>>();

    part1(&valves);
    part2(&valves);
}

