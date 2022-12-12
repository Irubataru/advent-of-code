use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Node {
    x: i32,
    y: i32,
}

fn read_heights() -> (HashMap<Node, i32>, Node, Node) {
    let args: Vec<String> = env::args().collect();
    let reader = BufReader::new(File::open(&args[1]).unwrap());

    let mut start_node = Node { x: 0, y: 0 };
    let mut end_node = Node { x: 0, y: 0 };

    let mut nodes: HashMap<Node, i32> = HashMap::new();
    let mut row = 0;
    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        let mut column = 0;

        for c in line.chars() {
            let node = Node { x: row, y: column };

            let height = match c {
                'S' => {
                    start_node = node;
                    0
                }
                'E' => {
                    end_node = node;
                    'z' as i32 - 'a' as i32
                }
                _ => c as i32 - 'a' as i32,
            };

            nodes.insert(node, height);
            column += 1;
        }

        row += 1;
    }

    return (nodes, start_node, end_node);
}

fn is_better_move(
    node: &Node,
    neighbour: &Node,
    heights: &HashMap<Node, i32>,
    costs: &HashMap<Node, i32>,
) -> bool {
    return costs[&neighbour] > costs[&node] && heights[&neighbour] - heights[&node] <= 1;
}

fn shortest_path(heights: HashMap<Node, i32>, open_nodes: HashSet<Node>, end: Node) -> i32 {
    let mut costs: HashMap<Node, i32> = heights.keys().map(|n| (*n, i32::MAX)).collect();
    let mut open_nodes = open_nodes.clone();

    for node in &open_nodes {
        costs.insert(*node, 0);
    }

    let nearest_neighbours: Vec<(i32, i32)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
    while !open_nodes.is_empty() {
        let node = open_nodes
            .iter()
            .min_by(|n, m| costs[n].cmp(&costs[m]))
            .unwrap()
            .clone();
        open_nodes.remove(&node);

        if node.eq(&end) {
            return costs[&node];
        }

        for nn in &nearest_neighbours {
            let neighbour = Node {
                x: node.x + nn.0,
                y: node.y + nn.1,
            };
            if !heights.contains_key(&neighbour) {
                continue;
            }

            if is_better_move(&node, &neighbour, &heights, &costs) {
                open_nodes.insert(neighbour);
                costs.insert(neighbour, costs[&node] + 1);
            }
        }
    }

    return i32::MAX;
}

fn main() {
    let (heights, start, end) = read_heights();

    let mut open_nodes: HashSet<Node> = HashSet::new();
    open_nodes.insert(start);

    let part1 = shortest_path(heights.clone(), open_nodes, end);
    println!("Part 1: {}", part1);

    let open_nodes: HashSet<Node> = heights
        .iter()
        .filter(|(_, v)| **v == 0)
        .map(|(k, _)| *k)
        .collect();

    let part2 = shortest_path(heights.clone(), open_nodes, end);
    println!("Part 2: {}", part2);
}
