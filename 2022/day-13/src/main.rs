#![warn(clippy::pedantic)]

use std::{
    cmp::Ordering,
    fmt::Debug,
    fs::File,
    io::{BufRead, BufReader},
    iter::{Iterator, Peekable},
};

#[derive(Debug, Eq, PartialEq)]
enum Packet {
    Integer(i32),
    List(Vec<Packet>),
}

impl Packet {
    fn new_integer(i: i32) -> Self {
        Packet::Integer(i)
    }

    fn new_list(l: Vec<Self>) -> Self {
        Packet::List(l)
    }

    fn new_list_from_integer(i: i32) -> Self {
        Packet::List(vec![Packet::new_integer(i)])
    }

    fn new_divider(i: i32) -> Self {
        Self::new_list(vec![Self::new_list(vec![Self::new_integer(i)])])
    }

    fn parse_integer<I: Iterator<Item = char>>(peekable: &mut Peekable<I>) -> Self {
        let mut value = 0;

        while let Some(maybe_digit) = peekable.peek() {
            match maybe_digit {
                '0'..='9' => value = value * 10 + i32::from(*maybe_digit as u8 - b'0'),
                _ => {
                    return Self::new_integer(value);
                }
            }
            peekable.next();
        }

        Self::new_integer(value)
    }

    fn parse_list<I: Iterator<Item = char>>(peekable: &mut Peekable<I>) -> Self {
        // Consume opening '['
        peekable.next();

        let mut list = Vec::new();

        while peekable.peek().is_some() {
            if *peekable.peek().unwrap() == ']' {
                peekable.next(); // Consume closing bracket
                return Self::new_list(list);
            }

            list.push(Self::parse(peekable));
            if let Some(',') = peekable.peek() {
                peekable.next(); // Consume comma
            }
        }

        Self::new_list(list)
    }

    fn parse<I: Iterator<Item = char>>(peekable: &mut Peekable<I>) -> Self {
        match peekable.peek().unwrap() {
            '[' => Self::parse_list(peekable),
            _ => Self::parse_integer(peekable),
        }
    }
}

impl std::cmp::Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Packet::Integer(self_int) => {
                if let Packet::Integer(other_int) = other {
                    return self_int.cmp(other_int);
                }
                Packet::new_list_from_integer(*self_int).cmp(other)
            }
            Packet::List(self_list) => match other {
                Packet::List(other_list) => {
                    for index in 0..self_list.len().max(other_list.len()) {
                        if index >= self_list.len() {
                            return Ordering::Less;
                        }
                        if index >= other_list.len() {
                            return Ordering::Greater;
                        }
                        let ordering = self_list[index].cmp(&other_list[index]);
                        if let Ordering::Equal = ordering {
                            continue;
                        }
                        return ordering;
                    }
                    Ordering::Equal
                }
                Packet::Integer(other_int) => self.cmp(&Packet::new_list_from_integer(*other_int)),
            },
        }
    }
}

impl std::cmp::PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let filename = std::env::args().nth(1).expect("Filename not found");

    let file =
        File::open(&filename).unwrap_or_else(|_| panic!("Couldn't open {}", filename.as_str()));
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(std::result::Result::unwrap);

    let mut index = 1;
    let mut index_sum = 0;
    let mut packets = Vec::new();
    while let Some(line) = lines.next() {
        let left = Packet::parse(&mut line.chars().peekable());
        let right = Packet::parse(&mut lines.next().unwrap().chars().peekable());
        lines.next(); // Consume the blank line

        let ordering = left.cmp(&right);
        if let Ordering::Less = ordering {
            index_sum += index;
        }
        index += 1;

        packets.push(left);
        packets.push(right);
    }

    println!("Index sum: {index_sum}");

    packets.push(Packet::new_divider(2));
    packets.push(Packet::new_divider(6));

    packets.sort_unstable();

    let divider_2_position = packets
        .iter()
        .position(|packet| *packet == Packet::new_divider(2))
        .unwrap()
        + 1;

    let divider_6_position = packets
        .iter()
        .position(|packet| *packet == Packet::new_divider(6))
        .unwrap()
        + 1;

    println!("Decoder key: {}", divider_2_position * divider_6_position);
}
