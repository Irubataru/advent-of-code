use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Clone, Copy, Debug)]
struct Coordinate {
    x: usize,
    y: usize,
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
            let delta_x = next.x as i32 - current.x as i32;
            for i in 0..delta_x.abs() {
                result.push(Coordinate {
                    x: (current.x as i32 + delta_x.signum() * (i + 1)) as usize,
                    y: current.y,
                });
            }

            let delta_y = next.y as i32 - current.y as i32;
            for i in 0..delta_y.abs() {
                result.push(Coordinate {
                    x: current.x,
                    y: (current.y as i32 + delta_y.signum() * (i + 1)) as usize,
                });
            }

            current = next;
        }

        return result;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let reader = BufReader::new(File::open(&args[1]).unwrap());

    let mut polylines: Vec<PolyLine> = Vec::new();

    let mut min_x = usize::MAX;
    let mut max_x = 0;
    let mut max_y = 0;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        polylines.push(PolyLine {
            moves: line
                .split(" -> ")
                .map(|s| {
                    let mut parts = s.split(",");
                    let x = parts.next().unwrap().parse::<usize>().unwrap();
                    let y = parts.next().unwrap().parse::<usize>().unwrap();

                    min_x = std::cmp::min(min_x, x);
                    max_x = std::cmp::max(max_x, x);
                    max_y = std::cmp::max(max_y, y);

                    Coordinate { x, y }
                })
                .collect(),
        });
    }

    let mut cave: Vec<Vec<bool>> = vec![vec![false; max_y + 1]; max_x + 1 - min_x];

    for line in &polylines {
        for c in line.to_coordinates() {
            println!("({}, {})", c.x - min_x, c.y);
            cave[c.x - min_x][c.y] = true;
        }
    }
}
