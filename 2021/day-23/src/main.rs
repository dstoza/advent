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

const ROOM_A: [Position; 2] = [Position::RoomANorth, Position::RoomASouth];
const ROOM_B: [Position; 2] = [Position::RoomBNorth, Position::RoomBSouth];
const ROOM_C: [Position; 2] = [Position::RoomCNorth, Position::RoomCSouth];
const ROOM_D: [Position; 2] = [Position::RoomDNorth, Position::RoomDSouth];

impl Position {
    fn is_in_hallway(self) -> bool {
        matches!(
            self,
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
                | Position::Hallway10
        )
    }

    fn is_immediately_outside_room(self) -> bool {
        matches!(
            self,
            Position::Hallway02 | Position::Hallway04 | Position::Hallway06 | Position::Hallway08
        )
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
    let mut path = vec![FromPrimitive::from_usize(u).unwrap()];
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

#[derive(Clone, Copy, Eq, FromPrimitive, PartialEq)]
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

impl Amphipod {
    fn get_room(self) -> [Position; 2] {
        match self {
            Amphipod::A1 | Amphipod::A2 => ROOM_A,
            Amphipod::B1 | Amphipod::B2 => ROOM_B,
            Amphipod::C1 | Amphipod::C2 => ROOM_C,
            Amphipod::D1 | Amphipod::D2 => ROOM_D,
        }
    }

    fn get_siblings(self) -> [Amphipod; 2] {
        match self {
            Amphipod::A1 | Amphipod::A2 => [Amphipod::A1, Amphipod::A2],
            Amphipod::B1 | Amphipod::B2 => [Amphipod::B1, Amphipod::B2],
            Amphipod::C1 | Amphipod::C2 => [Amphipod::C1, Amphipod::C2],
            Amphipod::D1 | Amphipod::D2 => [Amphipod::D1, Amphipod::D2],
        }
    }

    fn get_step_cost(self) -> usize {
        match self {
            Amphipod::A1 | Amphipod::A2 => 1,
            Amphipod::B1 | Amphipod::B2 => 10,
            Amphipod::C1 | Amphipod::C2 => 100,
            Amphipod::D1 | Amphipod::D2 => 1000,
        }
    }
}

const AMPHIPOD_COUNT: usize = Amphipod::D2 as usize + 1;

type Configuration = [Position; AMPHIPOD_COUNT];
type Cost = usize;

fn is_complete(configuration: Configuration) -> bool {
    configuration.iter().enumerate().all(|(index, position)| {
        let amphipod: Amphipod = FromPrimitive::from_usize(index).unwrap();
        amphipod.get_room().contains(position)
    })
}

fn is_valid_destination_for_amphipod(destination: Position, amphipod: Amphipod) -> bool {
    destination.is_in_hallway() || amphipod.get_room().contains(&destination)
}

fn room_is_pure_for_amphipod(configuration: Configuration, amphipod: Amphipod) -> bool {
    configuration
        .iter()
        .enumerate()
        .filter(|(_index, position)| amphipod.get_room().contains(position))
        .all(|(index, _position)| {
            amphipod
                .get_siblings()
                .contains(&FromPrimitive::from_usize(index).unwrap())
        })
}

fn get_path_cost(path: &[Position], amphipod: Amphipod) -> usize {
    (path.len() - 1) * amphipod.get_step_cost()
}

fn get_closest_destination_for_amphipod(amphipod: Amphipod) -> Position {
    match amphipod {
        Amphipod::A1 | Amphipod::A2 => Position::RoomANorth,
        Amphipod::B1 | Amphipod::B2 => Position::RoomBNorth,
        Amphipod::C1 | Amphipod::C2 => Position::RoomCNorth,
        Amphipod::D1 | Amphipod::D2 => Position::RoomDNorth,
    }
}

fn get_estimated_completion_cost(configuration: Configuration, paths: &[Vec<Position>]) -> usize {
    configuration
        .iter()
        .enumerate()
        .map(|(index, position)| {
            let amphipod: Amphipod = FromPrimitive::from_usize(index).unwrap();
            if is_valid_destination_for_amphipod(*position, amphipod) {
                return 0;
            }

            let closest_destination = get_closest_destination_for_amphipod(amphipod);
            get_path_cost(
                paths
                    .iter()
                    .find(|path| {
                        path[0] == *position && path[path.len() - 1] == closest_destination
                    })
                    .unwrap(),
                amphipod,
            )
        })
        .sum()
}

fn is_valid_configuration(configuration: Configuration) -> bool {
    if configuration[Amphipod::A1 as usize..=Amphipod::A2 as usize].contains(&Position::RoomANorth)
        && !configuration.contains(&Position::RoomASouth)
    {
        return false;
    }

    if configuration[Amphipod::B1 as usize..=Amphipod::B2 as usize].contains(&Position::RoomBNorth)
        && !configuration.contains(&Position::RoomBSouth)
    {
        return false;
    }

    if configuration[Amphipod::C1 as usize..=Amphipod::C2 as usize].contains(&Position::RoomCNorth)
        && !configuration.contains(&Position::RoomCSouth)
    {
        return false;
    }

    if configuration[Amphipod::D1 as usize..=Amphipod::D2 as usize].contains(&Position::RoomDNorth)
        && !configuration.contains(&Position::RoomDSouth)
    {
        return false;
    }

    true
}

fn organize_amphipods(configuration: Configuration) -> Cost {
    let mut visited = HashSet::new();

    let paths = get_paths(&get_adjacencies());

    let mut queue = BinaryHeap::new();
    queue.push(Reverse((
        get_estimated_completion_cost(configuration, &paths),
        0usize,
        configuration,
    )));
    while let Some(Reverse((_estimated_cost, actual_cost, configuration))) = queue.pop() {
        if visited.contains(&configuration) {
            continue;
        }

        if actual_cost > 13000 {
            println!("Breaking");
            break;
        }

        if is_complete(configuration) {
            println!("Found {:?} for {}", configuration, actual_cost);
            return actual_cost;
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

            let mut new_configuration = configuration;
            new_configuration[amphipod as usize] = path[path.len() - 1];

            if !is_valid_configuration(new_configuration) {
                continue;
            }

            if !destination.is_in_hallway()
                && !room_is_pure_for_amphipod(new_configuration, amphipod)
            {
                continue;
            }

            let actual_cost = actual_cost + get_path_cost(path, amphipod);

            queue.push(Reverse((
                actual_cost + get_estimated_completion_cost(new_configuration, &paths),
                actual_cost,
                new_configuration,
            )));
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
