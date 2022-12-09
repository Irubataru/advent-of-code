use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn next_head(head: (i32, i32), dir: &str) -> (i32, i32) {
    match dir {
        "U" => (head.0, head.1 + 1),
        "D" => (head.0, head.1 - 1),
        "R" => (head.0 + 1, head.1),
        "L" => (head.0 - 1, head.1),
        _ => panic!("Unknown direction {}", dir),
    }
}

fn next_tail(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    let x = head.0 - tail.0;
    let y = head.1 - tail.1;
    
    if x.abs() < 2 && y.abs() < 2 {
        return (tail.0, tail.1);
    }

    return (tail.0 + x.signum(), tail.1 + y.signum());
}

fn simulate(len: usize) -> usize {
    let args: Vec<String> = env::args().collect();
    let reader = BufReader::new(File::open(&args[1]).unwrap());

    let mut rope = vec![(0, 0); len];

    let mut visits: HashSet<(i32, i32)> = HashSet::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        let mut parts = line.split(" ");
        let dir = parts.next().unwrap();
        let count = parts.next().unwrap().parse::<u32>().unwrap();

        for _ in 0..count {
            rope[0] = next_head(rope[0], dir);

            for i in 1..len {
                rope[i] = next_tail(rope[i - 1], rope[i]);
            }

            visits.insert(rope[len - 1]);
        }
    }

    visits.len()
}

fn main() {
    println!("Part 1: {}", simulate(2));
    println!("Part 2: {}", simulate(10));
}
