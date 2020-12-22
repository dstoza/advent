#![deny(clippy::all, clippy::pedantic)]
#![feature(test)]

extern crate test;

use std::collections::VecDeque;

use clap::{crate_name, App, Arg};
use common::LineReader;

fn main() {
    let args = App::new(crate_name!())
        .arg(Arg::from_usage("<FILE>"))
        .get_matches();

    let mut reader = LineReader::new(args.value_of("FILE").unwrap());

    let mut player1 = VecDeque::new();
    reader.read_with(|line| {
        if line.len() > 2 {
            return;
        }

        player1.push_back(
            line.parse::<usize>()
                .unwrap_or_else(|_| panic!("Failed to parse {}", line)),
        )
    });

    let mut player2 = VecDeque::new();
    reader.read_with(|line| {
        if line.len() > 2 {
            return;
        }

        player2.push_back(
            line.parse::<usize>()
                .unwrap_or_else(|_| panic!("Failed to parse {}", line)),
        )
    });

    println!("player1: {:?}", player1);
    println!("player2: {:?}", player2);

    while !player1.is_empty() && !player2.is_empty() {
        let card1 = player1.pop_front().unwrap();
        let card2 = player2.pop_front().unwrap();

        if card1 > card2 {
            player1.push_back(card1);
            player1.push_back(card2);
        } else {
            player2.push_back(card2);
            player2.push_back(card1);
        }
    }

    println!("player1: {:?}", player1);
    println!("player2: {:?}", player2);

    let score: usize = 
    if !player1.is_empty() {
        player1.iter().enumerate().map(|(index, card)| (player1.len() - index) * *card).sum()
    } else {
        player2.iter().enumerate().map(|(index, card)| (player2.len() - index) * *card).sum()
    };
    println!("Score: {}", score);
}

#[cfg(test)]
mod tests {
    // use test::Bencher;
}
