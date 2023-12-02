#![warn(clippy::pedantic)]
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let games = reader
        .lines()
        .map(std::result::Result::unwrap)
        .map(|line| {
            let rounds = line
                .split(": ")
                .nth(1)
                .unwrap()
                .split("; ")
                .map(|round| {
                    round
                        .split(", ")
                        .map(|turn| {
                            let mut split = turn.split(' ');
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

    let initial = HashMap::from([
        (String::from("red"), 12),
        (String::from("green"), 13),
        (String::from("blue"), 14),
    ]);

    let possible_sum: usize = games
        .iter()
        .enumerate()
        .map(|(index, game)| {
            (index + 1)
                * usize::from(
                    game.iter()
                        .flat_map(HashMap::iter)
                        .all(|(color, count)| *count <= initial[color]),
                )
        })
        .sum();

    println!("{possible_sum}");

    let power_sum: i32 = games
        .iter()
        .map(|game| {
            game.iter().fold(HashMap::new(), |mut minimums, round| {
                for (color, count) in round {
                    minimums
                        .entry(color)
                        .and_modify(|value: &mut i32| *value = (*value).max(*count))
                        .or_insert(*count);
                }
                minimums
            })
        })
        .map(|minimums| minimums.values().copied().product::<i32>())
        .sum();

    println!("{power_sum}");
}
