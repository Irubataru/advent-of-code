use core::hash::Hash;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::hash::Hasher;
use std::io::{prelude::*, BufReader};

type Id = u16;
type PathMap = HashMap<Id, i32>;
type DistanceMap = HashMap<Id, PathMap>;
type Network = HashMap<Id, Valve>;

struct NameIdMap {
    map: HashMap<String, Id>,
}

impl NameIdMap {
    fn new() -> Self {
        NameIdMap {
            map: HashMap::new(),
        }
    }

    fn get(&mut self, name: &str) -> Id {
        let name = name.to_string();

        if !self.map.contains_key(&name) {
            self.map.insert(name.clone(), self.map.len() as Id);
        }

        self.map[&name]
    }
}

#[derive(Debug, Clone)]
struct Valve {
    flow_rate: i32,
    tunnels: Vec<Id>,
}

impl Valve {
    fn from_str(text: &str, name_id_map: &mut NameIdMap) -> (Id, Self) {
        let mut parts = text.split(";");
        let mut first = parts.next().unwrap().split(",");
        let name = first.next().unwrap().to_string();
        let flow_rate = first.next().unwrap().parse::<i32>().unwrap();

        let tunnels = parts
            .next()
            .unwrap()
            .split(",")
            .map(|s| name_id_map.get(s))
            .collect::<Vec<_>>();

        (name_id_map.get(&name), Self { flow_rate, tunnels })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Agent {
    pos: Id,
    time: i32,
}

#[derive(Clone, Debug)]
struct Sequence {
    score: i32,
    open_valves: Vec<Id>,
    agent: Agent,
}

impl Sequence {
    fn new(start: Id) -> Self {
        Self {
            score: 0,
            open_valves: Vec::new(),
            agent: Agent {
                pos: start,
                time: 1,
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct SequenceMulti {
    score: i32,
    open_valves: Vec<Id>,
    agent1: Agent,
    agent2: Agent,
}

impl Hash for SequenceMulti {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.score.hash(state);

        let time = self.agent1.time + self.agent2.time;
        time.hash(state);

        let pos = self.agent1.pos + self.agent2.pos;
        pos.hash(state);

        self.open_valves.hash(state);
    }
}

impl SequenceMulti {
    fn new(start: Id) -> Self {
        Self {
            score: 0,
            open_valves: Vec::new(),
            agent1: Agent {
                pos: start,
                time: 5,
            },
            agent2: Agent {
                pos: start,
                time: 5,
            },
        }
    }

    fn get_hash(&self) -> u64 {
        let mut h = DefaultHasher::new();
        self.hash(&mut h);
        h.finish()
    }
}

fn shortest_path(from: Id, valves: &Network) -> PathMap {
    let mut costs: PathMap = valves.keys().map(|n| (*n, i32::MAX)).collect();
    let mut open_nodes: HashSet<Id> = HashSet::new();

    open_nodes.insert(from);

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

fn create_distance_map(valves: &Network) -> DistanceMap {
    valves
        .iter()
        .map(|(k, _)| {
            let mut paths = shortest_path(*k, &valves);

            // We want to remove keys where we can't get to the valve and where
            // the target has 0 flow to save resources
            let invalid = paths
                .iter()
                .filter(|(k, &v)| v >= 30 || valves[k].flow_rate == 0)
                .map(|(&k, _)| k)
                .collect::<Vec<_>>();

            for key in invalid {
                paths.remove(&key);
            }

            (k.clone(), paths)
        })
        .collect()
}

fn part1(valves: &Network, start: Id) {
    let mut best_score = 0;
    let mut sequences: Vec<Sequence> = Vec::new();

    let distance_map = create_distance_map(valves);

    sequences.push(Sequence::new(start));

    while let Some(seq) = sequences.pop() {
        let paths = &distance_map[&seq.agent.pos];

        for (next, dist) in paths {
            let valve = &valves[next];

            if seq.open_valves.contains(&next) {
                continue;
            }

            if seq.agent.time + dist > 29 {
                continue;
            }

            let mut seq = seq.clone();
            seq.agent.time += dist + 1;
            seq.open_valves.push(next.clone());
            seq.open_valves.sort();
            seq.agent.pos = next.clone();
            seq.score += valve.flow_rate * (30 - seq.agent.time + 1);

            if seq.score > best_score {
                best_score = seq.score;
            }

            sequences.push(seq);
        }
    }

    println!("Part 1: {}", best_score);
}

fn find_next_moves(
    agent_id: i32,
    current: &SequenceMulti,
    valves: &Network,
    distance_map: &DistanceMap,
    cache: &HashSet<u64>,
) -> Vec<SequenceMulti> {
    let mut result: Vec<SequenceMulti> = Vec::new();

    let agent = if agent_id == 0 {
        current.agent1
    } else {
        current.agent2
    };

    let paths = &distance_map[&agent.pos];

    for (next, dist) in paths {
        let valve = &valves[next];

        if current.open_valves.contains(next) {
            continue;
        }

        if agent.time + dist > 29 {
            continue;
        }

        let mut next_seq = current.clone();

        let agent = if agent_id == 0 {
            &mut next_seq.agent1
        } else {
            &mut next_seq.agent2
        };

        agent.time += dist + 1;
        agent.pos = *next;

        next_seq.open_valves.push(*next);
        next_seq.open_valves.sort();
        next_seq.score += valve.flow_rate * (30 - agent.time + 1);

        if cache.contains(&next_seq.get_hash()) {
            continue;
        }

        result.push(next_seq);
    }

    result
}

fn part2(valves: &Network, start: Id) {
    let mut best_score = 0;
    let mut sequences: Vec<SequenceMulti> = Vec::new();

    let distance_map = create_distance_map(valves);

    // Use a hash of previous branches to break the symmetry of swapping agent1 <-> agent2
    let mut previously_handled: HashSet<u64> = HashSet::new();

    sequences.push(SequenceMulti::new(start));

    while let Some(seq) = sequences.pop() {
        previously_handled.insert(seq.get_hash());
        best_score = std::cmp::max(best_score, seq.score);

        for next_move in find_next_moves(0, &seq, &valves, &distance_map, &previously_handled) {
            sequences.push(next_move);
        }

        if seq.agent1.pos == seq.agent2.pos {
            continue;
        }

        for next_move in find_next_moves(1, &seq, &valves, &distance_map, &previously_handled) {
            sequences.push(next_move);
        }
    }

    println!("Part 2: {}", best_score);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let reader = BufReader::new(File::open(&args[1]).unwrap());

    let mut name_id_map: NameIdMap = NameIdMap::new();

    let valves = reader
        .lines()
        .map(|l| Valve::from_str(&l.unwrap(), &mut name_id_map))
        .collect::<HashMap<Id, Valve>>();

    let start = name_id_map.get("AA");

    part1(&valves, start);
    part2(&valves, start);
}
