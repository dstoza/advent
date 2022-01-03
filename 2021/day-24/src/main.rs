#![warn(clippy::pedantic)]
#![feature(test)]

extern crate itertools;
extern crate test;

use std::{
    collections::VecDeque,
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
    ops::AddAssign,
    sync::atomic::{AtomicUsize, Ordering},
};

use itertools::join;

#[derive(Clone)]
enum Expression {
    Literal(i32),
    Input(usize),
    Sum(Vec<Expression>),
}

impl Expression {
    fn new_literal(value: i32) -> Self {
        Expression::Literal(value)
    }

    fn new_input() -> Self {
        static NEXT_INPUT: AtomicUsize = AtomicUsize::new(1);
        Expression::Input(NEXT_INPUT.fetch_add(1, Ordering::Relaxed))
    }
}

impl AddAssign for Expression {
    fn add_assign(&mut self, rhs: Self) {
        match self {
            Expression::Literal(value) => match rhs {
                Expression::Literal(other_value) => {
                    *self = Expression::Literal(*value + other_value);
                }
                _ => unimplemented!(),
            },
            Expression::Input(index) => match rhs {
                Expression::Literal(value) => {
                    *self = Expression::Sum(vec![
                        Expression::Input(*index),
                        Expression::new_literal(value),
                    ]);
                }
                _ => unimplemented!(),
            },
            Expression::Sum(values) => {
                let constant = values
                    .iter_mut()
                    .find(|expression| matches!(**expression, Expression::Literal(_)))
                    .unwrap();

                if let Expression::Literal(other_value) = rhs {
                    if let Expression::Literal(value) = &mut *constant {
                        *value += other_value;
                    } else {
                        unimplemented!()
                    }
                } else {
                    unimplemented!()
                }
            }
        }
    }
}

impl Debug for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Literal(value) => write!(f, "{}", value),
            Expression::Input(index) => write!(f, "i{}", index),
            Expression::Sum(values) => write!(
                f,
                "({})",
                join(values.iter().map(|e| format!("{:?}", e)), "+")
            ),
        }
    }
}

impl PartialEq for Expression {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Expression::Literal(value) => match other {
                Expression::Literal(other_value) => value == other_value,
                Expression::Input(_) => {
                    if *value > 9 {
                        false
                    } else {
                        unimplemented!()
                    }
                }
                Expression::Sum(_) => unimplemented!(),
            },
            Expression::Input(_) => {
                if let Expression::Input(_) = other {
                    unimplemented!()
                } else {
                    false
                }
            }
            Expression::Sum(_) => match other {
                Expression::Literal(_) => false,
                _ => unimplemented!(),
            },
        }
    }
}

type Register = VecDeque<Expression>;

#[derive(Clone)]
struct RegisterFile {
    x: Register,
    y: Register,
    z: Register,
    w: Register,
}

fn set_register(value: Expression) -> Register {
    VecDeque::from([value])
}

impl RegisterFile {
    fn new() -> Self {
        Self {
            x: set_register(Expression::new_literal(0)),
            y: set_register(Expression::new_literal(0)),
            z: set_register(Expression::new_literal(0)),
            w: set_register(Expression::new_literal(0)),
        }
    }

    fn get(&self, name: RegisterName) -> &Register {
        match name {
            RegisterName::X => &self.x,
            RegisterName::Y => &self.y,
            RegisterName::Z => &self.z,
            RegisterName::W => &self.w,
        }
    }

    fn get_mut(&mut self, name: RegisterName) -> &mut Register {
        match name {
            RegisterName::X => &mut self.x,
            RegisterName::Y => &mut self.y,
            RegisterName::Z => &mut self.z,
            RegisterName::W => &mut self.w,
        }
    }

    fn set(&mut self, name: RegisterName, value: Expression) {
        *self.get_mut(name) = set_register(value);
    }

    fn set_all(&mut self, name: RegisterName, values: Register) {
        *self.get_mut(name) = values;
    }

    fn add(&mut self, name: RegisterName, value: Expression) {
        let register = self.get_mut(name);

        if let Expression::Literal(0) = value {
            return;
        }

        if register.len() == 1 {
            register[0] += value;
        } else if let Expression::Literal(0) = register[0] {
            register[0] = value;
        } else {
            unimplemented!()
        }
    }

    fn mask(&mut self, name: RegisterName) {
        self.get_mut(name).resize(1, Expression::new_literal(1234));
    }

    fn shift_up(&mut self, name: RegisterName) {
        self.get_mut(name).push_front(Expression::new_literal(0));
    }

    fn shift_down(&mut self, name: RegisterName) {
        self.get_mut(name).pop_front();
    }
}

impl Debug for RegisterFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "x: {:?} ", self.x)?;
        write!(f, "y: {:?} ", self.y)?;
        write!(f, "z: {:?} ", self.z)?;
        writeln!(f, "w: {:?}", self.w)
    }
}

