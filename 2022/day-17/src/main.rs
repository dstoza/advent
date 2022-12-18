#![warn(clippy::pedantic)]

use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
};

#[derive(Clone, Copy, Debug, Default)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn next_left(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
        }
    }

    fn next_right(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
        }
    }

    fn next_down(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
        }
    }
}

impl std::ops::Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn get_positions_by_row(
    all: &[Position],
    comparison: impl Fn(usize, usize) -> usize,
) -> Vec<Position> {
    let mut by_row = HashMap::new();
    for position in all {
        let entry = by_row.entry(position.y).or_insert(position.x);
        *entry = comparison(*entry, position.x);
    }
    by_row
        .iter()
        .map(|(row, column)| Position::new(*column, *row))
        .collect()
}

fn get_left_positions(all: &[Position]) -> Vec<Position> {
    get_positions_by_row(all, std::cmp::min)
}

fn get_right_positions(all: &[Position]) -> Vec<Position> {
    get_positions_by_row(all, std::cmp::max)
}

fn get_bottom_positions(all: &[Position]) -> Vec<Position> {
    let mut by_column = HashMap::new();
    for position in all {
        let entry = by_column.entry(position.x).or_insert(position.y);
        *entry = (*entry).min(position.y);
    }
    by_column
        .iter()
        .map(|(column, row)| Position::new(*column, *row))
        .collect()
}

struct Shape {
    all: Vec<Position>,
    left: Vec<Position>,
    right: Vec<Position>,
    right_edge: usize,
    bottom: Vec<Position>,
}

impl Shape {
    fn new(all: Vec<Position>) -> Self {
        Self {
            left: get_left_positions(&all),
            right: get_right_positions(&all),
            right_edge: all.iter().map(|position| position.x).max().unwrap(),
            bottom: get_bottom_positions(&all),
            all,
        }
    }

    fn new_flat() -> Self {
        let all = vec![
            Position::new(0, 0),
            Position::new(1, 0),
            Position::new(2, 0),
            Position::new(3, 0),
        ];
        Shape::new(all)
    }

    fn new_plus() -> Self {
        let all = vec![
            Position::new(1, 0),
            Position::new(0, 1),
            Position::new(1, 1),
            Position::new(2, 1),
            Position::new(1, 2),
        ];
        Shape::new(all)
    }

    fn new_ell() -> Self {
        let all = vec![
            Position::new(0, 0),
            Position::new(1, 0),
            Position::new(2, 0),
            Position::new(2, 1),
            Position::new(2, 2),
        ];
        Shape::new(all)
    }

    fn new_tall() -> Self {
        let all = vec![
            Position::new(0, 0),
            Position::new(0, 1),
            Position::new(0, 2),
            Position::new(0, 3),
        ];
        Shape::new(all)
    }

    fn new_square() -> Self {
        let all = vec![
            Position::new(0, 0),
            Position::new(1, 0),
            Position::new(0, 1),
            Position::new(1, 1),
        ];
        Shape::new(all)
    }
}

fn shape_would_collide<'a>(
    shape_positions: &[Position],
    position: Position,
    chamber: &Chamber,
) -> bool {
    shape_positions
        .iter()
        .map(|l| *l + position)
        .any(|position| chamber.is_occupied(position))
}

struct Rock<'a> {
    shape: &'a Shape,
    position: Position,
}

impl<'a> Rock<'a> {
    fn new(shape: &'a Shape, position: Position) -> Self {
        Self { shape, position }
    }

    fn move_left(&mut self, chamber: &Chamber) {
        if self.position.x == 0 {
            return;
        }

        if shape_would_collide(&self.shape.left, self.position.next_left(), chamber) {
            return;
        }

        self.position = self.position.next_left();
    }

    fn move_right(&mut self, chamber: &Chamber) {
        if self.position.x + self.shape.right_edge + 1 >= Chamber::WIDTH {
            return;
        }

        if shape_would_collide(&self.shape.right, self.position.next_right(), chamber) {
            return;
        }

        self.position = self.position.next_right();
    }

    fn move_down(&mut self, chamber: &Chamber) -> bool {
        if self.position.y == 0 {
            return false;
        }

        if shape_would_collide(&self.shape.bottom, self.position.next_down(), chamber) {
            return false;
        }

        self.position = self.position.next_down();
        return true;
    }
}

struct Chamber {
    columns: Vec<Vec<bool>>,
}

impl Chamber {
    const WIDTH: usize = 7;

    fn new() -> Self {
        Self {
            columns: vec![Vec::new(); Chamber::WIDTH],
        }
    }

    fn get_top(&self) -> usize {
        self.columns
            .iter()
            .map(|column| column.len())
            .max()
            .unwrap()
    }

    fn get_signature(&self) -> Vec<usize> {
        let mut signature: Vec<_> = self.columns.iter().map(|column| column.len()).collect();
        let minimum = signature.iter().copied().min().unwrap();
        for s in &mut signature {
            *s -= minimum;
        }
        signature
    }

    fn is_occupied(&self, position: Position) -> bool {
        // println!("is_occupied {:?}", position);

        if position.y >= self.columns[position.x].len() {
            return false;
        }

        self.columns[position.x][position.y]
    }

