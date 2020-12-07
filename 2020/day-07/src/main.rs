use std::{
    collections::HashMap,
    collections::{HashSet, VecDeque},
    env,
    fs::File,
    io::{BufRead, BufReader},
};

struct Bag {
    name: String,
    count: i32,
}

struct BagTracker {
    held_by: HashMap<String, Vec<String>>,
    holds: HashMap<String, Vec<Bag>>,
}

impl BagTracker {
    fn new() -> Self {
        Self {
            held_by: HashMap::new(),
            holds: HashMap::new(),
        }
    }

    fn parse_line(&mut self, line: &str) {
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

                Some(Bag {
                    name: String::from(&description[2..]),
                    count: description[0..1]
                        .parse()
                        .expect("Failed to parse count as i32"),
                })
            })
            .for_each(|containee| {
                self.held_by
                    .entry(containee.name.clone())
                    .or_default()
                    .push(String::from(container));

                self.holds
                    .entry(String::from(container))
                    .or_default()
                    .push(containee);
            });
    }

    fn compute_container_count(&self, name: &str) -> usize {
        let mut work_queue = VecDeque::new();
        work_queue.push_back(name);

        let mut containers = HashSet::new();

        while !work_queue.is_empty() {
            let current = work_queue
                .pop_front()
                .expect("Failed to pop front of queue");
            if let Some(parents) = self.held_by.get(current) {
                for parent in parents {
                    if containers.insert(parent) {
                        work_queue.push_back(parent)
                    }
                }
            }
        }

        containers.len()
    }

    fn compute_containee_count(
        &self,
        container: &Bag,
        containee_counts: &mut HashMap<String, i32>,
    ) -> i32 {
        if let Some(count) = containee_counts.get(&container.name) {
            return container.count * (1 + *count);
        }

        let mut containee_count = 0;
        for containee in self.holds.get(&container.name).unwrap_or(&Vec::new()) {
            containee_count += self.compute_containee_count(containee, containee_counts);
        }

        containee_counts.insert(container.name.clone(), containee_count);
        container.count * (1 + containee_count)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return;
    }

    let filename = &args[1];
    let file = File::open(filename).expect(format!("Failed to open file {}", filename).as_str());
    let mut reader = BufReader::new(file);

    let mut tracker = BagTracker::new();

    let mut line = String::new();
    loop {
        let bytes = reader.read_line(&mut line).expect("Failed to read line");
        if bytes == 0 {
            break;
        }

        tracker.parse_line(&line);

        line.clear();
    }

    println!(
        "Can contain shiny gold: {}",
        tracker.compute_container_count("shiny gold")
    );

    // Subtract 1 since we don't want to account for the shiny gold bag itself
    println!(
        "Shiny gold contains: {}",
        tracker.compute_containee_count(
            &Bag {
                name: String::from("shiny gold"),
                count: 1,
            },
            &mut HashMap::new()
        ) - 1
    );
}
