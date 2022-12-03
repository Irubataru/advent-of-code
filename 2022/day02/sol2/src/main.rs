use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let reader = BufReader::new(File::open(&args[1]).unwrap());

    let mut score1 = 0;
    let mut score2 = 0;

    for line in reader.lines() {
        let chars: Vec<char> = line
            .expect("Failed to read line from file")
            .chars()
            .collect();
        let challenge = chars[0] as u32 - 'A' as u32;
        let response = chars[2] as u32 - 'X' as u32;

        score1 += 1 + response + 3 * ((4 + response - challenge) % 3);
        score2 += 1 + ((2 + challenge + response) % 3) + 3 * response;
    }

    println!("Part 1: {}", score1);
    println!("Part 2: {}", score2);
}
