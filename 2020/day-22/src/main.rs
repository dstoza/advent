#![deny(clippy::all, clippy::pedantic)]
#![feature(test)]

extern crate test;

use std::collections::{HashSet, VecDeque};

use clap::{crate_name, App, Arg};
use common::LineReader;

fn compute_score(deck: &VecDeque<u8>) -> usize {
    deck.iter()
        .enumerate()
        .map(|(index, card)| (deck.len() - index) * (*card as usize))
        .sum()
}

fn play_basic_game(mut player1: VecDeque<u8>, mut player2: VecDeque<u8>) -> usize {
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

    if player1.is_empty() {
        compute_score(&player2)
    } else {
        compute_score(&player1)
    }
}

fn play_recursive_game(mut player1: VecDeque<u8>, mut player2: VecDeque<u8>) -> (i8, usize) {
    let mut previous_rounds = HashSet::new();

    loop {
        let concatenated: Vec<_> = player1
            .iter()
            .chain(std::iter::once(&u8::max_value()))
            .chain(player2.iter())
            .copied()
            .collect();

        if previous_rounds.contains(&concatenated) {
            return (1, 0);
        }

        previous_rounds.insert(concatenated);

        let card1 = player1.pop_front().unwrap();
        let card2 = player2.pop_front().unwrap();

        let winner = if player1.len() >= card1 as usize && player2.len() >= card2 as usize {
            let (winner, _) = play_recursive_game(
                player1.iter().take(card1 as usize).copied().collect(),
                player2.iter().take(card2 as usize).copied().collect(),
            );
            winner
        } else if card1 > card2 {
            1
        } else {
            2
        };

        match winner {
            1 => {
                player1.push_back(card1);
                player1.push_back(card2);
            }
            2 => {
                player2.push_back(card2);
                player2.push_back(card1);
            }
            _ => panic!("Unexpected winner {}", winner),
        };

        if player1.is_empty() {
            return (2, compute_score(&player2));
        } else if player2.is_empty() {
            return (1, compute_score(&player1));
        }
    }
}

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
            line.parse::<u8>()
                .unwrap_or_else(|_| panic!("Failed to parse {}", line)),
        )
    });

    let mut player2 = VecDeque::new();
    reader.read_with(|line| {
        if line.len() > 2 {
            return;
        }

        player2.push_back(
            line.parse::<u8>()
                .unwrap_or_else(|_| panic!("Failed to parse {}", line)),
        )
    });

    println!(
        "Basic game score: {}",
        play_basic_game(player1.clone(), player2.clone())
    );

    let (_winner, score) = play_recursive_game(player1, player2);
    println!("Recursive game score: {}", score);
}

#[cfg(test)]
mod tests {
    // use test::Bencher;
}
