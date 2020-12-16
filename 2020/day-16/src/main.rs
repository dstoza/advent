#![deny(clippy::all, clippy::pedantic)]

use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

use bit_set::BitSet;

struct Range {
    begin: i32,
    end: i32,
}

struct Field {
    id: usize,
    name: String,
    ranges: Vec<Range>,
}

struct TicketValidator {
    fields: Vec<Field>,
}

impl TicketValidator {
    fn new() -> Self {
        Self { fields: Vec::new() }
    }

    fn add_field(&mut self, line: &str) {
        let mut split = line.split(':');

        let name = split.next().expect("Failed to find field name");

        let ranges: Vec<Range> = split
            .next()
            .expect("Failed to find ranges")
            .trim()
            .split(" or ")
            .map(|range| {
                let mut endpoints = range.split('-');
                let begin: i32 = endpoints
                    .next()
                    .expect("Failed to find beginning of range")
                    .parse()
                    .expect("Failed to parse beginning of range as i32");
                let end: i32 = endpoints
                    .next()
                    .expect("Failed to find end of range")
                    .parse()
                    .expect("Failed to parse end of range as i32");
                Range { begin, end }
            })
            .collect();

        self.fields.push(Field {
            id: self.fields.len(),
            name: String::from(name),
            ranges,
        });
    }

    fn get_invalid_sum(&self, ticket: &str) -> Option<i32> {
        let mut sum = None;

        for value in ticket.split(',').map(|value| {
            value
                .parse::<i32>()
                .expect("Failed to parse field value as i32")
        }) {
            if !self
                .fields
                .iter()
                .map(|field| &field.ranges)
                .flat_map(|ranges| ranges.iter())
                .any(|range| value >= range.begin && value <= range.end)
            {
                *sum.get_or_insert(0) += value;
            }
        }

        sum
    }

    fn get_possible_field_ids(&self, ticket: &str) -> Vec<BitSet> {
        let mut possibilities = Vec::new();

        for value in ticket.split(',').map(|value| {
            value
                .parse::<i32>()
                .expect("Failed to parse field value as i32")
        }) {
            let field_ids: BitSet = self
                .fields
                .iter()
                .filter_map(|field| {
                    for range in &field.ranges {
                        if value >= range.begin && value <= range.end {
                            return Some(field.id);
                        }
                    }
                    None
                })
                .collect();
            possibilities.push(field_ids);
        }

        possibilities
    }

    fn get_field_name(&self, id: usize) -> String {
        for field in &self.fields {
            if field.id == id {
                return field.name.clone();
            }
        }

        String::from("Unknown")
    }
}

fn simplify_possibilities(possibilities: &mut Vec<BitSet>) {
    let mut singletons: Vec<usize> = possibilities
        .iter()
        .filter_map(|field_possibilities| {
            if field_possibilities.len() == 1 {
                Some(
                    field_possibilities
                        .iter()
                        .next()
                        .expect("Failed to get only element"),
                )
            } else {
                None
            }
        })
        .collect();

    while !singletons.is_empty() {
        let singleton = singletons
            .pop()
            .expect("Failed to get singleton from non-empty collection");

        for field_possibilities in &mut *possibilities {
            if field_possibilities.len() > 1 {
                field_possibilities.remove(singleton);
                if field_possibilities.len() == 1 {
                    singletons.push(
                        field_possibilities
                            .iter()
                            .next()
                            .expect("Failed to get only singleton"),
                    );
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return;
    }

    let filename = &args[1];
    let file = File::open(filename).unwrap_or_else(|_| panic!("Failed to open file {}", filename));
    let mut reader = BufReader::new(file);

    let mut validator = TicketValidator::new();

    let mut line = String::new();

    // Parse fields
    loop {
        reader
            .read_line(&mut line)
            .unwrap_or_else(|_| panic!("Failed to read field"));

        if line.trim() == "" {
            break;
        }

        validator.add_field(line.trim());

        line.clear();
    }

    // Skip "your ticket" header
    reader
        .read_line(&mut line)
        .unwrap_or_else(|_| panic!("Failed to read 'your ticket' header"));

    line.clear();
    reader
        .read_line(&mut line)
        .unwrap_or_else(|_| panic!("Failed to read your ticket"));
    let your_ticket = String::from(line.trim());

    // Skip blank line and "nearby tickets" header
    reader
        .read_line(&mut line)
        .unwrap_or_else(|_| panic!("Failed to read blank line"));
    reader
        .read_line(&mut line)
        .unwrap_or_else(|_| panic!("Failed to read 'nearby tickets' header"));
    line.clear();

    let mut possibilities = Vec::new();

    let mut invalid_sum = 0;
    loop {
        let bytes = reader
            .read_line(&mut line)
            .unwrap_or_else(|_| panic!("Failed to read line"));
        if bytes == 0 {
            break;
        }

        if let Some(ticket_sum) = validator.get_invalid_sum(line.trim()) {
            invalid_sum += ticket_sum;
        } else if possibilities.is_empty() {
            possibilities = validator.get_possible_field_ids(line.trim());
        } else {
            let ticket_possibilities = validator.get_possible_field_ids(line.trim());
            for i in 0..possibilities.len() {
                possibilities[i].intersect_with(&ticket_possibilities[i]);
            }
        }

        line.clear();
    }

    simplify_possibilities(&mut possibilities);

    let mut your_values = your_ticket
        .split(',')
        .map(|field| field.parse::<i64>().expect("Failed to parse field as i64"));

    let product: i64 = possibilities
        .iter()
        .filter_map(|field_possibilities| {
            let value = your_values.next().expect("Failed to find field value");
            let field_name = validator.get_field_name(
                field_possibilities
                    .iter()
                    .next()
                    .expect("Failed to find only field id"),
            );

            if field_name.len() >= 9 && &field_name[0..9] == "departure" {
                Some(value)
            } else {
                None
            }
        })
        .product();

    println!("Invalid sum: {}", invalid_sum);
    println!("Your product: {}", product);
}
