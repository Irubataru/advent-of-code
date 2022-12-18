use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

static CACHE: usize = 10000;
static BUFFER: usize = 200;
static TOP: usize = 50;

fn get_shapes() -> Vec<Vec<Coordinate>> {
    vec![
        vec![
            Coordinate { x: 2, y: 0 },
            Coordinate { x: 3, y: 0 },
            Coordinate { x: 4, y: 0 },
            Coordinate { x: 5, y: 0 },
        ],
        vec![
            Coordinate { x: 3, y: 2 },
            Coordinate { x: 2, y: 1 },
            Coordinate { x: 3, y: 1 },
            Coordinate { x: 4, y: 1 },
            Coordinate { x: 3, y: 0 },
        ],
        vec![
            Coordinate { x: 4, y: 2 },
            Coordinate { x: 4, y: 1 },
            Coordinate { x: 2, y: 0 },
            Coordinate { x: 3, y: 0 },
            Coordinate { x: 4, y: 0 },
        ],
        vec![
            Coordinate { x: 2, y: 3 },
            Coordinate { x: 2, y: 2 },
            Coordinate { x: 2, y: 1 },
            Coordinate { x: 2, y: 0 },
        ],
        vec![
            Coordinate { x: 2, y: 1 },
            Coordinate { x: 3, y: 1 },
            Coordinate { x: 2, y: 0 },
            Coordinate { x: 3, y: 0 },
        ],
    ]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

fn shift_wind(shape: &mut Vec<Coordinate>, to_right: bool, solids: &Vec<Vec<bool>>) {
    for c in &*shape {
        if to_right && (c.x == 6 || solids[c.y][c.x + 1]) {
            return;
        }

        if !to_right && (c.x == 0 || solids[c.y][c.x - 1]) {
            return;
        }
    }

    for mut pos in shape {
        pos.x = if to_right { pos.x + 1 } else { pos.x - 1 };
    }
}

fn shift_height(shape: &mut Vec<Coordinate>, y: usize) {
    for mut pos in shape {
        pos.y = pos.y + y;
    }
}

fn can_drop(shape: &Vec<Coordinate>, solids: &Vec<Vec<bool>>) -> bool {
    for c in shape {
        if solids[c.y - 1][c.x] {
            return false;
        }
    }
    true
}

fn drop(shape: &mut Vec<Coordinate>) {
    for mut pos in shape {
        pos.y = pos.y - 1;
    }
}

fn place(shape: &Vec<Coordinate>, solids: &mut Vec<Vec<bool>>, ceil: usize) -> usize {
    let mut ceil = ceil;

    for c in shape {
        solids[c.y][c.x] = true;
        ceil = std::cmp::max(ceil, c.y);
    }

    ceil
}

fn update_buffer(solids: &mut Vec<Vec<bool>>, floor: usize, ceil: usize) -> (usize, usize) {
    let floor = floor + BUFFER;
    let ceil = ceil - BUFFER;

    for i in 0..CACHE - BUFFER {
        for j in 0..7 {
            solids[i][j] = solids[i + BUFFER][j];
        }
    }

    for i in CACHE - BUFFER..CACHE {
        for j in 0..7 {
            solids[i][j] = false;
        }
    }

    (floor, ceil)
}

fn clone_top(solids: &Vec<Vec<bool>>, ceil: usize) -> Vec<Vec<bool>> {
    let mut cave_top = vec![vec![false; 7]; TOP];

    for i in 0..TOP {
        let y = ceil - (TOP - 1) + i;
        for j in 0..7 {
            cave_top[i][j] = solids[y][j];
        }
    }

    cave_top
}

fn simulate(target: u64, moves: &Vec<bool>) -> u64 {
    let shapes = get_shapes();

    let mut floor: usize = 0;
    let mut ceil: usize = 0;
    let mut solids = vec![vec![false; 7]; CACHE];

    for i in 0..7 {
        solids[0][i] = true;
    }

    // Keep a cache of states that we can compare with to find cycles
    let mut checkpoints: HashMap<(usize, Vec<Vec<bool>>), (u64, u64)> = HashMap::new();

    let mut index = 0;
    let mut shape_index = 0;
    let mut num_rocks: u64 = 0;
    while num_rocks < target {
        let mut shape = shapes[shape_index].clone();

        // Place rock at its start position
        shift_height(&mut shape, ceil + 4);

        loop {
            shift_wind(&mut shape, moves[index], &solids);
            index = (index + 1) % moves.len();

            if can_drop(&shape, &solids) {
                drop(&mut shape);
                continue;
            }

            ceil = place(&shape, &mut solids, ceil);

            // Update the sliding window so we only store CACHE rows
            if ceil + BUFFER > CACHE {
                (floor, ceil) = update_buffer(&mut solids, floor, ceil);
            }

            break;
        }

        num_rocks += 1;
        shape_index = (shape_index + 1) % shapes.len();

        // Loop for a cycle every time we start over withe the rocks
        if shape_index == 0 && ceil > TOP {
            let height = floor + ceil;
            let cave_top = clone_top(&solids, ceil);

            // See if we find a cycle. A cycle occues if the wind index is the same
            // and the TOP of the tower is the same
            if let Some((prev_count, prev_height)) =
                checkpoints.insert((index, cave_top), (num_rocks, height as u64))
            {
                let cycle_length = num_rocks - prev_count;
                let num_cycles = (target - num_rocks) / cycle_length;
                num_rocks += cycle_length * num_cycles;

                floor += (height - prev_height as usize) * num_cycles as usize;

                checkpoints.clear();
            }
        }
    }

    floor as u64 + ceil as u64
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let line = BufReader::new(File::open(&args[1]).unwrap())
        .lines()
        .next()
        .expect("Input should have 1 line")
        .unwrap();

    let moves = line
        .chars()
        .map(|c| match c {
            '>' => true,
            _ => false,
        })
        .collect::<Vec<bool>>();

    println!("Part 1: {}", simulate(2022, &moves));
    println!("Part 2: {}", simulate(1000000000000, &moves));
}
