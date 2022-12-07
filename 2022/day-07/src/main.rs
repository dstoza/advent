#![warn(clippy::pedantic)]
use std::{
    cell::RefCell,
    io::{BufRead, BufReader},
    iter::Iterator,
    rc::{Rc, Weak},
};

struct Directory {
    name: String,
    parent: Weak<RefCell<Self>>,
    contents: Vec<Node>,
}

impl Directory {
    fn new(name: String, parent: Weak<RefCell<Self>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Directory {
            name,
            parent,
            contents: Vec::new(),
        }))
    }

    fn add_node(&mut self, node: Node) {
        self.contents.push(node);
    }

    fn get_directory(&self, name: &str) -> Rc<RefCell<Self>> {
        for item in &self.contents {
            if let Node::Directory(d) = item {
                if d.borrow().name == name {
                    return d.clone();
                }
            }
        }
        unreachable!()
    }

    fn get_total_size(&self) -> usize {
        self.contents
            .iter()
            .map(|item| match item {
                Node::Directory(d) => d.borrow().get_total_size(),
                Node::File(f) => f.size,
            })
            .sum()
    }

    fn get_directory_sizes(&self) -> Vec<(String, usize)> {
        let mut sizes = vec![(self.name.clone(), self.get_total_size())];
        for mut child in self.contents.iter().filter_map(|node| match node {
            Node::Directory(d) => Some(d.borrow().get_directory_sizes()),
            Node::File(_) => None,
        }) {
            sizes.append(&mut child);
        }
        sizes
    }
}

struct File {
    _name: String,
    size: usize,
}

impl File {
    fn new(name: String, size: usize) -> Self {
        Self { _name: name, size }
    }
}

enum Node {
    Directory(Rc<RefCell<Directory>>),
    File(File),
}

fn parse_file_tree(lines: impl Iterator<Item = String>) -> Rc<RefCell<Directory>> {
    let mut lines = lines.peekable();

    // Skip the first line and create the root node
    lines.next();
    let root = Directory::new(String::from("/"), Weak::new());
    let mut current = root.clone();

    let mut line = lines.next().unwrap();
    loop {
        if line == "$ ls" {
            for possible_node in lines.by_ref() {
                if possible_node.starts_with('$') {
                    line = possible_node;
                    break;
                }

                let node = if possible_node.starts_with("dir") {
                    Node::Directory(Directory::new(
                        String::from(possible_node.strip_prefix("dir ").unwrap()),
                        Rc::downgrade(&current),
                    ))
                } else {
                    let mut split = possible_node.split(' ');
                    let size = split.next().unwrap().parse().unwrap();
                    let name = split.next().unwrap();
                    Node::File(File::new(String::from(name), size))
                };
                current.borrow_mut().add_node(node);
            }

            if lines.peek().is_none() {
                break;
            }
        } else if line == "$ cd .." {
            let parent = current.borrow().parent.upgrade().unwrap();
            current = parent;
            line = lines.next().unwrap();
        } else if line.starts_with("$ cd") {
            let dir_name = line.strip_prefix("$ cd ").unwrap();
            let child = current.borrow().get_directory(dir_name);
            current = child;
            line = lines.next().unwrap();
        }
    }

    root
}

fn main() {
    let filename = std::env::args().nth(1).expect("Filename not found");

    let file = std::fs::File::open(&filename)
        .unwrap_or_else(|_| panic!("Couldn't open {}", filename.as_str()));
    let reader = BufReader::new(file);

    let root = parse_file_tree(reader.lines().map(std::result::Result::unwrap));

    let mut directory_sizes = root.borrow().get_directory_sizes();
    let total_capped_size: usize = directory_sizes
        .iter()
        .map(|(_name, size)| if *size <= 100_000 { *size } else { 0 })
        .sum();
    println!("Total capped size {total_capped_size}");

    let to_free = root.borrow().get_total_size() - 40_000_000;
    println!("Need to free {to_free}");

    directory_sizes.sort_unstable_by_key(|(_name, size)| *size);
    let (name, size) = directory_sizes
        .iter()
        .find(|(_name, size)| *size >= to_free)
        .unwrap();
    println!("Should free {name} to save {size}");
}
