use std::collections::HashMap;
use std::env;
use std::io::{prelude::*, BufReader};

#[derive(Debug)]
struct Dir {
    files: HashMap<String, usize>,
}

impl Dir {
    fn new() -> Self {
        Self {
            files: HashMap::new(),
        }
    }

    fn add_file(&mut self, name: &str, size: usize) {
        if self.files.contains_key(name) {
            return;
        }
        self.files.insert(String::from(name), size);
    }

    fn get_size(&self) -> usize {
        self.files.values().sum()
    }
}

type FS = HashMap<String, Dir>;

fn main() {
    let args: Vec<String> = env::args().collect();
    let reader = BufReader::new(std::fs::File::open(&args[1]).unwrap());

    let mut fs = FS::new();
    fs.insert(String::from(""), Dir::new());

    let mut path = String::from("");
    let mut dirname = String::from("");
    let mut current = fs.get_mut("").unwrap();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut words = line.split(" ");

        match words.next() {
            Some("$") => match words.next() {
                Some("cd") => match words.next() {
                    Some("/") => {
                        path = String::from("");
                        dirname = String::from("");
                        current = fs.get_mut(&path).unwrap();
                    }
                    Some("..") => {
                        path = path
                            .strip_suffix(&format!("/{}", dirname))
                            .unwrap()
                            .to_string();
                        dirname = path.rsplit("/").next().unwrap_or("").to_string();
                        current = fs.get_mut(&path).unwrap();
                    }
                    Some(dir) => {
                        path.push_str(&format!("/{}", dir));
                        dirname = dir.to_string();
                        fs.insert(path.clone(), Dir::new());
                        current = fs.get_mut(&path).unwrap();
                    }
                    None => (),
                },
                _ => (),
            },
            Some("dir") => (),
            Some(word) => {
                current.add_file(words.next().unwrap(), word.parse::<usize>().unwrap());
            }
            None => (),
        }
    }

    let dir_sizes: Vec<usize> = fs
        .keys()
        .map(|name| {
            fs.iter()
                .filter(|(key, _)| key.starts_with(name))
                .map(|(_, value)| value.get_size())
                .sum()
        })
        .collect();
    let part1: usize = dir_sizes.iter().filter(|&&size| size <= 100000).sum();
    println!("Part 1: {}", part1);

    let total_size : usize = fs.values().map(|dir| dir.get_size()).sum();

    let part2 = dir_sizes.iter().filter(|&&size| size >= total_size - 40000000).min().unwrap();
    println!("Part 2: {}", part2);
}
