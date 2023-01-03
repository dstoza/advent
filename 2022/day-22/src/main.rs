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

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
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

    fn opposite(self) -> Self {
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

    fn step(self, facing: Direction) -> Self {
        match facing {
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
        wrap_cache: &mut WrapCache,
        facing: Direction,
    ) -> Option<Self> {
        let next_coordinates = self.step(facing);

        match board[next_coordinates.row][next_coordinates.column] {
            b'#' => None,
            b'.' => Some(next_coordinates),
            b' ' => wrap_cache.next(board, self, facing),
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
    tile_coordinates: Position,
}

impl OrientedFace {
    fn new(face: Face, north_toward: Face, tile_coordinates: Position) -> Self {
        Self {
            face,
            north_toward,
            tile_coordinates,
        }
    }
}

fn get_northwest_of_tile(tile_coordinates: Position, face_dimension: usize) -> Position {
    Position::new(
        1 + tile_coordinates.row * face_dimension,
        1 + tile_coordinates.column * face_dimension,
    )
}

fn neighbor_exists(
    tile_coordinates: Position,
    direction: Direction,
    board: &[Vec<u8>],
    face_dimension: usize,
) -> bool {
    let neighbor_position = tile_coordinates.step(direction);
    let neighbor_northwest = get_northwest_of_tile(neighbor_position, face_dimension);
    board[neighbor_northwest.row][neighbor_northwest.column] != b' '
}

fn get_oriented_faces(board: &[Vec<u8>], face_dimension: usize) -> Vec<OrientedFace> {
    let board_tile_width = (board[0].len() - 2) / face_dimension;
    let board_tile_height = (board.len() - 2) / face_dimension;

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
        let north_neighbor_position = neighbors
            .iter()
            .position(|n| *n == current_face.north_toward)
            .unwrap();
        neighbors.rotate_left(north_neighbor_position);

        // North neighbor
        if !visited.contains(&neighbors[0])
            && current_face.tile_coordinates.row > 0
            && neighbor_exists(
                current_face.tile_coordinates,
                Direction::North,
                board,
                face_dimension,
            )
        {
            to_process.push(OrientedFace::new(
                neighbors[0],
                current_face.face.get_opposite(),
                current_face.tile_coordinates.step(Direction::North),
            ));
            visited.insert(neighbors[0]);
        }

        // East neighbor
        if !visited.contains(&neighbors[1])
            && current_face.tile_coordinates.column < board_tile_width - 1
            && neighbor_exists(
                current_face.tile_coordinates,
                Direction::East,
                board,
                face_dimension,
            )
        {
            to_process.push(OrientedFace::new(
                neighbors[1],
                current_face.north_toward,
                current_face.tile_coordinates.step(Direction::East),
            ));
            visited.insert(neighbors[1]);
        }

        // South neighbor
        if !visited.contains(&neighbors[2])
            && current_face.tile_coordinates.row < board_tile_height - 1
            && neighbor_exists(
                current_face.tile_coordinates,
                Direction::South,
                board,
                face_dimension,
            )
        {
            to_process.push(OrientedFace::new(
                neighbors[2],
                current_face.face,
                current_face.tile_coordinates.step(Direction::South),
            ));
            visited.insert(neighbors[2]);
        }

        // West neighbor
        if !visited.contains(&neighbors[3])
            && current_face.tile_coordinates.column > 0
            && neighbor_exists(
                current_face.tile_coordinates,
                Direction::West,
                board,
                face_dimension,
            )
        {
            to_process.push(OrientedFace::new(
                neighbors[3],
                current_face.north_toward,
                current_face.tile_coordinates.step(Direction::West),
            ));
            visited.insert(neighbors[3]);
        }

        oriented_faces.push(current_face);
    }

    oriented_faces
}

struct WrapCache {
    cache: HashMap<(Position, Direction), Option<Position>>,
}

impl WrapCache {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    fn next(
        &mut self,
        board: &[Vec<u8>],
        position: Position,
        facing: Direction,
    ) -> Option<Position> {
        if let Some(hit) = self.cache.get(&(position, facing)) {
            return *hit;
        }

        let mut search = position.step(facing.opposite());
        while board[search.row][search.column] != b' ' {
            search = search.step(facing.opposite());
        }
        search = search.step(facing);

        let result = match board[search.row][search.column] {
            b'#' => None,
            b'.' => Some(search),
            _ => unimplemented!(),
        };

        self.cache.insert((position, facing), result);
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

fn parse_commands(line: String) -> Vec<Command> {
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

fn run_commands(commands: &[Command], board: &[Vec<u8>]) {
    let mut wrap_cache = WrapCache::new();

    let column = board[1].iter().position(|b| *b == b'.').unwrap();
    let mut position = Position { row: 1, column };
    let mut facing = Direction::East;

    for command in commands {
        match command {
            Command::Turn(turn) => facing = facing.turn(*turn),
            Command::Step(steps) => {
                for _ in 0..*steps {
                    let next_position = position.next(board, &mut wrap_cache, facing);
                    if let Some(next_position) = next_position {
                        position = next_position;
                    } else {
                        break;
                    }
                }
            }
        }
    }

    let password = 1000 * position.row + 4 * position.column + facing as usize;
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
    let commands = parse_commands(lines.next().unwrap());

    run_commands(&commands, &board);

    let oriented_faces = get_oriented_faces(&board, face_dimension);
    for face in oriented_faces {
        println!("{face:?}");
    }
}
