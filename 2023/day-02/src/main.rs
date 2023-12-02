#![warn(clippy::pedantic)]
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
};

fn get_initial() -> HashMap<String, i32> {
    HashMap::from([
        (String::from("red"), 12),
        (String::from("green"), 13),
        (String::from("blue"), 14),
    ])
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let games = reader
        .lines()
        .map(std::result::Result::unwrap)
        .map(|line| {
            let split = line.split(": ");
            let rounds = split
                .skip(1)
                .next()
                .unwrap()
                .split("; ")
                .map(|round| {
                    round
                        .split(", ")
                        .map(|turn| {
                            let mut split = turn.split(" ");
                            let count: i32 = split.next().unwrap().parse().unwrap();
                            let color = String::from(split.next().unwrap());
                            (color, count)
                        })
                        .collect::<HashMap<_, _>>()
                })
                .collect::<Vec<_>>();

            rounds
        })
        .collect::<Vec<_>>();

    let initial = get_initial();
    let mut sum = 0;
    for (index, game) in games.iter().enumerate() {
        let mut possible = true;
        for round in game {
            for (color, count) in round {
                if *count > initial[color] {
                    possible = false;
                }
            }
        }
        if possible {
            sum += index + 1;
        }
    }

    println!("{sum}");
}
