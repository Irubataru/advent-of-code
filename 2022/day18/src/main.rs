use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::File;
use std::hash::Hash;
use std::io::{prelude::*, BufReader};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    fn add(lhs: &Self, rhs: &Self) -> Self {
        Point {
            x: lhs.x + rhs.x,
            y: lhs.y + rhs.y,
            z: lhs.z + rhs.z,
        }
    }

    fn from_string(s: &str) -> Self {
        let vec = s
            .split(",")
            .map(|p| p.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        Point {
            x: vec[0],
            y: vec[1],
            z: vec[2],
        }
    }
}

struct Bounds {
    lower: Point,
    upper: Point,
}

impl Bounds {
    fn on_or_outside(&self, p: &Point) -> bool {
        p.x <= self.lower.x
            || p.y <= self.lower.y
            || p.z <= self.lower.z
            || p.x >= self.upper.x
            || p.y >= self.upper.y
            || p.z >= self.upper.z
    }
}

fn pop_set<T>(set: &mut HashSet<T>) -> Option<T>
where
    T: Eq + Copy + std::hash::Hash,
{
    match set.iter().next().cloned() {
        Some(item) => {
            set.remove(&item);
            Some(item)
        }
        None => None,
    }
}

fn get_bounds(points: &HashSet<Point>) -> Bounds {
    Bounds {
        lower: Point {
            x: points.iter().map(|p| p.x).min().unwrap(),
            y: points.iter().map(|p| p.y).min().unwrap(),
            z: points.iter().map(|p| p.z).min().unwrap(),
        },
        upper: Point {
            x: points.iter().map(|p| p.x).max().unwrap(),
            y: points.iter().map(|p| p.y).max().unwrap(),
            z: points.iter().map(|p| p.z).max().unwrap(),
        },
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let reader = BufReader::new(File::open(&args[1]).unwrap());

    let nn = vec![
        Point { x: 1, y: 0, z: 0 },
        Point { x: -1, y: 0, z: 0 },
        Point { x: 0, y: 1, z: 0 },
        Point { x: 0, y: -1, z: 0 },
        Point { x: 0, y: 0, z: 1 },
        Point { x: 0, y: 0, z: -1 },
    ];

    let points: HashSet<Point> = reader
        .lines()
        .map(|l| Point::from_string(&l.expect("Failed to read line")))
        .collect();

    // A droplet is any coordinate at the surface of a rock, not occupied by another rock.
    // The value of the map is the number of rocks touch this droplet

    let mut droplets: HashMap<Point, i32> = HashMap::new();
    for point in &points {

        for neighbour in nn.iter().map(|p| Point::add(&point, p)) {
            if points.contains(&neighbour) {
                continue;
            }

            match droplets.get(&neighbour) {
                Some(count) => droplets.insert(neighbour, *count + 1),
                None => droplets.insert(neighbour, 1),
            };
        }
    }

    let part1: i32 = droplets.values().sum();
    println!("Part 1: {}", part1);

    let bounds = get_bounds(&points);

    let mut part2 = 0;
    for (droplet, count) in &droplets {
        let mut inspecting: HashSet<Point> = HashSet::new();
        let mut checked: HashSet<Point> = HashSet::new();

        // Expand outwards from the droplet until free (reach the bounds), or all paths blocked
        inspecting.insert(*droplet);
        while let Some(next) = pop_set(&mut inspecting) {
            if !checked.insert(next) {
                continue;
            }

            let mut is_free = false;
            for neighbour in nn.iter().map(|p| Point::add(&next, p)) {
                if checked.contains(&neighbour) || points.contains(&neighbour) {
                    continue;
                }

                if bounds.on_or_outside(&neighbour) {
                    is_free = true;
                    break;
                }

                inspecting.insert(neighbour);
            }

            if is_free {
                part2 += count;
                break;
            }
        }
    }

    println!("Part 2: {}", part2);
}
