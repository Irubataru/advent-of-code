use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Coordinate {
    x: i32,
    y: i32,
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

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
struct Sensor {
    pos: Coordinate,
    beacon_pos: Coordinate,
    distance_to_beacon: i32,
}

impl Sensor {
    fn new(sensor: &Coordinate, beacon: &Coordinate) -> Self {
        Sensor {
            pos: *sensor,
            beacon_pos: *beacon,
            distance_to_beacon: sensor.dist(&beacon),
        }
    }

    fn from_str(s: &str) -> Self {
        let mut parts = s.split(":");

        let sensor = Coordinate::from_str(parts.next().unwrap());
        let beacon = Coordinate::from_str(parts.next().unwrap());

        Self::new(&sensor, &beacon)
    }
}

fn part1(sensors: &Vec<Sensor>) {
    let y_line = 2000000;
    let mut not_possible: HashSet<Coordinate> = HashSet::new();
    for sensor in sensors {
        let dist = (sensor.pos.y - y_line).abs();

        for i in dist..sensor.distance_to_beacon + 1 {
            not_possible.insert(Coordinate {
                x: sensor.pos.x + (i - dist),
                y: y_line,
            });
            not_possible.insert(Coordinate {
                x: sensor.pos.x - (i - dist),
                y: y_line,
            });
        }
    }

    for sensor in sensors {
        not_possible.remove(&sensor.beacon_pos);
    }

    println!("Part 1: {}", not_possible.len());
}

fn part2(sensors: &Vec<Sensor>) {
    // For part 2 we only scan along the edge of every beacon's range. If the result is unique,
    // then the answer has to lie on one of these edges.

    let bounds = 4000000;
    for sensor in sensors {
        for i in 0..sensor.distance_to_beacon + 2 {
            let x = sensor.pos.x + sensor.distance_to_beacon + 1 - i;
            let y = sensor.pos.y + i;

            if x < 0 || x > bounds {
                continue;
            }

            if y < 0 || y > bounds {
                continue;
            }

            if sensors
                .iter()
                .any(|s| (s.pos.x - x).abs() + (s.pos.y - y).abs() <= s.distance_to_beacon)
            {
                continue;
            }

            println!("Part 2: {}", (x as u128) * (bounds as u128) + y as u128);
            return;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let reader = BufReader::new(File::open(&args[1]).unwrap());

    let sensors = reader
        .lines()
        .map(|l| Sensor::from_str(&l.expect("Failed to read line")))
        .collect::<Vec<_>>();

    part1(&sensors);
    part2(&sensors);
}
