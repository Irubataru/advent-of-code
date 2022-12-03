use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let reader = BufReader::new(File::open(&args[1]).unwrap());

    let mut elf_calories: Vec<i32> = Vec::new();

    let mut aggregate = 0;
    for line in reader.lines() {
        let line = line.expect("Failed to read line from file");
        if line == "" {
            elf_calories.push(aggregate);
            aggregate = 0;
            continue;
        }

        aggregate += line.parse::<i32>().expect("Expected a list of numbers");
    }

    if aggregate != 0  {
        elf_calories.push(aggregate);
    }

    elf_calories.sort();

    println!("Elf with the most calories has {} calories.", elf_calories.last().unwrap());
    println!("Sum of the last 3 elves' calories is {}", elf_calories.iter().rev().take(3).sum::<i32>());
}
