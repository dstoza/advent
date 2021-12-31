#![warn(clippy::pedantic)]
#![feature(test)]
#[macro_use]
extern crate num_derive;
extern crate test;

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
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
    RoomA1,
    RoomA2,
    RoomA3,
    RoomA4,
    RoomB1,
    RoomB2,
    RoomB3,
    RoomB4,
    RoomC1,
    RoomC2,
    RoomC3,
    RoomC4,
    RoomD1,
    RoomD2,
    RoomD3,
    RoomD4,
}

const ROOM_A: [Position; 4] = [
    Position::RoomA1,
    Position::RoomA2,
    Position::RoomA3,
    Position::RoomA4,
];
const ROOM_B: [Position; 4] = [
    Position::RoomB1,
    Position::RoomB2,
    Position::RoomB3,
    Position::RoomB4,
];
const ROOM_C: [Position; 4] = [
    Position::RoomC1,
    Position::RoomC2,
    Position::RoomC3,
    Position::RoomC4,
];
const ROOM_D: [Position; 4] = [
    Position::RoomD1,
    Position::RoomD2,
    Position::RoomD3,
    Position::RoomD4,
];

impl Position {
    fn get_room(self) -> Option<[Position; 4]> {
        if ROOM_A.contains(&self) {
            Some(ROOM_A)
        } else if ROOM_B.contains(&self) {
            Some(ROOM_B)
        } else if ROOM_C.contains(&self) {
            Some(ROOM_C)
        } else if ROOM_D.contains(&self) {
            Some(ROOM_D)
        } else {
            None
        }
    }

    fn is_in_hallway(self) -> bool {
        // matches!(
        //     self,
        //     Position::Hallway00
        //         | Position::Hallway01
        //         | Position::Hallway02
        //         | Position::Hallway03
        //         | Position::Hallway04
        //         | Position::Hallway05
        //         | Position::Hallway06
        //         | Position::Hallway07
        //         | Position::Hallway08
        //         | Position::Hallway09
        //         | Position::Hallway10
        // )
        self as usize <= Position::Hallway10 as usize
    }

    fn is_immediately_outside_room(self) -> bool {
        matches!(
            self,
            Position::Hallway02 | Position::Hallway04 | Position::Hallway06 | Position::Hallway08
        )
    }
}

const POSITION_COUNT: usize = Position::RoomD4 as usize + 1;

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
        (Position::Hallway02, Position::RoomA1),
        (Position::RoomA1, Position::RoomA2),
        (Position::RoomA2, Position::RoomA3),
        (Position::RoomA3, Position::RoomA4),
        (Position::Hallway04, Position::RoomB1),
        (Position::RoomB1, Position::RoomB2),
        (Position::RoomB2, Position::RoomB3),
        (Position::RoomB3, Position::RoomB4),
        (Position::Hallway06, Position::RoomC1),
        (Position::RoomC1, Position::RoomC2),
        (Position::RoomC2, Position::RoomC3),
        (Position::RoomC3, Position::RoomC4),
        (Position::Hallway08, Position::RoomD1),
        (Position::RoomD1, Position::RoomD2),
        (Position::RoomD2, Position::RoomD3),
        (Position::RoomD3, Position::RoomD4),
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

            // If we start in a room, we must leave that room
            if !from_position.is_in_hallway()
                && from_position.get_room().unwrap().contains(&to_position)
            {
                continue;
            }

            paths.push(get_path(&next_indices, from, to));
        }
    }

    println!("Generated {} paths", paths.len());

    paths
}

#[derive(Clone, Copy, Eq, FromPrimitive, PartialEq)]
enum Amphipod {
    A1,
    A2,
    A3,
    A4,
    B1,
    B2,
    B3,
    B4,
    C1,
    C2,
    C3,
    C4,
    D1,
    D2,
    D3,
    D4,
}

