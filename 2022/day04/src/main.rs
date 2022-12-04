use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn parse_assignment(data: &str) -> (i32, i32) {
    let parts: Vec<&str> = data.split("-").collect();
    return (
        parts[0].parse::<i32>().expect("Expected an integer on lhs"),
        parts[1].parse::<i32>().expect("Expected an integer on rhs"),
    );
}

fn parse_line(line: String) -> ((i32, i32), (i32, i32)) {
    let parts: Vec<&str> = line.split(",").collect();
    return (parse_assignment(parts[0]), parse_assignment(parts[1]));
}

fn is_fully_contained(x: (i32, i32), y: (i32, i32)) -> bool {
    return (x.0 <= y.0 && y.1 <= x.1) || (y.0 <= x.0 && x.1 <= y.1);
}

fn overlaps(x: (i32, i32), y: (i32, i32)) -> bool {
    return (y.0 <= x.0 && y.1 >= x.0) || (y.0 >= x.0 && y.0 <= x.1);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let reader = BufReader::new(File::open(&args[1]).unwrap());

    let mut num_fully_contained = 0;
    let mut num_overlapping = 0;
    for line in reader.lines() {
        let (assignment1, assignment2) = parse_line(line.expect("Failed to read line"));
        num_fully_contained =
            num_fully_contained + is_fully_contained(assignment1, assignment2) as i32;
        num_overlapping = num_overlapping + overlaps(assignment1, assignment2) as i32;
    }

    println!("Part 1: {}", num_fully_contained);
    println!("Part 2: {}", num_overlapping);
}
