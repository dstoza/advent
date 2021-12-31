#![warn(clippy::pedantic)]
#![feature(test)]
#[macro_use]
extern crate num_derive;
extern crate test;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
};

use num_traits::FromPrimitive;

#[derive(Clone, Copy, Debug, Eq, FromPrimitive, Hash, Ord, PartialEq, PartialOrd)]
enum Position {
    Hallway00,
    Hallway01,
    Hallway02,
    Hallway03,
    Hallway04,
    Hallway05,
    Hallway06,
    Hallway07,
    Hallway08,
    Hallway09,
    Hallway10,
    RoomANorth,
    RoomASouth,
    RoomBNorth,
    RoomBSouth,
    RoomCNorth,
    RoomCSouth,
    RoomDNorth,
    RoomDSouth,
}

impl Position {
    fn is_in_hallway(&self) -> bool {
        match *self {
            Position::Hallway00
            | Position::Hallway01
            | Position::Hallway02
            | Position::Hallway03
            | Position::Hallway04
            | Position::Hallway05
            | Position::Hallway06
            | Position::Hallway07
            | Position::Hallway08
            | Position::Hallway09
            | Position::Hallway10 => true,
            _ => false,
        }
    }

    fn is_immediately_outside_room(&self) -> bool {
        match *self {
            Position::Hallway02
            | Position::Hallway04
            | Position::Hallway06
            | Position::Hallway08 => true,
            _ => false,
        }
    }
}

const POSITION_COUNT: usize = Position::RoomDSouth as usize + 1;

fn get_adjacencies() -> HashSet<(Position, Position)> {
    let mut adjacencies = HashSet::new();
    for (a, b) in [
        (Position::Hallway00, Position::Hallway01),
        (Position::Hallway01, Position::Hallway02),
        (Position::Hallway02, Position::Hallway03),
        (Position::Hallway03, Position::Hallway04),
        (Position::Hallway04, Position::Hallway05),
        (Position::Hallway05, Position::Hallway06),
        (Position::Hallway06, Position::Hallway07),
        (Position::Hallway07, Position::Hallway08),
        (Position::Hallway08, Position::Hallway09),
        (Position::Hallway09, Position::Hallway10),
        (Position::Hallway02, Position::RoomANorth),
        (Position::RoomANorth, Position::RoomASouth),
        (Position::Hallway04, Position::RoomBNorth),
        (Position::RoomBNorth, Position::RoomBSouth),
        (Position::Hallway06, Position::RoomCNorth),
        (Position::RoomCNorth, Position::RoomCSouth),
        (Position::Hallway08, Position::RoomDNorth),
        (Position::RoomDNorth, Position::RoomDSouth),
    ] {
        adjacencies.insert((a, b));
        adjacencies.insert((b, a));
    }

    adjacencies
}

fn get_path(next_indices: &[[usize; POSITION_COUNT]], mut u: usize, v: usize) -> Vec<Position> {
    let mut path = Vec::new();
    path.push(FromPrimitive::from_usize(u).unwrap());
    while u != v {
        u = next_indices[u][v];
        path.push(FromPrimitive::from_usize(u).unwrap());
    }
    path
}

fn get_paths(adjacencies: &HashSet<(Position, Position)>) -> Vec<Vec<Position>> {
    // Use Floyd-Warshall to generate minimum distances and a shortest-path tree

    let mut distances = [[usize::MAX / 4; POSITION_COUNT]; POSITION_COUNT];
    let mut next_indices = [[usize::MAX; POSITION_COUNT]; POSITION_COUNT];

    for (u, v) in adjacencies {
        distances[*u as usize][*v as usize] = 1;
        next_indices[*u as usize][*v as usize] = *v as usize;
    }

    for v in 0..POSITION_COUNT {
        distances[v][v] = 0;
        next_indices[v][v] = v;
    }

    for k in 0..POSITION_COUNT {
        for i in 0..POSITION_COUNT {
            for j in 0..POSITION_COUNT {
                if distances[i][j] > distances[i][k] + distances[k][j] {
                    distances[i][j] = distances[i][k] + distances[k][j];
                    next_indices[i][j] = next_indices[i][k];
                }
            }
        }
    }

    // Generate paths using shortest-path tree and domain rules

    let mut paths = Vec::new();

    for from in 0..POSITION_COUNT {
        let from_position: Position = FromPrimitive::from_usize(from).unwrap();

        // It's impossible to start from the space immediately outside a room
        if from_position.is_immediately_outside_room() {
            continue;
        }

        for to in 0..POSITION_COUNT {
            // We must always move to a new position
            if from == to {
                continue;
            }

            let to_position: Position = FromPrimitive::from_usize(to).unwrap();

            // It's impossible to end in a space immediately outside a room
            if to_position.is_immediately_outside_room() {
                continue;
            }

            // If we start from the hallway, we must move into a room
            if from_position.is_in_hallway() && to_position.is_in_hallway() {
                continue;
            }

            paths.push(get_path(&next_indices, from, to));
        }
    }

    paths
}

#[derive(Clone, Copy, FromPrimitive)]
enum Amphipod {
    A1,
    A2,
    B1,
    B2,
    C1,
    C2,
    D1,
    D2,
}

const AMPHIPOD_COUNT: usize = Amphipod::D2 as usize + 1;

