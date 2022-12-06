use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() {
    let args: Vec<String> = env::args().collect();
    let reader = BufReader::new(File::open(&args[1]).unwrap());

    let line = reader.lines().next().expect("Failed to read line").unwrap();
    let characters : Vec<char> = line.chars().collect();

    let unique = 4;

    for (i, slice) in characters.windows(unique).enumerate() {
        let mut slice = slice.iter().cloned().collect::<Vec<char>>();
        slice.sort();
        slice.dedup();

        if slice.len() == unique {
            println!("{}", i + unique);
            break;
        }
    }
}