#[derive(Clone, Copy)]
enum RegisterName {
    X,
    Y,
    Z,
    W,
}

impl RegisterName {
    fn try_from_string(string: &str) -> Option<Self> {
        match string {
            "x" => Some(RegisterName::X),
            "y" => Some(RegisterName::Y),
            "z" => Some(RegisterName::Z),
            "w" => Some(RegisterName::W),
            _ => None,
        }
    }
}

impl Debug for RegisterName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegisterName::X => f.write_str("x"),
            RegisterName::Y => f.write_str("y"),
            RegisterName::Z => f.write_str("z"),
            RegisterName::W => f.write_str("w"),
        }
    }
}

#[derive(Clone, Copy)]
enum ConstraintKind {
    Equal,
    NotEqual,
}

impl Debug for ConstraintKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ConstraintKind::Equal => "==",
                ConstraintKind::NotEqual => "!=",
            }
        )
    }
}

#[derive(Clone)]
struct Constraint {
    kind: ConstraintKind,
    lhs: Register,
    rhs: Register,
}

impl Constraint {
    fn new(kind: ConstraintKind, lhs: Register, rhs: Register) -> Self {
        Self { kind, lhs, rhs }
    }
}

impl Debug for Constraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?} {:?}", self.lhs, self.kind, self.rhs)
    }
}

type Destination = RegisterName;

enum Source {
    Register(RegisterName),
    Literal(i32),
}

impl Source {
    fn from_string(string: &str) -> Self {
        if let Some(register) = RegisterName::try_from_string(string) {
            Source::Register(register)
        } else {
            Source::Literal(string.parse().unwrap())
        }
    }
}

impl Debug for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Source::Register(register) => register.fmt(f),
            Source::Literal(value) => write!(f, "{}", value),
        }
    }
}

enum Instruction {
    Inp(Destination),
    Add(Destination, Source),
    Mul(Destination, Source),
    Div(Destination, Source),
    Mod(Destination, Source),
    Eql(Destination, Source),
}

impl Instruction {
    fn parse_from_lines<I: Iterator<Item = String>>(lines: I) -> Vec<Instruction> {
        lines
            .map(|line| {
                let mut split = line.split_whitespace();
                match split.next().unwrap() {
                    "inp" => Instruction::Inp(
                        RegisterName::try_from_string(split.next().unwrap()).unwrap(),
                    ),
                    "add" => Instruction::Add(
                        RegisterName::try_from_string(split.next().unwrap()).unwrap(),
                        Source::from_string(split.next().unwrap()),
                    ),
                    "mul" => Instruction::Mul(
                        RegisterName::try_from_string(split.next().unwrap()).unwrap(),
                        Source::from_string(split.next().unwrap()),
                    ),
                    "div" => Instruction::Div(
                        RegisterName::try_from_string(split.next().unwrap()).unwrap(),
                        Source::from_string(split.next().unwrap()),
                    ),
                    "mod" => Instruction::Mod(
                        RegisterName::try_from_string(split.next().unwrap()).unwrap(),
                        Source::from_string(split.next().unwrap()),
                    ),
                    "eql" => Instruction::Eql(
                        RegisterName::try_from_string(split.next().unwrap()).unwrap(),
                        Source::from_string(split.next().unwrap()),
                    ),
                    _ => unreachable!(),
                }
            })
            .collect()
    }

    fn execute_add(
        register_file: &mut RegisterFile,
        destination: RegisterName,
        source: &Source,
    ) -> bool {
        let destination_value = register_file.get(destination);
        if destination_value.len() == 1 && destination_value[0] == Expression::Literal(0) {
            match source {
                Source::Register(name) => {
                    register_file.set_all(destination, register_file.get(*name).clone());
                }
                Source::Literal(value) => {
                    register_file.set(destination, Expression::new_literal(*value));
                }
            }
        } else {
            match source {
                Source::Register(name) => {
                    let source_value = register_file.get(*name);
                    let source_value = if source_value.len() == 1 {
                        source_value[0].clone()
                    } else {
                        unimplemented!()
                    };

                    register_file.add(destination, source_value);
                }
                Source::Literal(value) => {
                    register_file.add(destination, Expression::new_literal(*value));
                }
            }
        }
        true
    }

    fn execute_mul(
        register_file: &mut RegisterFile,
        destination: RegisterName,
        source: &Source,
    ) -> bool {
        match source {
            Source::Literal(0) => {
                register_file.set(destination, Expression::new_literal(0));
            }
            Source::Register(name) => {
                let destination_value = register_file.get(destination);
                let source_value = register_file.get(*name);
                if source_value.len() == 1 && source_value[0] == Expression::new_literal(1)
                    || destination_value.len() == 1
                        && destination_value[0] == Expression::new_literal(0)
                {
                    // Nothing happens
                } else if source_value.len() == 1 && source_value[0] == Expression::new_literal(26)
                {
                    register_file.shift_up(destination);
                } else if source_value.len() == 1 && source_value[0] == Expression::new_literal(0) {
                    register_file.set(destination, Expression::new_literal(0));
                } else {
                    unimplemented!()
                }
            }
            Source::Literal(_) => unimplemented!(),
        }
        true
    }

