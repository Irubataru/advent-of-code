use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Coordinate {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct PolyLine {
    moves: Vec<Coordinate>,
}

impl PolyLine {
    fn to_coordinates(&self) -> Vec<Coordinate> {
        let mut result: Vec<Coordinate> = Vec::new();
        let mut iter = self.moves.iter();

        let mut current = match iter.next() {
            Some(c) => c,
            None => return result,
        };

        result.push(current.clone());
        while let Some(next) = iter.next() {
            let delta_x = next.x - current.x;
            for i in 0..delta_x.abs() {
                result.push(Coordinate {
                    x: current.x + delta_x.signum() * (i + 1),
                    y: current.y,
                });
            }

            let delta_y = next.y - current.y;
            for i in 0..delta_y.abs() {
                result.push(Coordinate {
                    x: current.x,
                    y: current.y + delta_y.signum() * (i + 1),
                });
            }

            current = next;
        }

        return result;
    }
}

fn add_sand(
    solid: &HashSet<Coordinate>,
    source: Coordinate,
    max_y: i32,
    floor_y: i32,
) -> Option<Coordinate> {
    let mut sand_pos = source;

    if solid.contains(&source) {
        return None;
    }

    while sand_pos.y < max_y {
        if sand_pos.y + 1 == floor_y {
            return Some(Coordinate {
                x: sand_pos.x,
                y: sand_pos.y,
            });
        }

        sand_pos.y += 1;
        if !solid.contains(&sand_pos) {
            continue;
        }

        sand_pos.x -= 1;
        if !solid.contains(&sand_pos) {
            continue;
        }

        sand_pos.x += 2;
        if !solid.contains(&sand_pos) {
            continue;
        }

        return Some(Coordinate {
            x: sand_pos.x - 1,
            y: sand_pos.y - 1,
        });
    }

    return None;
}

fn simulate(rocks: &HashSet<Coordinate>, max_offset: i32, floor_offset: i32) -> i32 {
    let max_y = rocks.iter().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y;

    let mut solid = rocks.clone();

    let mut count = 0;
    while let Some(new_sand_pos) = add_sand(
        &solid,
        Coordinate { x: 500, y: 0 },
        max_y + max_offset,
        max_y + floor_offset,
    ) {
        solid.insert(new_sand_pos);
        count += 1;
    }

    count
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let reader = BufReader::new(File::open(&args[1]).unwrap());

    let mut polylines: Vec<PolyLine> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        polylines.push(PolyLine {
            moves: line
                .split(" -> ")
                .map(|s| {
                    let mut parts = s.split(",");
                    Coordinate {
                        x: parts.next().unwrap().parse::<i32>().unwrap(),
                        y: parts.next().unwrap().parse::<i32>().unwrap(),
                    }
                })
                .collect(),
        });
    }

    let rocks: HashSet<Coordinate> = polylines.iter().flat_map(|l| l.to_coordinates()).collect();

    let part1 = simulate(&rocks, 0, 1);
    println!("Part 1: {}", part1);

    let part2 = simulate(&rocks, 3, 2);
    println!("Part 2: {}", part2);
}
