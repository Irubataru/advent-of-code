use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

struct Monkey {
    items: Vec<u128>,
    operation: Box<dyn Fn(u128) -> u128>,
    divisor: u128,
    to_true: usize,
    to_false: usize,
    inspections: u128,
}

fn parse_monkeys() -> Vec<Monkey> {
    let args: Vec<String> = env::args().collect();
    let mut reader = BufReader::new(File::open(&args[1]).unwrap()).lines();

    let mut monkeys: Vec<Monkey> = Vec::new();

    loop {
        // Read Monkey i
        match reader.next() {
            Some(_) => (),
            None => break,
        };

        // Read "  Starting items: ..."
        let line = reader.next().unwrap().unwrap()[18..].to_string();
        let start_items: Vec<u128> = line
            .split(", ")
            .map(|s| s.parse::<u128>().unwrap_or(0))
            .collect();

        // Read "  Operation: new = ..."
        let line = reader.next().unwrap().unwrap()[19..].to_string();
        let expression_parts: Vec<&str> = line.split(" ").collect();

        let operation: Box<dyn Fn(u128) -> u128> = match expression_parts[1] {
            "+" => {
                let add = expression_parts[2].parse::<u128>().unwrap();
                Box::new(move |x| x + add)
            }
            "*" => match expression_parts[2] {
                "old" => Box::new(|x| x * x),
                s => {
                    let add = s.parse::<u128>().unwrap();
                    Box::new(move |x| x * add)
                }
            },
            s => panic!("Unknown operation {}", s),
        };

        // Read "  Test: divisible by ..."
        let line = reader.next().unwrap().unwrap()[21..].to_string();
        let divisor = line.parse::<u128>().unwrap();

        // Read "    If true: throw to monkey ..."
        let line = reader.next().unwrap().unwrap()[29..].to_string();
        let to_true = line.parse::<usize>().unwrap();

        // Read "    If true: throw to monkey ..."
        let line = reader.next().unwrap().unwrap()[30..].to_string();
        let to_false = line.parse::<usize>().unwrap();

        // Skip newline
        reader.next();

        monkeys.push(Monkey {
            items: start_items,
            operation,
            divisor,
            to_true,
            to_false,
            inspections: 0,
        });
    }

    monkeys
}

impl Monkey {
    fn inspect(&mut self, worry_scaling: u128, modulus: u128) -> Option<(u128, usize)> {
        if self.items.is_empty() {
            return None;
        }

        self.inspections += 1;

        let item = ((self.operation)(self.items.remove(0)) / worry_scaling) % modulus;
        let index = if (item % self.divisor) == 0 {
            self.to_true
        } else {
            self.to_false
        };

        Some((item, index))
    }
}

fn round(monkeys: &mut Vec<Monkey>, worry_scaling: u128) {
    let modulus = monkeys.iter().fold(1, |tot, m| tot * m.divisor);

    for i in 0..monkeys.len() {
        loop {
            let monkey = &mut monkeys[i];

            let (item, index) = match monkey.inspect(worry_scaling, modulus) {
                Some(x) => x,
                None => break,
            };

            monkeys[index].items.push(item);
        }
    }
}

fn run(iterations: usize, worry_scaling: u128) -> u128 {
    let mut monkeys = parse_monkeys();

    for _ in 0..iterations {
        round(&mut monkeys, worry_scaling);
    }

    let mut inspections: Vec<u128> = monkeys.iter().map(|m| m.inspections).collect();
    inspections.sort();
    inspections[inspections.len() - 1] * inspections[inspections.len() - 2]
}

fn main() {
    println!("Part 1: {}", run(20, 3));
    println!("Part 2: {}", run(10000, 1));
}
