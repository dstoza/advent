#![feature(iter_intersperse)]
#![warn(clippy::pedantic)]

use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
};

#[derive(Clone, Copy, Debug)]
enum Turn {
    Right = 1,
    Left = 3,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    East,
    South,
    West,
    North,
}

impl Direction {
    fn turn(self, turn: Turn) -> Self {
        match (self as u8 + turn as u8) % 4 {
            0 => Self::East,
            1 => Self::South,
            2 => Self::West,
            3 => Self::North,
            _ => unreachable!(),
        }
    }

    fn get_opposite(self) -> Self {
        match self {
            Self::East => Self::West,
            Self::South => Self::North,
            Self::West => Self::East,
            Self::North => Self::South,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Position {
    row: usize,
    column: usize,
}

impl Position {
    fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }

    fn step(self, direction: Direction) -> Self {
        match direction {
            Direction::East => Self {
                row: self.row,
                column: self.column + 1,
            },
            Direction::South => Self {
                row: self.row + 1,
                column: self.column,
            },
            Direction::West => Self {
                row: self.row,
                column: self.column - 1,
            },
            Direction::North => Self {
                row: self.row - 1,
                column: self.column,
            },
        }
    }

    fn next(
        self,
        board: &[Vec<u8>],
        wrap_cache: &mut impl WrapCache,
        direction: Direction,
    ) -> Option<(Self, Direction)> {
        let next_coordinates = self.step(direction);

        match board[next_coordinates.row]
            .get(next_coordinates.column)
            .unwrap()
        {
            b'#' => None,
            b'.' => Some((next_coordinates, direction)),
            b' ' => wrap_cache.next(board, self, direction),
            _ => unimplemented!(),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Face {
    Up,
    Down,
    Front,
    Back,
    Left,
    Right,
}

impl Face {
    fn get_neighbors(self) -> [Face; 4] {
        match self {
            Face::Up => [Face::Back, Face::Right, Face::Front, Face::Left],
            Face::Down => [Face::Front, Face::Right, Face::Back, Face::Left],
            Face::Front => [Face::Up, Face::Right, Face::Down, Face::Left],
            Face::Back => [Face::Up, Face::Left, Face::Down, Face::Right],
            Face::Left => [Face::Up, Face::Front, Face::Down, Face::Back],
            Face::Right => [Face::Up, Face::Back, Face::Down, Face::Front],
        }
    }

    fn get_opposite(self) -> Face {
        match self {
            Face::Up => Face::Down,
            Face::Down => Face::Up,
            Face::Front => Face::Back,
            Face::Back => Face::Front,
            Face::Left => Face::Right,
            Face::Right => Face::Left,
        }
    }
}

#[derive(Debug)]
struct OrientedFace {
    face: Face,
    north_toward: Face,
    coordinates: Position,
}

impl OrientedFace {
    fn new(face: Face, north_toward: Face, coordinates: Position) -> Self {
        Self {
            face,
            north_toward,
            coordinates,
        }
    }
}

fn get_northwest_of_face(coordinates: Position, face_dimension: usize) -> Position {
    Position::new(
        1 + coordinates.row * face_dimension,
        1 + coordinates.column * face_dimension,
    )
}

fn orient_neighbors(neighbors: &mut [Face; 4], north_toward: Face) {
    let north_neighbor_position = neighbors.iter().position(|n| *n == north_toward).unwrap();
    neighbors.rotate_left(north_neighbor_position);
}

fn neighbor_exists(
    coordinates: Position,
    direction: Direction,
    board: &[Vec<u8>],
    face_dimension: usize,
) -> bool {
    let neighbor_coordinates = coordinates.step(direction);
    let neighbor_northwest = get_northwest_of_face(neighbor_coordinates, face_dimension);
    board[neighbor_northwest.row][neighbor_northwest.column] != b' '
}

fn get_oriented_faces(board: &[Vec<u8>], face_dimension: usize) -> Vec<OrientedFace> {
    let board_face_width = (board[0].len() - 2) / face_dimension;
    let board_face_height = (board.len() - 2) / face_dimension;

    // Find top face
    let mut column = 1;
    while board[1][column] == b' ' {
        column += face_dimension;
    }

    let mut visited = HashSet::from([Face::Up]);
    let mut to_process = vec![OrientedFace::new(
        Face::Up,
        Face::Back,
        Position::new(0, (column - 1) / face_dimension),
    )];
    let mut oriented_faces = Vec::new();
    while let Some(current_face) = to_process.pop() {
        // Orient neighbors to north
        let mut neighbors = current_face.face.get_neighbors();
        orient_neighbors(&mut neighbors, current_face.north_toward);

        // North neighbor
        if !visited.contains(&neighbors[0])
            && current_face.coordinates.row > 0
            && neighbor_exists(
                current_face.coordinates,
                Direction::North,
                board,
                face_dimension,
            )
        {
            to_process.push(OrientedFace::new(
                neighbors[0],
                current_face.face.get_opposite(),
                current_face.coordinates.step(Direction::North),
            ));
            visited.insert(neighbors[0]);
        }

        // East neighbor
        if !visited.contains(&neighbors[1])
            && current_face.coordinates.column < board_face_width - 1
            && neighbor_exists(
                current_face.coordinates,
                Direction::East,
                board,
                face_dimension,
            )
        {
            to_process.push(OrientedFace::new(
                neighbors[1],
                current_face.north_toward,
                current_face.coordinates.step(Direction::East),
            ));
            visited.insert(neighbors[1]);
        }

        // South neighbor
        if !visited.contains(&neighbors[2])
            && current_face.coordinates.row < board_face_height - 1
            && neighbor_exists(
                current_face.coordinates,
                Direction::South,
                board,
                face_dimension,
            )
        {
            to_process.push(OrientedFace::new(
                neighbors[2],
                current_face.face,
                current_face.coordinates.step(Direction::South),
            ));
            visited.insert(neighbors[2]);
        }

        // West neighbor
        if !visited.contains(&neighbors[3])
            && current_face.coordinates.column > 0
            && neighbor_exists(
                current_face.coordinates,
                Direction::West,
                board,
                face_dimension,
            )
        {
            to_process.push(OrientedFace::new(
                neighbors[3],
                current_face.north_toward,
                current_face.coordinates.step(Direction::West),
            ));
            visited.insert(neighbors[3]);
        }

        oriented_faces.push(current_face);
    }

    oriented_faces
}

trait WrapCache {
    fn next(
        &mut self,
        board: &[Vec<u8>],
        position: Position,
        direction: Direction,
    ) -> Option<(Position, Direction)>;
}

struct FlatWrapCache {
    cache: HashMap<(Position, Direction), Option<(Position, Direction)>>,
}

impl FlatWrapCache {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }
}

impl WrapCache for FlatWrapCache {
    fn next(
        &mut self,
        board: &[Vec<u8>],
        position: Position,
        direction: Direction,
    ) -> Option<(Position, Direction)> {
        if let Some(hit) = self.cache.get(&(position, direction)) {
            return *hit;
        }

        let mut search = position.step(direction.get_opposite());
        while board[search.row][search.column] != b' ' {
            search = search.step(direction.get_opposite());
        }
        search = search.step(direction);

        let result = match board[search.row].get(search.column).unwrap() {
            b'#' => None,
            b'.' => Some((search, direction)),
            _ => unimplemented!(),
        };

        self.cache.insert((position, direction), result);
        result
    }
}

struct CubeWrapCache {
    cache: HashMap<(Position, Direction), Option<(Position, Direction)>>,
    oriented_faces: Vec<OrientedFace>,
    face_dimension: usize,
}

impl CubeWrapCache {
    fn new(oriented_faces: Vec<OrientedFace>, face_dimension: usize) -> Self {
        Self {
            cache: HashMap::new(),
            oriented_faces,
            face_dimension,
        }
    }
}

fn map_face_relative_position(
    position: Position,
    from: Direction,
    to: Direction,
    face_dimension: usize,
) -> Position {
    match (from, to) {
        (Direction::North, Direction::North) => {
            Position::new(0, face_dimension - 1 - position.column)
        }
        (Direction::North, Direction::East) => {
            Position::new(face_dimension - 1 - position.column, face_dimension - 1)
        }
        (Direction::North, Direction::South) => Position::new(face_dimension - 1, position.column),
        (Direction::North, Direction::West) => Position::new(position.column, 0),
        (Direction::East, Direction::North) => Position::new(0, face_dimension - 1 - position.row),
        (Direction::East, Direction::East) => {
            Position::new(face_dimension - 1 - position.row, face_dimension - 1)
        }
        (Direction::East, Direction::South) => Position::new(face_dimension - 1, position.row),
        (Direction::East, Direction::West) => Position::new(position.row, 0),
        (Direction::South, Direction::North) => Position::new(0, position.column),
        (Direction::South, Direction::East) => Position::new(position.column, face_dimension - 1),
        (Direction::South, Direction::South) => {
            Position::new(face_dimension - 1, face_dimension - 1 - position.column)
        }
        (Direction::South, Direction::West) => {
            Position::new(face_dimension - 1 - position.column, 0)
        }
        (Direction::West, Direction::North) => Position::new(0, position.row),
        (Direction::West, Direction::East) => Position::new(position.row, face_dimension - 1),
        (Direction::West, Direction::South) => {
            Position::new(face_dimension - 1, face_dimension - 1 - position.row)
        }
        (Direction::West, Direction::West) => Position::new(face_dimension - 1 - position.row, 0),
    }
}

impl WrapCache for CubeWrapCache {
    fn next(
        &mut self,
        board: &[Vec<u8>],
        position: Position,
        direction: Direction,
    ) -> Option<(Position, Direction)> {
        if let Some(hit) = self.cache.get(&(position, direction)) {
            return *hit;
        }

        let coordinates = Position::new(
            (position.row - 1) / self.face_dimension,
            (position.column - 1) / self.face_dimension,
        );
        let face = self
            .oriented_faces
            .iter()
            .find(|face| face.coordinates == coordinates)
            .unwrap();

        let mut neighbors = face.face.get_neighbors();
        orient_neighbors(&mut neighbors, face.north_toward);

        let neighbor = match direction {
            Direction::North => neighbors[0],
            Direction::East => neighbors[1],
            Direction::South => neighbors[2],
            Direction::West => neighbors[3],
        };
        let neighbor_face = self
            .oriented_faces
            .iter()
            .find(|face| face.face == neighbor)
            .unwrap();

        let mut neighbor_neighbors = neighbor_face.face.get_neighbors();
        orient_neighbors(&mut neighbor_neighbors, neighbor_face.north_toward);
        let entering_neighbor_from = match neighbor_neighbors
            .iter()
            .position(|n| *n == face.face)
            .unwrap()
        {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            3 => Direction::West,
            _ => unimplemented!(),
        };

        let face_relative_position = Position::new(
            (position.row - 1) % self.face_dimension,
            (position.column - 1) % self.face_dimension,
        );

        let destination = map_face_relative_position(
            face_relative_position,
            direction,
            entering_neighbor_from,
            self.face_dimension,
        );

        let northwest_of_neighbor =
            get_northwest_of_face(neighbor_face.coordinates, self.face_dimension);
        let destination = Position::new(
            destination.row + northwest_of_neighbor.row,
            destination.column + northwest_of_neighbor.column,
        );

        let result = match board[destination.row].get(destination.column).unwrap() {
            b'#' => None,
            b'.' => Some((destination, entering_neighbor_from.get_opposite())),
            _ => unimplemented!(),
        };

        self.cache.insert((position, direction), result);
        result
    }
}

fn parse_board(lines: impl Iterator<Item = String>) -> Vec<Vec<u8>> {
    let mut board = vec![Vec::new()];
    let mut max_length = 0;
    for line in lines {
        if line.is_empty() {
            break;
        }

        board.push(
            " ".as_bytes()
                .iter()
                .chain(line.as_bytes())
                .chain(" ".as_bytes().iter())
                .copied()
                .collect(),
        );
        max_length = max_length.max(board.last().unwrap().len());
    }

    for row in &mut board {
        row.resize(max_length, b' ');
    }

    board.push(vec![b' '; max_length]);

    board
}

#[derive(Debug)]
enum Command {
    Step(usize),
    Turn(Turn),
}

fn parse_commands(line: &str) -> Vec<Command> {
    line.split('R')
        .intersperse("R")
        .flat_map(|s| s.split('L').intersperse("L"))
        .map(|s| match s {
            "R" => Command::Turn(Turn::Right),
            "L" => Command::Turn(Turn::Left),
            _ => Command::Step(s.parse().unwrap()),
        })
        .collect()
}

fn run_commands(commands: &[Command], board: &[Vec<u8>], mut wrap_cache: impl WrapCache) {
    let column = board[1].iter().position(|b| *b == b'.').unwrap();
    let mut position = Position { row: 1, column };
    let mut direction = Direction::East;

    for command in commands {
        match command {
            Command::Turn(turn) => direction = direction.turn(*turn),
            Command::Step(steps) => {
                for _ in 0..*steps {
                    let next_position = position.next(board, &mut wrap_cache, direction);
                    if let Some((next_position, next_direction)) = next_position {
                        position = next_position;
                        direction = next_direction;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    let password = 1000 * position.row + 4 * position.column + direction as usize;
    println!("Password: {password}");
}

fn main() {
    let filename = std::env::args().nth(1).expect("Filename not found");
    let face_dimension = std::env::args()
        .nth(2)
        .expect("Face dimension not found")
        .parse()
        .unwrap();

    let file =
        File::open(&filename).unwrap_or_else(|_| panic!("Couldn't open {}", filename.as_str()));
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(std::result::Result::unwrap);

    let board = parse_board(&mut lines);
    let commands = parse_commands(&lines.next().unwrap());

    run_commands(&commands, &board, FlatWrapCache::new());

    let oriented_faces = get_oriented_faces(&board, face_dimension);
    run_commands(
        &commands,
        &board,
        CubeWrapCache::new(oriented_faces, face_dimension),
    );
}
