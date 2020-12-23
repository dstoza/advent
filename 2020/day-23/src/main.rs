#![deny(clippy::all, clippy::pedantic)]
#![feature(test)]

extern crate test;

use clap::{crate_name, App, Arg};
use common::LineReader;

fn main() {
    let args = App::new(crate_name!())
        .arg(Arg::from_usage("<INPUT>"))
        .arg(Arg::from_usage("<STEPS>"))
        .get_matches();

    let mut cups: Vec<_> = args
        .value_of("INPUT")
        .unwrap()
        .chars()
        .map(|character| {
            String::from(character)
                .parse::<u8>()
                .expect("Failed to parse cup as u8")
        })
        .collect();
    let cup_count = cups.len();

    let steps: usize = args.value_of("STEPS").unwrap().parse().unwrap();

    let mut current = 0;
    let mut pickup = [0; 3];
    for _ in 0..steps {
        let mut cursor = current + 1;
        let mut destination = (cups[current] as usize + cup_count - 2) % cup_count + 1;
        
        for p in 0..3 {
            pickup[p] = cups[cursor % cup_count];
            cursor += 1;
        }

        while pickup
            .iter()
            .find(|cup| **cup as usize == destination)
            .is_some()
        {
            destination = (destination + cup_count - 2) % cup_count + 1;
        }

        // Copy elements from after the picked-up cups until we find the destination
        cursor = (current + 1) % cup_count;
        while cups[(cursor + 3) % cup_count] as usize != destination {
            cups[cursor] = cups[(cursor + 3) % cup_count];
            cursor = (cursor + 1) % cup_count;
        }

        // Now we're at the destination, so copy it
        cups[cursor] = cups[(cursor + 3) % cup_count];
        cursor = (cursor + 1) % cup_count;

        // Fill in the picked-up cups
        for p in 0..3 {
            cups[cursor] = pickup[p];
            cursor = (cursor + 1) % cup_count;
        }

        // The remainder of the cups don't need to move

        current = (current + 1) % cup_count;
    }

    let mut cursor = 0;
    while cups[cursor] != 1 {
        cursor += 1;
    }
    cursor += 1;
    for _ in 0..cup_count - 1 {
        print!("{}", cups[cursor]);
        cursor = (cursor + 1) % cup_count;
    }
    println!();
}

#[cfg(test)]
mod tests {
    // use test::Bencher;
}
