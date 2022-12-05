use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Move {
    from: usize,
    to: usize,
    count: usize,
}

fn read_init() -> Vec<Vec<char>> {
    let args: Vec<String> = env::args().collect();
    let reader = BufReader::new(File::open(&args[1]).unwrap());

    let mut result: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line.");
        result.push(line.split(",").map(|x| x.chars().next().unwrap()).collect());
    }

    return result;
}

fn read_move(line: String) -> Move {
    let parts: Vec<&str> = line.split(" ").collect();
    return Move {
        from: parts[3]
            .parse::<usize>()
            .expect("Expect format \"move _ from _ to _\"")
            - 1,
        to: parts[5]
            .parse::<usize>()
            .expect("Expect format \"move _ from _ to _\"")
            - 1,
        count: parts[1]
            .parse::<usize>()
            .expect("Expect format \"move _ from _ to _\""),
    };
}

fn print_tops(state: &Vec<Vec<char>>) {
    for top in state {
        print!(
            "{}",
            match top.last() {
                Some(x) => x,
                None => &'_',
            }
        )
    }
    print!("\n");
}

fn drive_crane(reverse: bool) {
    let mut stacks = read_init();

    let args: Vec<String> = env::args().collect();
    let reader = BufReader::new(File::open(&args[2]).unwrap());

    for line in reader.lines() {
        let line = line.expect("Failed to read line.");
        let stack_move = read_move(line);

        let len = stacks[stack_move.from].len();
        let mut crane: Vec<char> = stacks[stack_move.from]
            .iter()
            .skip(len - stack_move.count)
            .take(stack_move.count)
            .cloned()
            .collect();

        if reverse {
            crane.reverse();
        }

        for i in 0..stack_move.count {
            stacks[stack_move.from].pop();
            stacks[stack_move.to].push(crane[i]);
        }
    }

    print_tops(&stacks);
}

fn main() {
    print!("Part 1: ");
    drive_crane(true);
    print!("Part 2: ");
    drive_crane(false);
}
