#![deny(clippy::all, clippy::pedantic)]
#![feature(test)]

extern crate test;

use clap::{crate_name, App, Arg};

fn main() {
    let args = App::new(crate_name!())
        .arg(Arg::from_usage("<INPUT>"))
        .arg(Arg::from_usage("<STEPS>"))
        .get_matches();

    let mut next_cup = Vec::new();
    next_cup.resize(1_000_001, 0);

    let mut max = 0;
    let mut head = 0;
    let mut tail = 0;
    for value in args.value_of("INPUT").unwrap().chars().map(|character| {
        String::from(character)
            .parse::<usize>()
            .expect("Failed to parse cup as u8")
    }) {
        max = max.max(value);
        if head == 0 {
            head = value;
        }
        if tail != 0 {
            next_cup[tail] = value;
        }
        tail = value;
    }

    // let cup_count = max;

    let cup_count = 1_000_000;
    for value in max + 1..=cup_count {
        next_cup[tail] = value;
        tail = value;
    }

    // Complete the circular list
    next_cup[tail] = head;

    let steps: usize = args.value_of("STEPS").unwrap().parse().unwrap();

    let mut current = head;
    for _ in 0..steps {
        let mut destination = (current + cup_count - 2) % cup_count + 1;

        let mut picker = current;
        let mut picked_up = [0; 3];
        for i in 0..3 {
            picker = next_cup[picker];
            picked_up[i] = picker;
        }
        next_cup[current] = next_cup[picker];

        // println!("Picked up: {:?}", picked_up);

        while picked_up.iter().any(|value| *value == destination) {
            destination = (destination + cup_count - 2) % cup_count + 1
        }

        // println!("Destination: {}", destination);

        let destination_next = next_cup[destination];
        next_cup[destination] = *picked_up.first().unwrap();
        next_cup[*picked_up.last().unwrap()] = destination_next;

        current = next_cup[current];
    }

    while current != 1 {
        current = next_cup[current];
    }

    current = next_cup[current];

    let mut product = 1;
    for _ in 0..2 {
        product *= current as u64;
        current = next_cup[current];
    }

    println!("Product: {}", product);

    /*
    for _ in 0..cup_count - 1 {
        print!("{}", current);
        current = next_cup[current];
    }
    println!();
    */
}

#[cfg(test)]
mod tests {
    // use test::Bencher;
}
