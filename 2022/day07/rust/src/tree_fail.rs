use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::io::{prelude::*, BufReader};
use std::path::Prefix;
use std::rc::Rc;

#[derive(Debug)]
pub struct File {
    name: String,
    size: usize,
}

#[derive(Debug)]
pub struct Directory {
    name: String,
    directories: HashMap<String, Directory>,
    files: HashMap<String, File>
}

impl Directory {
    fn new(name: &str) -> Self {
        Directory {
            name: String::from(name),
            directories: HashMap::new(),
            files: HashMap::new()
        }
    }

    fn add_dir(&mut self, name: &str) {
        let new_dir = Directory {
            name: String::from(name),
            directories: HashMap::new(),
            files: HashMap::new()
        };

        self.directories.insert(String::from(name), new_dir);
    }

    fn add_file(&mut self, name: &str, size: usize) {
        let new_file = File {
            name: String::from(name), size
        };

        self.files.insert(String::from(name), new_file);
    }

    fn print(self) {
        self.print_impl("- ");
    }

    fn print_impl(self, prefix: &str) {
        println!("{}{} (dir)", prefix, self.name);

        let mut prefix = String::from(prefix);
        prefix.insert_str(0, "  ");

        for (_, dir) in self.directories {
            dir.print_impl(&prefix);
        }

        for (_, file) in self.files {
            println!("{}{} (file, size={})", prefix, file.name, file.size);
        }
    }
}

#[derive(Debug)]
struct DirectoryRef<'a> {
    node: &'a mut Directory,
    parent: Option<&'a DirectoryRef<'a>>
}

impl<'a> DirectoryRef<'a> {
    fn subdir(&mut self, name: &str) -> DirectoryRef {
        DirectoryRef {
            node: self.node.directories.get_mut(name).unwrap(),
            parent: Some(self)
        }
    }
}

// fn read_lines(directory: &mut Directory,  lines: &mut std::io::Lines<BufReader<std::fs::File>>) {
//     let line = lines.next();
//     if line.is_none() {
//         return;
//     }
//
//     let line = line.expect("Failed to read line");
//     let mut words = line.unwrap().split(" ");
//
//     match words.next() {
//         Some("$") => match words.next() {
//             Some("cd") => {
//                 let next_dir = &mut directory.directories.get_mut(words.next().unwrap()).unwrap();
//                 read_lines(next_dir, lines);
//                 return;
//             }
//             _ => ()
//         },
//         Some("dir") => {
//             let dirname = String::from(words.next().unwrap());
//             directory
//                 .directories
//                 .insert(dirname.clone(), Directory::new(&dirname));
//         }
//         Some(n) => {
//             let size = n.parse::<usize>();
//             let filename = n.
//
//         }
//         None => panic!("No words"),
//     }
//     read_lines(directory, lines)
// }
