#![deny(clippy::all, clippy::pedantic)]
#![feature(test)]

extern crate test;

use clap::{crate_name, App, Arg};

struct Transformer {
    subject: u64,
    value: u64,
    loop_count: u32
}

impl Transformer {
    fn new(subject: u64) -> Self {
        Self {
            subject,
            value: 1,
            loop_count: 0,
        }
    }

    fn run_loop(&mut self) {
        self.value *= self.subject;
        self.value %= 20201227;
        self.loop_count += 1;
    }

    fn get_value(&self) -> u64 {
        self.value
    }

    fn get_loop_count(&self) -> u32 {
        self.loop_count
    }
}

fn main() {
    let args = App::new(crate_name!())
        .arg(Arg::from_usage("<CARD>"))
        .arg(Arg::from_usage("<ROOM>"))
        .get_matches();

    let card_public_key: u64 = args.value_of("CARD").unwrap().parse().expect("Failed to parse card public key as u64");
    let room_public_key: u64 = args.value_of("ROOM").unwrap().parse().expect("Failed to parse room public key as u64");

    let mut card_transformer = Transformer::new(7);
    let mut room_transformer = Transformer::new(7);

    loop {
        card_transformer.run_loop();
        room_transformer.run_loop();

        if card_transformer.get_value() == card_public_key {
            let mut key_transformer = Transformer::new(room_public_key);
            for _ in 0..card_transformer.get_loop_count() {
                key_transformer.run_loop();
            }
            println!("Encryption key: {}", key_transformer.get_value());
            return;
        }
        if room_transformer.get_value() == room_public_key {
            let mut key_transformer = Transformer::new(card_public_key);
            for _ in 0..room_transformer.get_loop_count() {
                key_transformer.run_loop();
            }
            println!("Encryption key: {}", key_transformer.get_value());
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    // use test::Bencher;
}