    fn execute_eql(
        register_file: &mut RegisterFile,
        constraints: &[Constraint],
        remainder: &[Instruction],
        destination: RegisterName,
        source: &Source,
    ) -> bool {
        match source {
            Source::Register(name) => {
                let destination_value = register_file.get(destination);
                let source_value = register_file.get(*name);
                if destination_value.len() != 1 {
                    unimplemented!();
                }

                match &destination_value[0] {
                    Expression::Input(_) | Expression::Literal(_) => {
                        let equal = destination_value == source_value;
                        register_file.set(destination, Expression::new_literal(equal as i32));
                    }
                    Expression::Sum(values) => {
                        let literal_sum: i32 = values
                            .iter()
                            .filter_map(|expression| {
                                if let Expression::Literal(value) = *expression {
                                    Some(value)
                                } else {
                                    None
                                }
                            })
                            .sum();

                        if source_value.len() != 1 {
                            unimplemented!()
                        }

                        if let Expression::Input(_) = source_value[0] {
                            if literal_sum > 9 {
                                register_file.set(destination, Expression::new_literal(0));
                            } else {
                                // Split the universe
                                for kind in [ConstraintKind::Equal, ConstraintKind::NotEqual] {
                                    let constraint = Constraint::new(
                                        kind,
                                        destination_value.clone(),
                                        source_value.clone(),
                                    );
                                    let constraints: Vec<Constraint> =
                                        constraints.iter().cloned().chain([constraint]).collect();
                                    let mut equal_register_file = register_file.clone();
                                    let value = match kind {
                                        ConstraintKind::Equal => 1,
                                        ConstraintKind::NotEqual => 0,
                                    };
                                    equal_register_file
                                        .set(destination, Expression::new_literal(value));
                                    execute(equal_register_file, &constraints, remainder);
                                }

                                return false;
                            }
                        } else {
                            unimplemented!()
                        }
                    }
                }

                true
            }
            Source::Literal(value) => {
                let destination_value = register_file.get(destination);
                if destination_value.len() == 1 {
                    let equal = destination_value[0] == Expression::new_literal(*value);
                    register_file.set(destination, Expression::new_literal(equal as i32));
                } else {
                    unimplemented!()
                }
                true
            }
        }
    }

    fn execute(
        &self,
        register_file: &mut RegisterFile,
        constraints: &[Constraint],
        remainder: &[Instruction],
    ) -> bool {
        match self {
            Instruction::Inp(destination) => {
                register_file.set(*destination, Expression::new_input());
                true
            }
            Instruction::Add(destination, source) => {
                Instruction::execute_add(register_file, *destination, source)
            }
            Instruction::Mul(destination, source) => {
                Instruction::execute_mul(register_file, *destination, source)
            }
            Instruction::Div(destination, source) => {
                match source {
                    Source::Literal(1) => {}
                    Source::Literal(26) => register_file.shift_down(*destination),
                    _ => unimplemented!(),
                }
                true
            }
            Instruction::Mod(destination, source) => {
                if let Source::Literal(26) = source {
                    register_file.mask(*destination);
                } else {
                    unimplemented!()
                }
                true
            }
            Instruction::Eql(destination, source) => Instruction::execute_eql(
                register_file,
                constraints,
                remainder,
                *destination,
                source,
            ),
        }
    }
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Inp(destination) => write!(f, "inp {:?}", destination),
            Instruction::Add(destination, source) => {
                write!(f, "add {:?} {:?}", destination, source)
            }
            Instruction::Mul(destination, source) => {
                write!(f, "mul {:?} {:?}", destination, source)
            }
            Instruction::Div(destination, source) => {
                write!(f, "div {:?} {:?}", destination, source)
            }
            Instruction::Mod(destination, source) => {
                write!(f, "mod {:?} {:?}", destination, source)
            }
            Instruction::Eql(destination, source) => {
                write!(f, "eql {:?} {:?}", destination, source)
            }
        }
    }
}

fn execute(
    mut register_file: RegisterFile,
    constraints: &[Constraint],
    instructions: &[Instruction],
) {
    for (index, instruction) in instructions.iter().enumerate() {
        // println!("{} {:?}", index, instruction);
        if !instruction.execute(&mut register_file, constraints, &instructions[index + 1..]) {
            return;
        }
        // println!("{:?} {:?}", constraints, register_file);
    }
    println!("{:?} {:?}", constraints, register_file);
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let instructions =
        Instruction::parse_from_lines(reader.lines().map(std::result::Result::unwrap));
    execute(RegisterFile::new(), &Vec::new(), &instructions);
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