    fn place(&mut self, rock: Rock) {
        for position in rock.shape.all.iter().map(|p| *p + rock.position) {
            if position.y >= self.columns[position.x].len() {
                self.columns[position.x].resize(position.y + 1, false);
            }

            self.columns[position.x][position.y] = true;
        }
    }
}

impl std::fmt::Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in (0..self.get_top()).rev() {
            write!(f, "*")?;
            for column in 0..Chamber::WIDTH {
                if row >= self.columns[column].len() {
                    write!(f, ".")?;
                } else if self.columns[column][row] {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            write!(f, "*")?;
            writeln!(f)?;
        }
        write!(f, "")
    }
}

fn main() {
    let filename = std::env::args().nth(1).expect("Filename not found");

    let file =
        File::open(&filename).unwrap_or_else(|_| panic!("Couldn't open {}", filename.as_str()));
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(std::result::Result::unwrap);
    let commands: Vec<_> = lines.next().unwrap().chars().collect();
    println!("found {} commands", commands.len());
    let mut command_cycle = commands.iter().cycle();

    let shapes = [
        Shape::new_flat(),
        Shape::new_plus(),
        Shape::new_ell(),
        Shape::new_tall(),
        Shape::new_square(),
    ];
    let mut shape_cycle = shapes.iter().cycle();

    let mut chamber = Chamber::new();

    let mut last_seen = HashMap::new();

    let mut possible_cycle = None;

    let mut tower_heights = vec![0];

    for iteration in 0..2000 {
        for _ in 0..commands.len() {
            let mut rock = Rock::new(
                shape_cycle.next().unwrap(),
                Position::new(2, chamber.get_top() + 3),
            );

            for command in &mut command_cycle {
                match command {
                    '<' => rock.move_left(&chamber),
                    '>' => rock.move_right(&chamber),
                    _ => unimplemented!(),
                }

                if !rock.move_down(&chamber) {
                    chamber.place(rock);
                    break;
                }
            }
        }

        let tower_height = chamber.get_top();
        let difference = tower_height - tower_heights.last().unwrap();
        let mut signature = chamber.get_signature();

        signature.push(difference);
        if last_seen.contains_key(&signature) {
            let last_seen_iteration = *last_seen.get(&signature).unwrap();
            let cycle_length = iteration - last_seen_iteration;
            if let Some((possible_length, _)) = possible_cycle {
                if possible_length == cycle_length {
                    // Cycle confirmed, break
                    break;
                }
            }

            possible_cycle = Some((cycle_length, last_seen_iteration));
        } else {
            possible_cycle = None;
        }
        last_seen.insert(signature, iteration);

        tower_heights.push(tower_height);
    }

    if let Some((cycle_length, starting_iteration)) = possible_cycle {
        println!(
            "Cycle confirmed: {} iterations long, starting iteration {}",
            cycle_length, starting_iteration
        );

        let mut rocks = 1_000_000_000_000;
        rocks -= commands.len() * starting_iteration;
        println!("{} rocks are part of a cycle", rocks);

        let rocks_in_cycle = cycle_length * commands.len();
        let cycles = rocks / rocks_in_cycle;
        let growth_per_cycle = tower_heights[starting_iteration + 1 + cycle_length]
            - tower_heights[starting_iteration + 1];
        let growth_in_cycles = growth_per_cycle * cycles;
        println!("{} growth in cycles", growth_in_cycles);

        // TODO: Generalize
        println!(
            "{} including pre-cycle iterations",
            growth_in_cycles + tower_heights[1]
        );

        let rocks_in_cycle = cycle_length * commands.len();
        rocks %= rocks_in_cycle;
        println!("{} rocks after mod reduction", rocks);

        let mut chamber = Chamber::new();
        let mut command_cycle = commands.iter().cycle();
        let mut shape_cycle = shapes.iter().cycle();

        for _ in 0..starting_iteration {
            for _ in 0..commands.len() {
                let mut rock = Rock::new(
                    shape_cycle.next().unwrap(),
                    Position::new(2, chamber.get_top() + 3),
                );

                for command in &mut command_cycle {
                    match command {
                        '<' => rock.move_left(&chamber),
                        '>' => rock.move_right(&chamber),
                        _ => unimplemented!(),
                    }

                    if !rock.move_down(&chamber) {
                        chamber.place(rock);
                        break;
                    }
                }
            }
        }

        let after_cycles = chamber.get_top();

        for _ in 0..rocks {
            let mut rock = Rock::new(
                shape_cycle.next().unwrap(),
                Position::new(2, chamber.get_top() + 3),
            );

            for command in &mut command_cycle {
                match command {
                    '<' => rock.move_left(&chamber),
                    '>' => rock.move_right(&chamber),
                    _ => unimplemented!(),
                }

                if !rock.move_down(&chamber) {
                    chamber.place(rock);
                    break;
                }
            }
        }

        println!("Adding a final {}", chamber.get_top() - after_cycles);

        println!(
            "Total: {}",
            growth_in_cycles + tower_heights[1] + chamber.get_top() - after_cycles
        );
    }
}
