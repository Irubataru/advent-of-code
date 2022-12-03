use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug, Clone, Copy)]
enum Rps {
    Rock = 0,
    Paper = 1,
    Scissor = 2,
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Loss = 0,
    Draw = 1,
    Win = 2,
}

fn challenge_to_rps(value: char) -> Result<Rps, String> {
    match value {
        'A' => Ok(Rps::Rock),
        'B' => Ok(Rps::Paper),
        'C' => Ok(Rps::Scissor),
        _ => Err("Expected the challenge char to be in {A,B,C}".to_string()),
    }
}

fn response_to_rps(value: char) -> Result<Rps, String> {
    match value {
        'X' => Ok(Rps::Rock),
        'Y' => Ok(Rps::Paper),
        'Z' => Ok(Rps::Scissor),
        _ => Err("Expected the response char to be in {X,Y,Z}".to_string()),
    }
}

fn response_to_outcome(value: char) -> Result<Outcome, String> {
    match value {
        'X' => Ok(Outcome::Loss),
        'Y' => Ok(Outcome::Draw),
        'Z' => Ok(Outcome::Win),
        _ => Err("Expected the response char to be in {X,Y,Z}".to_string()),
    }
}

fn parse_line(line: String) -> (Rps, Rps, Outcome) {
    let inputs: Vec<char> = line.chars().collect();
    return (
        challenge_to_rps(inputs[0]).unwrap(),
        response_to_rps(inputs[2]).unwrap(),
        response_to_outcome(inputs[2]).unwrap(),
    );
}

fn get_outcome(challenge: Rps, response: Rps) -> Outcome {
    let encoded_outcome = 3 + (response as u8) - (challenge as u8);

    match encoded_outcome % 3 {
        0 => return Outcome::Draw,
        1 => return Outcome::Win,
        2 => return Outcome::Loss,
        _ => panic!("Math is broken"),
    }
}

fn calculate_score(challenge: Rps, response: Rps) -> i32 {
    return (1 + (response as u8) + 3 * (get_outcome(challenge, response) as u8)) as i32;
}

fn calculate_score_from_outcome(challenge: Rps, outcome: Outcome) -> i32 {
    return (1 + ((2 + challenge as u8 + outcome as u8) % 3) + 3 * (outcome as u8)) as i32;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let reader = BufReader::new(File::open(&args[1]).unwrap());

    let mut score1 = 0;
    let mut score2 = 0;
    for line in reader.lines() {
        let line = line.expect("Failed to read line from file");
        let (challenge, response, outcome) = parse_line(line);
        score1 += calculate_score(challenge, response);
        score2 += calculate_score_from_outcome(challenge, outcome);
    }

    println!("Part 1: {}", score1);
    println!("Part 2: {}", score2);
}
