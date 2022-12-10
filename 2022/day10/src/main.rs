use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn print_display(pixels: &Vec<bool>) {
    for i in 0..pixels.len() {
        print!(
            "{}{}",
            if i % 40 == 0 { "\n" } else { "" },
            if pixels[i] { "#" } else { "." }
        );
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let reader = BufReader::new(File::open(&args[1]).unwrap());

    let mut value: i32 = 1;
    let mut cycle: usize = 0;

    let mut result = 0;
    let mut pixels = vec![false; 240];

    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        let mut parts = line.split(" ");

        let (step, add) = match parts.next() {
            Some("addx") => (2, parts.next().unwrap().parse::<i32>().unwrap()),
            Some("noop") => (1, 0),
            Some(s) => panic!("Unknown command {}", s),
            _ => panic!("Empty string"),
        };

        for _ in 0..step {
            pixels[cycle] = ((cycle % 40) as i32 - value).abs() <= 1;

            cycle += 1;
            if (cycle + 20) % 40 == 0 {
                result += (cycle as i32) * value;
            }
        }

        value += add;
    }

    println!("Part 1: {}", result);
    print_display(&pixels);
}
