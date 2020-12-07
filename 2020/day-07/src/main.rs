use std::{
    collections::HashMap,
    collections::{HashSet, VecDeque},
    env,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug)]
struct Containee {
    name: String,
    count: i32,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return;
    }

    let filename = &args[1];
    let file = File::open(filename).expect(format!("Failed to open file {}", filename).as_str());
    let mut reader = BufReader::new(file);

    let mut held_by = HashMap::<String, Vec<String>>::new();
    let mut holds = HashMap::<String, Vec<Containee>>::new();

    let mut line = String::new();
    loop {
        let bytes = reader.read_line(&mut line).expect("Failed to read line");
        if bytes == 0 {
            break;
        }

        let mut split = line.split("contain");
        let container = split
            .nth(0)
            .expect("Failed to find container")
            .strip_suffix(" bags ")
            .expect("Failed to strip 'bags' suffix");

        split
            .nth(0)
            .expect("Failed to find containees")
            .split(",")
            .filter_map(|token| {
                let description = token
                    .trim()
                    .trim_end_matches(".")
                    .trim_end_matches("s")
                    .strip_suffix(" bag")
                    .expect("Failed to strip 'bags' suffix");

                if description == "no other" {
                    return None;
                }

                Some(Containee {
                    name: String::from(&description[2..]),
                    count: description[0..1]
                        .parse()
                        .expect("Failed to parse count as i32"),
                })
            })
            .for_each(|containee| {
                held_by
                    .entry(containee.name.clone())
                    .or_default()
                    .push(String::from(container));

                holds
                    .entry(String::from(container))
                    .or_default()
                    .push(containee);
            });

        line.clear();
    }

    let mut work_queue = VecDeque::new();
    work_queue.push_back("shiny gold");

    let mut containers = HashSet::new();

    while !work_queue.is_empty() {
        let current = work_queue
            .pop_front()
            .expect("Failed to pop front of queue");
        if let Some(parents) = held_by.get(current) {
            for parent in parents {
                if containers.insert(parent) {
                    work_queue.push_back(parent)
                }
            }
        }
    }

    println!("Can contain shiny gold: {}", containers.len());

    let mut work_queue = VecDeque::new();
    work_queue.push_back(Containee {
        name: String::from("shiny gold"),
        count: 1,
    });

    // Initialize to -1 so we don't count the original shiny gold bag
    let mut contained_count = -1;

    while !work_queue.is_empty() {
        let current = work_queue
            .pop_front()
            .expect("Failed to pop front of queue");

        contained_count += current.count;

        if let Some(children) = holds.get(&current.name) {
            for child in children {
                work_queue.push_back(Containee {
                    name: child.name.clone(),
                    count: current.count * child.count,
                });
            }
        }
    }

    println!("Shiny gold contains: {}", contained_count);
}
