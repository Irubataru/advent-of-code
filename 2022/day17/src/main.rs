use std::collections::{HashSet, HashMap};
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

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

fn shift_height(shape: &mut Vec<Coordinate>, y: i32) {
    for mut pos in shape {
        pos.y = (pos.y as i32 + y) as usize;
    }
}

fn print_tower(solids: &Vec<Vec<bool>>, shape: &Vec<Coordinate>) {
    for i in 0..solids.len() {
        let i = solids.len() - 1 - i;

        for j in 0..solids[i].len() {
            if shape.contains(&Coordinate { x: j, y: i }) {
                print!("@");
                continue;
            }

            print!("{}", if solids[i][j] { "#" } else { "." });
        }

        println!("");
    }

    println!("-------\n");
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

    let shapes = vec![
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
    ];

    let mut index = 0;
    let mut shape_index = 0;

    // let mut solids: HashSet<Coordinate> = HashSet::new();
    //

    let cache = 10000;
    let buffer = 200;

    let mut y_floor: u128 = 0;
    let mut y_ceil: usize = 0;
    let mut solids = vec![vec![false; 7]; cache];

    for i in 0..7 {
        solids[0][i] = true;
    }

    let mut num_rocks: u128 = 0;

    let mut checkpoints : HashMap<(usize, Vec<Vec<bool>>), (u128, u128)> = HashMap::new();

    let target = 1000000000000;

    while num_rocks < target {
        let mut shape = shapes[shape_index].clone();

        if num_rocks % 100000 == 0 {
            println!("{} {}", num_rocks, y_floor + y_ceil as u128);
        }

        shift_height(&mut shape, y_ceil as i32 + 4);
        // print_tower(&solids, &shape);

        loop {
            shift_wind(&mut shape, moves[index], &solids);
            index = (index + 1) % moves.len();

            let mut will_intersect = false;
            for c in &shape {
                if solids[c.y - 1][c.x] {
                    will_intersect = true;
                    break;
                }
            }

            if will_intersect {
                for c in &shape {
                    solids[c.y][c.x] = true;
                    y_ceil = std::cmp::max(y_ceil, c.y);
                }

                if y_ceil + buffer > cache {

                    y_floor += buffer as u128;
                    y_ceil -= buffer;

                    for i in 0..cache-buffer {
                        for j in 0..7 {
                            solids[i][j] = solids[i+buffer][j];

                        }
                    }

                    for i in cache-buffer..cache {
                        for j in 0..7 {
                            solids[i][j] = false;
                        }
                    }
                }


                break;
            }

            // print_tower(&solids, &shape);
            shift_height(&mut shape, -1);
        }

        num_rocks += 1;
        shape_index = (shape_index + 1) % shapes.len();

        if shape_index == 0 && y_ceil > 1000 {

            let mut cave_top = vec![vec![false; 7]; 1000];

            for i in 0..1000 {
                let y = y_ceil - 999 + i;
                for j in 0..7 {
                    cave_top[i][j] = solids[y][j];
                }
            }

            let height = y_floor as u128 + y_ceil as u128;

            if let Some((prev_count, prev_height)) = checkpoints.insert((index, cave_top), (num_rocks, height)) {

                let cycle_length = num_rocks - prev_count;
                let num_cycles = (target - num_rocks) / cycle_length;
                num_rocks += cycle_length * num_cycles;

                y_floor += (height - prev_height) * num_cycles;
            }


        }
    }

    println!("Part 1: {}", y_floor + y_ceil as u128);
    // print_tower(&solids, &Vec::new());
}