impl Amphipod {
    fn get_room(self) -> [Position; 4] {
        match self {
            Amphipod::A1 | Amphipod::A2 | Amphipod::A3 | Amphipod::A4 => ROOM_A,
            Amphipod::B1 | Amphipod::B2 | Amphipod::B3 | Amphipod::B4 => ROOM_B,
            Amphipod::C1 | Amphipod::C2 | Amphipod::C3 | Amphipod::C4 => ROOM_C,
            Amphipod::D1 | Amphipod::D2 | Amphipod::D3 | Amphipod::D4 => ROOM_D,
        }
    }

    fn get_siblings(self) -> [Amphipod; 4] {
        match self {
            Amphipod::A1 | Amphipod::A2 | Amphipod::A3 | Amphipod::A4 => {
                [Amphipod::A1, Amphipod::A2, Amphipod::A3, Amphipod::A4]
            }
            Amphipod::B1 | Amphipod::B2 | Amphipod::B3 | Amphipod::B4 => {
                [Amphipod::B1, Amphipod::B2, Amphipod::B3, Amphipod::B4]
            }
            Amphipod::C1 | Amphipod::C2 | Amphipod::C3 | Amphipod::C4 => {
                [Amphipod::C1, Amphipod::C2, Amphipod::C3, Amphipod::C4]
            }
            Amphipod::D1 | Amphipod::D2 | Amphipod::D3 | Amphipod::D4 => {
                [Amphipod::D1, Amphipod::D2, Amphipod::D3, Amphipod::D4]
            }
        }
    }

    fn get_step_cost(self) -> usize {
        match self {
            Amphipod::A1 | Amphipod::A2 | Amphipod::A3 | Amphipod::A4 => 1,
            Amphipod::B1 | Amphipod::B2 | Amphipod::B3 | Amphipod::B4 => 10,
            Amphipod::C1 | Amphipod::C2 | Amphipod::C3 | Amphipod::C4 => 100,
            Amphipod::D1 | Amphipod::D2 | Amphipod::D3 | Amphipod::D4 => 1000,
        }
    }
}

const AMPHIPOD_COUNT: usize = Amphipod::D4 as usize + 1;

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
    amphipod.get_room()[0]
}

