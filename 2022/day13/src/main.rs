use std::cmp::Ordering;
use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Clone, Debug)]
enum Element {
    Multiple(Vec<Element>),
    Single(i32),
}

fn parse_line(line: &str) -> Element {
    let mut result: Vec<Element> = Vec::new();
    let chars: Vec<char> = line.chars().collect();

    if chars[0] != '[' {
        return Element::Single(line.parse::<i32>().unwrap());
    }

    let mut i = 1;
    while i < chars.len() - 1 {
        match chars[i] {
            '[' => {
                let mut depth = 0;

                let mut j = i;
                while j < chars.len() - 1 {
                    match chars[j] {
                        '[' => depth += 1,
                        ']' => {
                            depth -= 1;
                            if depth == 0 {
                                break;
                            }
                        }
                        _ => (),
                    }

                    j += 1;
                }

                result.push(parse_line(&line[i..j + 1]));
                i = j + 1;
            }
            ',' | ' ' => {
                i += 1;
            }
            _ => {
                let mut j = i + 1;
                while chars[j] != ',' && chars[j] != ']' {
                    j += 1;
                }

                result.push(Element::Single(line[i..j].parse::<i32>().unwrap()));
                i = j;
            }
        }
    }

    return Element::Multiple(result);
}

impl Element {
    fn to_multiple(&self) -> Vec<Element> {
        match self {
            Element::Multiple(multiple) => multiple.clone(),
            Element::Single(single) => vec![Element::Single(*single)],
        }
    }

    fn to_string(&self) -> String {
        match self {
            Element::Multiple(multiple) => {
                let mut result = String::from("[");
                for item in multiple {
                    result.push_str(&item.to_string());
                    result.push_str(",");
                }
                result.push_str("]");
                result
            }
            Element::Single(single) => format!("{}", single),
        }
    }
}

fn cmp(lhs: &Element, rhs: &Element) -> std::cmp::Ordering {
    // println!("cmp({}, {})", lhs.to_string(), rhs.to_string());

    if let Element::Single(lhs_single) = lhs {
        if let Element::Single(rhs_single) = rhs {
            return lhs_single.cmp(rhs_single);
        }
    }

    return cmp_multiple(&lhs.to_multiple(), &rhs.to_multiple());
}

fn cmp_multiple(lhs: &Vec<Element>, rhs: &Vec<Element>) -> Ordering {
    for (i, lhs_elem) in lhs.iter().enumerate() {
        if i == rhs.len() {
            return Ordering::Greater;
        }

        let ord = cmp(lhs_elem, &rhs[i]);
        if ord != Ordering::Equal {
            return ord;
        }
    }

    if lhs.len() == rhs.len() {
        return Ordering::Equal;
    }

    return Ordering::Less;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut reader = BufReader::new(File::open(&args[1]).unwrap()).lines();

    let mut packets: Vec<Element> = Vec::new();

    let mut index = 1;
    let mut count = 0;
    loop {
        let line = match reader.next() {
            Some(line) => line.expect("Failed to read line"),
            None => break,
        };

        if line == "" {
            continue;
        }

        let lhs = parse_line(&line);
        let rhs = parse_line(&reader.next().unwrap().unwrap());

        if cmp(&lhs, &rhs) != Ordering::Greater {
            count += index;
        }

        packets.push(lhs);
        packets.push(rhs);

        index += 1;
    }

    println!("Part 1: {}", count);

    packets.push(parse_line("[[2]]"));
    packets.push(parse_line("[[6]]"));
    packets.sort_by(|x, y| cmp(x, y));

    let mut start = 0;
    let mut stop = 0;

    for (i, packet) in packets.iter().enumerate() {
        match packet {
            Element::Multiple(mult) => {
                if mult.len() != 1 {
                    continue;
                }

                match &mult[0] {
                    Element::Multiple(mult2) => {
                        if mult2.len() != 1 {
                            continue;
                        }

                        match mult2[0] {
                            Element::Single(x) => match x {
                                2 => start = i + 1,
                                6 => stop = i + 1,
                                _ => (),
                            },
                            _ => (),
                        }
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }

    println!("{} * {} = {}", start, stop, start * stop);
}
