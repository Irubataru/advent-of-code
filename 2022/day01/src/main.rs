use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .filter(|item| item.is_ok())
        .map(|item| item.unwrap())
        .collect();

    let mut elf_calories: Vec<i32> = Vec::new();

    let mut calories = 0;
    for line in lines {
        if line == "" {
            elf_calories.push(calories);
            calories = 0;
            continue;
        }

        calories += line.parse::<i32>().unwrap();
    }

    if calories != 0  {
        elf_calories.push(calories);
    }

    elf_calories.sort();

    println!("Elf with the most calories has {} calories.", elf_calories.last().unwrap());
    println!("Sum of the last 3 elves' calories is {}", elf_calories.iter().rev().take(3).sum::<i32>());
}
