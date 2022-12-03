use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn calculate_priority(item: char) -> u32 {
    if item.is_lowercase() {
        return item as u32 - 'a' as u32 + 1;
    } else {
        return item as u32 - 'A' as u32 + 27;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let reader = BufReader::new(File::open(&args[1]).unwrap());

    let mut priorities1 = 0;
    let mut priorities2 = 0;
    let mut current_set = HashSet::<char>::new();

    let mut iteration = 0;
    for line in reader.lines() {
        let chars: Vec<char> = line
            .expect("Failed to read line from file")
            .chars()
            .collect();

        // Part 1
        let len = chars.len() / 2;
        let left: HashSet<char> = chars.iter().take(len).cloned().collect();
        let right: HashSet<char> = chars.iter().skip(len).cloned().collect();

        priorities1 += left
            .intersection(&right)
            .map(|item| calculate_priority(*item))
            .sum::<u32>();

        // Part 2
        let items: HashSet<char> = chars.iter().cloned().collect();
        if iteration % 3 == 0 {
            current_set = items;
        } else {
            current_set = current_set.intersection(&items).cloned().collect();
        }

        if iteration % 3 == 2 {
            priorities2 += current_set
                .iter()
                .map(|item| calculate_priority(*item))
                .sum::<u32>();
        }

        iteration += 1;
    }

    println!("Sum of priorities: {}", priorities1);
    println!("Sum of common priorities: {}", priorities2);
}