type Configuration = [Position; AMPHIPOD_COUNT];
type Cost = usize;
type State = (Cost, Configuration);

fn is_complete(configuration: &Configuration) -> bool {
    match configuration[Amphipod::A1 as usize] {
        Position::RoomANorth | Position::RoomASouth => (),
        _ => return false,
    };
    match configuration[Amphipod::A2 as usize] {
        Position::RoomANorth | Position::RoomASouth => (),
        _ => return false,
    };

    match configuration[Amphipod::B1 as usize] {
        Position::RoomBNorth | Position::RoomBSouth => (),
        _ => return false,
    };
    match configuration[Amphipod::B2 as usize] {
        Position::RoomBNorth | Position::RoomBSouth => (),
        _ => return false,
    };

    match configuration[Amphipod::C1 as usize] {
        Position::RoomCNorth | Position::RoomCSouth => (),
        _ => return false,
    };
    match configuration[Amphipod::C2 as usize] {
        Position::RoomCNorth | Position::RoomCSouth => (),
        _ => return false,
    };

    match configuration[Amphipod::D1 as usize] {
        Position::RoomDNorth | Position::RoomDSouth => (),
        _ => return false,
    };
    match configuration[Amphipod::D2 as usize] {
        Position::RoomDNorth | Position::RoomDSouth => (),
        _ => return false,
    };

    true
}

fn is_valid_destination_for_amphipod(destination: Position, amphipod: Amphipod) -> bool {
    match amphipod {
        Amphipod::A1 | Amphipod::A2 => {
            destination.is_in_hallway()
                || [Position::RoomANorth, Position::RoomASouth].contains(&destination)
        }
        Amphipod::B1 | Amphipod::B2 => {
            destination.is_in_hallway()
                || [Position::RoomBNorth, Position::RoomBSouth].contains(&destination)
        }
        Amphipod::C1 | Amphipod::C2 => {
            destination.is_in_hallway()
                || [Position::RoomCNorth, Position::RoomCSouth].contains(&destination)
        }
        Amphipod::D1 | Amphipod::D2 => {
            destination.is_in_hallway()
                || [Position::RoomDNorth, Position::RoomDSouth].contains(&destination)
        }
    }
}

fn get_step_cost(amphipod: Amphipod) -> usize {
    match amphipod {
        Amphipod::A1 | Amphipod::A2 => 1,
        Amphipod::B1 | Amphipod::B2 => 10,
        Amphipod::C1 | Amphipod::C2 => 100,
        Amphipod::D1 | Amphipod::D2 => 1000,
    }
}

fn get_path_cost(path: &[Position], amphipod: Amphipod) -> usize {
    (path.len() - 1) * get_step_cost(amphipod)
}

fn organize_amphipods(configuration: Configuration) -> Cost {
    let mut visited = HashSet::new();

    let paths = get_paths(&get_adjacencies());

    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0usize, configuration)));
    while let Some(Reverse((cost, configuration))) = queue.pop() {
        // println!("Evaluating cost {} {:?}", cost, configuration);

        if visited.contains(&configuration) {
            continue;
        }

        if cost > 13000 {
            break;
        }

        if is_complete(&configuration) {
            println!("Found {:?} for {}", configuration, cost);
            return cost;
        }

        for path in paths.iter().filter(|path| {
            configuration.contains(&path[0])
                && path[1..]
                    .iter()
                    .all(|position| !configuration.contains(position))
        }) {
            let amphipod: Amphipod = FromPrimitive::from_usize(
                configuration
                    .iter()
                    .position(|position| *position == path[0])
                    .unwrap(),
            )
            .unwrap();

            let destination = path[path.len() - 1];
            if !is_valid_destination_for_amphipod(destination, amphipod) {
                continue;
            }

            let mut new_configuration = configuration.clone();
            new_configuration[amphipod as usize] = path[path.len() - 1];

            // println!(
            //     "Pushing {} {:?}",
            //     cost + get_path_cost(path, amphipod),
            //     new_configuration
            // );
            queue.push(Reverse((
                cost + get_path_cost(path, amphipod),
                new_configuration,
            )))
        }

        visited.insert(configuration);
    }

    0
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    // organize_amphipods([
    //     Position::RoomASouth,
    //     Position::RoomDSouth,
    //     Position::RoomANorth,
    //     Position::RoomCNorth,
    //     Position::RoomBNorth,
    //     Position::RoomCSouth,
    //     Position::RoomBSouth,
    //     Position::RoomDNorth,
    // ]);
    organize_amphipods([
        Position::RoomANorth,
        Position::RoomCSouth,
        Position::RoomASouth,
        Position::RoomCNorth,
        Position::RoomBSouth,
        Position::RoomDSouth,
        Position::RoomBNorth,
        Position::RoomDNorth,
    ]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test() {}

    // #[bench]
    // fn bench_input(b: &mut Bencher) {
    //     let file = File::open("input.txt").unwrap();
    //     let reader = BufReader::new(file);
    //     let lines: Vec<_> = reader.lines().map(|line| line.unwrap()).collect();

    //     b.iter(|| {
    //         let (algorithm, pixels) = parse_input(lines.clone().into_iter());
    //         assert_eq!(run_iterations(&algorithm, pixels, 50), 12333);
    //     });
    // }
}