fn get_estimated_completion_cost(configuration: Configuration, paths: &[Vec<Position>]) -> usize {
    configuration
        .iter()
        .enumerate()
        .map(|(index, position)| {
            let amphipod: Amphipod = FromPrimitive::from_usize(index).unwrap();
            if !position.is_in_hallway() && is_valid_destination_for_amphipod(*position, amphipod) {
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
    if configuration[Amphipod::A1 as usize..=Amphipod::A4 as usize].contains(&Position::RoomA1)
        && !configuration.contains(&Position::RoomA2)
        && !configuration.contains(&Position::RoomA3)
        && !configuration.contains(&Position::RoomA4)
    {
        return false;
    }

    if configuration[Amphipod::A1 as usize..=Amphipod::A4 as usize].contains(&Position::RoomA2)
        && !configuration.contains(&Position::RoomA3)
        && !configuration.contains(&Position::RoomA4)
    {
        return false;
    }

    if configuration[Amphipod::A1 as usize..=Amphipod::A4 as usize].contains(&Position::RoomA3)
        && !configuration.contains(&Position::RoomA4)
    {
        return false;
    }

    if configuration[Amphipod::B1 as usize..=Amphipod::B4 as usize].contains(&Position::RoomB1)
        && !configuration.contains(&Position::RoomB2)
        && !configuration.contains(&Position::RoomB3)
        && !configuration.contains(&Position::RoomB4)
    {
        return false;
    }

    if configuration[Amphipod::B1 as usize..=Amphipod::B4 as usize].contains(&Position::RoomB2)
        && !configuration.contains(&Position::RoomB3)
        && !configuration.contains(&Position::RoomB4)
    {
        return false;
    }

    if configuration[Amphipod::B1 as usize..=Amphipod::B4 as usize].contains(&Position::RoomB3)
        && !configuration.contains(&Position::RoomB4)
    {
        return false;
    }

    if configuration[Amphipod::C1 as usize..=Amphipod::C4 as usize].contains(&Position::RoomC1)
        && !configuration.contains(&Position::RoomC2)
        && !configuration.contains(&Position::RoomC3)
        && !configuration.contains(&Position::RoomC4)
    {
        return false;
    }

    if configuration[Amphipod::C1 as usize..=Amphipod::C4 as usize].contains(&Position::RoomC2)
        && !configuration.contains(&Position::RoomC3)
        && !configuration.contains(&Position::RoomC4)
    {
        return false;
    }

    if configuration[Amphipod::C1 as usize..=Amphipod::C4 as usize].contains(&Position::RoomC3)
        && !configuration.contains(&Position::RoomC4)
    {
        return false;
    }

    if configuration[Amphipod::D1 as usize..=Amphipod::D4 as usize].contains(&Position::RoomD1)
        && !configuration.contains(&Position::RoomD2)
        && !configuration.contains(&Position::RoomD3)
        && !configuration.contains(&Position::RoomD4)
    {
        return false;
    }

    if configuration[Amphipod::D1 as usize..=Amphipod::D4 as usize].contains(&Position::RoomD2)
        && !configuration.contains(&Position::RoomD3)
        && !configuration.contains(&Position::RoomD4)
    {
        return false;
    }

    if configuration[Amphipod::D1 as usize..=Amphipod::D4 as usize].contains(&Position::RoomD3)
        && !configuration.contains(&Position::RoomD4)
    {
        return false;
    }

    true
}

fn position_is_complete(configuration: Configuration, position: Position) -> bool {
    let a_positions = &configuration[Amphipod::A1 as usize..=Amphipod::A4 as usize];
    let b_positions = &configuration[Amphipod::B1 as usize..=Amphipod::B4 as usize];
    let c_positions = &configuration[Amphipod::C1 as usize..=Amphipod::C4 as usize];
    let d_positions = &configuration[Amphipod::D1 as usize..=Amphipod::D4 as usize];

    match position {
        Position::RoomA1 => ROOM_A.iter().all(|p| a_positions.contains(p)),
        Position::RoomA2 => ROOM_A[1..].iter().all(|p| a_positions.contains(p)),
        Position::RoomA3 => ROOM_A[2..].iter().all(|p| a_positions.contains(p)),
        Position::RoomA4 => ROOM_A[3..].iter().all(|p| a_positions.contains(p)),
        Position::RoomB1 => ROOM_B.iter().all(|p| b_positions.contains(p)),
        Position::RoomB2 => ROOM_B[1..].iter().all(|p| b_positions.contains(p)),
        Position::RoomB3 => ROOM_B[2..].iter().all(|p| b_positions.contains(p)),
        Position::RoomB4 => ROOM_B[3..].iter().all(|p| b_positions.contains(p)),
        Position::RoomC1 => ROOM_C.iter().all(|p| c_positions.contains(p)),
        Position::RoomC2 => ROOM_C[1..].iter().all(|p| c_positions.contains(p)),
        Position::RoomC3 => ROOM_C[2..].iter().all(|p| c_positions.contains(p)),
        Position::RoomC4 => ROOM_C[3..].iter().all(|p| c_positions.contains(p)),
        Position::RoomD1 => ROOM_D.iter().all(|p| d_positions.contains(p)),
        Position::RoomD2 => ROOM_D[1..].iter().all(|p| d_positions.contains(p)),
        Position::RoomD3 => ROOM_D[2..].iter().all(|p| d_positions.contains(p)),
        Position::RoomD4 => ROOM_D[3..].iter().all(|p| d_positions.contains(p)),
        _ => false,
    }
}

fn organize_amphipods(configuration: Configuration) -> Cost {
    let mut visited = HashSet::new();

    let paths = get_paths(&get_adjacencies());

    let mut popped = 0usize;
    let mut skipped_because_visited = 0usize;
    let mut paths_visited = 0usize;
    let mut skipped_complete = 0usize;
    let mut skipped_because_of_invalid_destination = 0usize;
    let mut skipped_new_visited_configuration = 0usize;
    let mut skipped_because_of_invalid_configuration = 0usize;
    let mut skipped_because_of_impure_room = 0usize;
    let mut max_cost = 0;

    let mut queue = BinaryHeap::new();
    queue.push(Reverse((
        get_estimated_completion_cost(configuration, &paths),
        0usize,
        configuration,
    )));
    while let Some(Reverse((estimated_cost, actual_cost, configuration))) = queue.pop() {
        popped += 1;
        if visited.contains(&configuration) {
            skipped_because_visited += 1;
            continue;
        }

        if is_complete(configuration) {
            println!("Found {:?} for {}", configuration, actual_cost);
            return actual_cost;
        }

        if estimated_cost > max_cost {
            println!(
                "max {} popped {} visited {} paths {} complete {} dest {} vdest {} config {} impure {}",
                estimated_cost,
                popped,
                skipped_because_visited,
                paths_visited,
                skipped_complete,
                skipped_because_of_invalid_destination,
                skipped_new_visited_configuration,
                skipped_because_of_invalid_configuration,
                skipped_because_of_impure_room
            );
            max_cost = estimated_cost;
        }

        for path in paths.iter().filter(|path| {
            configuration.contains(&path[0])
                && path[1..]
                    .iter()
                    .all(|position| !configuration.contains(position))
        }) {
            paths_visited += 1;

            if position_is_complete(configuration, path[0]) {
                skipped_complete += 1;
                continue;
            }

            let amphipod: Amphipod = FromPrimitive::from_usize(
                configuration
                    .iter()
                    .position(|position| *position == path[0])
                    .unwrap(),
            )
            .unwrap();

            let destination = path[path.len() - 1];
            if !is_valid_destination_for_amphipod(destination, amphipod) {
                skipped_because_of_invalid_destination += 1;
                continue;
            }

            let mut new_configuration = configuration;
            new_configuration[amphipod as usize] = path[path.len() - 1];

            if visited.contains(&new_configuration) {
                skipped_new_visited_configuration += 1;
                continue;
            }

            if !is_valid_configuration(new_configuration) {
                skipped_because_of_invalid_configuration += 1;
                continue;
            }

            if !destination.is_in_hallway()
                && !room_is_pure_for_amphipod(new_configuration, amphipod)
            {
                skipped_because_of_impure_room += 1;
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
    // organize_amphipods([
    //     Position::RoomANorth,
    //     Position::RoomCSouth,
    //     Position::RoomASouth,
    //     Position::RoomCNorth,
    //     Position::RoomBSouth,
    //     Position::RoomDSouth,
    //     Position::RoomBNorth,
    //     Position::RoomDNorth,
    // ]);
    // organize_amphipods([
    //     Position::RoomD2,
    //     Position::RoomC3,
    //     Position::RoomA4,
    //     Position::RoomD4,
    //     Position::RoomA1,
    //     Position::RoomC1,
    //     Position::RoomC2,
    //     Position::RoomB3,
    //     Position::RoomB1,
    //     Position::RoomB2,
    //     Position::RoomD3,
    //     Position::RoomC4,
    //     Position::RoomD1,
    //     Position::RoomA2,
    //     Position::RoomA3,
    //     Position::RoomB4,
    // ]);
    organize_amphipods([
        Position::RoomA1,
        Position::RoomC4,
        Position::RoomD2,
        Position::RoomC3,
        Position::RoomA4,
        Position::RoomC1,
        Position::RoomC2,
        Position::RoomB3,
        Position::RoomB4,
        Position::RoomD4,
        Position::RoomB2,
        Position::RoomD3,
        Position::RoomB1,
        Position::RoomD1,
        Position::RoomA2,
        Position::RoomA3,
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
