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
            .parse::<u32>()
            .expect("Failed to parse cup as u8")
    }) {
        max = max.max(value);
        if head == 0 {
            head = value;
        }
        if tail != 0 {
            next_cup[tail as usize] = value;
        }
        tail = value;
    }

    // let cup_count = max;

    let cup_count = 1_000_000;
    for value in max + 1..=cup_count {
        next_cup[tail as usize] = value;
        tail = value;
    }

    // Complete the circular list
    next_cup[tail as usize] = head;

    let steps: usize = args.value_of("STEPS").unwrap().parse().unwrap();

    let mut current = head;
    for _ in 0..steps {
        let mut pick_cursor = current;
        let mut picked = [0; 3];
        for pick in &mut picked {
            pick_cursor = next_cup[pick_cursor as usize];
            *pick = pick_cursor;
        }
        next_cup[current as usize] = next_cup[pick_cursor as usize];

        let mut destination = (current + cup_count - 2) % cup_count + 1;
        while picked.iter().any(|value| *value == destination) {
            destination = (destination + cup_count - 2) % cup_count + 1
        }

        let destination_next = next_cup[destination as usize];
        next_cup[destination as usize] = picked[0];
        next_cup[picked[picked.len() - 1] as usize] = destination_next;

        current = next_cup[current as usize];
    }

    while current != 1 {
        current = next_cup[current as usize];
    }

    current = next_cup[current as usize];

    /*
    for _ in 0..cup_count - 1 {
        print!("{}", current);
        current = next_cup[current];
    }
    println!();
    */

    let mut product = 1;
    for _ in 0..2 {
        product *= u64::from(current);
        current = next_cup[current as usize];
    }

    println!("Product: {}", product);
}

#[cfg(test)]
mod tests {
    // use test::Bencher;
}
