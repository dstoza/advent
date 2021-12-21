use std::{collections::HashMap, mem::swap};

fn roll_die(die: &mut i32) -> i32 {
    let roll = *die;
    *die = (*die % 100) + 1;
    roll
}

fn play_game() -> i32 {
    let mut die = 1;
    let mut die_rolls = 0;

    let mut positions = [10, 7];
    let mut scores = [0, 0];

    let mut current_player = 0;
    loop {
        let roll = roll_die(&mut die) + roll_die(&mut die) + roll_die(&mut die);
        die_rolls += 3;
        positions[current_player] = (positions[current_player] + roll - 1) % 10 + 1;
        scores[current_player] += positions[current_player];
        if scores[current_player] >= 1000 {
            return scores[1 - current_player] * die_rolls;
        }

        current_player = 1 - current_player;
    }
}

const DICE_ROLLS: [(u8, usize); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];

#[derive(Clone, Eq, Hash, PartialEq)]
struct Universe {
    position: [u8; 2],
    score: [u8; 2],
}

fn simulate_multiverse() -> [usize; 2] {
    let mut wins = [0usize; 2];

    let mut initial = HashMap::from([(
        Universe {
            position: [10, 7],
            score: [0, 0],
        },
        1,
    )]);
    let mut result = HashMap::new();

    let mut current_player = 0;
    while !initial.is_empty() {
        for (universe, count) in &initial {
            for (roll_value, roll_count) in DICE_ROLLS {
                let mut new_universe = universe.clone();
                new_universe.position[current_player] =
                    (new_universe.position[current_player] + roll_value - 1) % 10 + 1;
                new_universe.score[current_player] += new_universe.position[current_player];
                if new_universe.score[current_player] >= 21 {
                    wins[current_player] += roll_count * count;
                } else {
                    result
                        .entry(new_universe)
                        .and_modify(|entry| *entry += count * roll_count)
                        .or_insert(count * roll_count);
                }
            }
        }

        swap(&mut initial, &mut result);
        result.clear();

        current_player = 1 - current_player;
    }

    wins
}

fn main() {
    println!("One game: {}", play_game());
    println!("Multiverse: {:?}", simulate_multiverse());
}
