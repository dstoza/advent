#![warn(clippy::pedantic)]
use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
};

enum Strategy {
    Choice,
    Outcome,
}

#[derive(Clone, Copy)]
enum Choice {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Copy)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

fn get_score_for_round(opponent: Choice, mine: Choice) -> i32 {
    match opponent {
        Choice::Rock => match mine {
            Choice::Rock => 3,
            Choice::Paper => 6,
            Choice::Scissors => 0,
        },
        Choice::Paper => match mine {
            Choice::Rock => 0,
            Choice::Paper => 3,
            Choice::Scissors => 6,
        },
        Choice::Scissors => match mine {
            Choice::Rock => 6,
            Choice::Paper => 0,
            Choice::Scissors => 3,
        },
    }
}

fn get_choice_for_outcome(opponent: Choice, outcome: Outcome) -> Choice {
    match opponent {
        Choice::Rock => match outcome {
            Outcome::Lose => Choice::Scissors,
            Outcome::Draw => Choice::Rock,
            Outcome::Win => Choice::Paper,
        },
        Choice::Paper => match outcome {
            Outcome::Lose => Choice::Rock,
            Outcome::Draw => Choice::Paper,
            Outcome::Win => Choice::Scissors,
        },
        Choice::Scissors => match outcome {
            Outcome::Lose => Choice::Paper,
            Outcome::Draw => Choice::Scissors,
            Outcome::Win => Choice::Rock,
        },
    }
}

fn main() {
    let filename = std::env::args().nth(1).expect("Filename not found");
    let strategy = match std::env::args()
        .nth(2)
        .expect("Strategy not found")
        .as_str()
    {
        "choice" => Strategy::Choice,
        "outcome" => Strategy::Outcome,
        _ => unreachable!("Unknown strategy. Valid options are 'choice' and 'outcome'"),
    };

    let file =
        File::open(&filename).unwrap_or_else(|_| panic!("Couldn't open {}", filename.as_str()));
    let reader = BufReader::new(file);
    let mut total = 0;
    for line in reader.lines().map(std::result::Result::unwrap) {
        let mut choices = line.split(' ');
        let opponent = match choices.next().unwrap() {
            "A" => Choice::Rock,
            "B" => Choice::Paper,
            "C" => Choice::Scissors,
            _ => unreachable!(),
        };

        let mine = match strategy {
            Strategy::Choice => match choices.next().unwrap() {
                "X" => Choice::Rock,
                "Y" => Choice::Paper,
                "Z" => Choice::Scissors,
                _ => unreachable!(),
            },
            Strategy::Outcome => {
                let outcome = match choices.next().unwrap() {
                    "X" => Outcome::Lose,
                    "Y" => Outcome::Draw,
                    "Z" => Outcome::Win,
                    _ => unreachable!(),
                };
                get_choice_for_outcome(opponent, outcome)
            }
        };

        let score = match mine {
            Choice::Rock => 1,
            Choice::Paper => 2,
            Choice::Scissors => 3,
        } + get_score_for_round(opponent, mine);
        total += score;
    }

    println!("Total score: {total}");
}
