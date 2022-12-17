use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Coordinate {
    x: i32,
    y: i32
}

impl Coordinate {
    fn from_str(s: &str) -> Self {
        let mut parts = s.split(",");
        Coordinate {
            x: parts.next().unwrap().parse::<i32>().unwrap(),
            y: parts.next().unwrap().parse::<i32>().unwrap(),
        }
    }

    fn dist(&self, other: &Coordinate) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let reader = BufReader::new(File::open(&args[1]).unwrap());

    let mut sensor_beacon_pairs: Vec<(Coordinate, Coordinate, i32)> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Error reading line.");

        let mut parts = line.split(":");

        let sensor = Coordinate::from_str(parts.next().unwrap());
        let beacon = Coordinate::from_str(parts.next().unwrap());

        sensor_beacon_pairs.push((
                sensor,
                beacon,
                sensor.dist(&beacon)
        ));
    }

    let y_line = 2000000;

    let mut not_possible : HashSet<Coordinate> = HashSet::new();

    for (sensor, beacon, d) in &sensor_beacon_pairs {

        let dist = (sensor.y - y_line).abs();

        for i in dist..d + 1 {
            not_possible.insert(Coordinate {
                x: sensor.x + (i - dist),
                y: y_line
            });
            not_possible.insert(Coordinate {
                x: sensor.x - (i - dist),
                y: y_line
            });
        }
    }

    for (_, beacon, _) in &sensor_beacon_pairs {
        not_possible.remove(beacon);
    }

    sensor_beacon_pairs.sort_by(|x, y| x.2.cmp(&y.2));
    sensor_beacon_pairs.reverse();

    println!("Part 1: {}", not_possible.len());

    for i in 3316867..4000001 {
        if (i % 1000) == 0 {
            println!("x: {}", i);
        }

        for j in 0..4000001 {
            let mut all_okay = true;

            for (sensor, beacon, d) in &sensor_beacon_pairs {
                if (sensor.x - i).abs() + (sensor.y - j).abs() <= *d {
                    all_okay = false;
                    break;
                }
            }

            if all_okay {
                println!("Part 2: ({}, {}) => {}", i, j, (i as u128) * 4000000 + j as u128);
                return;
            }
        }
    }
}
