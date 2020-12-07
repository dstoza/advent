#![deny(clippy::all, clippy::pedantic)]

use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
    ops::AddAssign,
};

struct QuestionCounter {
    any_person: u32,
    all_people: u32,
}

struct Counts {
    any_person: u32,
    all_people: u32,
}

impl AddAssign for Counts {
    fn add_assign(&mut self, other: Self) {
        self.any_person += other.any_person;
        self.all_people += other.all_people;
    }
}

impl QuestionCounter {
    fn new() -> Self {
        Self {
            any_person: 0_u32,
            all_people: u32::MAX,
        }
    }

    fn parse_questions(&mut self, line: &str) {
        let mut individual = 0;
        for byte in line.as_bytes() {
            let offset = byte - b'a';
            assert!(offset < 32, "Byte out of range");
            individual |= 1 << offset;
        }

        self.any_person |= individual;
        self.all_people &= individual;
    }

    fn add_line(&mut self, line: &str) -> Option<Counts> {
        if !line.trim().is_empty() {
            self.parse_questions(line);
            return None;
        }

        let counts = Some(Counts {
            any_person: self.any_person.count_ones(),
            all_people: self.all_people.count_ones(),
        });

        *self = Self::new();

        counts
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return;
    }

    let filename = &args[1];
    let file = File::open(filename).unwrap_or_else(|_| panic!("Failed to open file {}", filename));
    let mut reader = BufReader::new(file);

    let mut counter = QuestionCounter::new();
    let mut counts = Counts {
        any_person: 0,
        all_people: 0,
    };

    let mut line = String::new();
    loop {
        let bytes = reader
            .read_line(&mut line)
            .unwrap_or_else(|_| panic!("Failed to read line"));
        if bytes == 0 {
            break;
        }

        if let Some(group) = counter.add_line(line.trim()) {
            counts += group;
        }

        line.clear();
    }

    counts += counter.add_line("").expect("Failed to find last record");

    println!("Any person: {}", counts.any_person);
    println!("All people: {}", counts.all_people);
}
