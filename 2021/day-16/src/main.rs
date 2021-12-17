#![feature(test)]
extern crate test;

use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use bitvec::prelude::*;

fn convert_to_binary(mut message: String) -> BitVec<Msb0, usize> {
    // Pad message out to 32 bits
    let padded_length = ((message.len() + 7) / 8) * 8;
    while message.len() < padded_length {
        message.push('0');
    }

    let mut binary = BitVec::new();

    for word in message.as_bytes().chunks(8) {
        let word = String::from_utf8_lossy(word);
        let value = u32::from_str_radix(&word, 16).unwrap();
        let insertion_point = binary.len();
        binary.resize(binary.len() + 32, false);
        binary[insertion_point..].store(value);
    }

    binary
}

const HEADER_SIZE: usize = 6;
type Version = u8;

enum TypeId {
    Literal,
    Operator(Operation),
}

#[derive(Debug, Eq, PartialEq)]
enum Operation {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
}

impl Operation {
    fn from_u8(value: u8) -> Self {
        match value {
            0 => Operation::Sum,
            1 => Operation::Product,
            2 => Operation::Minimum,
            3 => Operation::Maximum,
            5 => Operation::GreaterThan,
            6 => Operation::LessThan,
            7 => Operation::EqualTo,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Packet {
    Literal(Version, u64),
    Operator(Version, Operation, Vec<Packet>),
}

fn parse_header(packet: &BitSlice<Msb0>) -> (Version, TypeId) {
    let version: Version = packet[0..3].load_be();
    let type_id = match packet[3..6].load_be::<u8>() {
        4 => TypeId::Literal,
        value => TypeId::Operator(Operation::from_u8(value)),
    };
    (version, type_id)
}

fn parse_literal(version: Version, payload: &BitSlice<Msb0>) -> (Packet, usize) {
    let mut value = BitVec::<Msb0>::new();
    let mut number_of_chunks = 0;
    for chunk in payload.chunks(5) {
        number_of_chunks += 1;
        value.extend_from_bitslice(&chunk[1..]);
        if !chunk[0] {
            break;
        }
    }

    (
        Packet::Literal(version, value.load_be()),
        HEADER_SIZE + number_of_chunks * 5,
    )
}

fn parse_operator(
    version: Version,
    operation: Operation,
    payload: &BitSlice<Msb0>,
) -> (Packet, usize) {
    let length_is_number_of_packets = payload[0];
    let mut total_size = HEADER_SIZE;

    let mut subpackets = Vec::new();
    if length_is_number_of_packets {
        let number_of_packets: usize = payload[1..12].load_be();
        total_size += 12;

        let mut packet_start = 12;
        for _ in 0..number_of_packets {
            let (subpacket, size) = Packet::parse_from_binary(&payload[packet_start..]);
            subpackets.push(subpacket);
            packet_start += size;
            total_size += size;
        }
    } else {
        let mut length_of_packets: usize = payload[1..16].load_be();
        total_size += 16;

        let mut packet_start = 16;
        while length_of_packets > 0 {
            let (subpacket, size) = Packet::parse_from_binary(&payload[packet_start..]);
            subpackets.push(subpacket);
            packet_start += size;
            total_size += size;
            length_of_packets -= size;
        }
    }

    (Packet::Operator(version, operation, subpackets), total_size)
}

impl Packet {
    fn parse_from_binary(binary: &BitSlice<Msb0>) -> (Self, usize) {
        let (version, type_id) = parse_header(binary);
        let (packet, size) = match type_id {
            TypeId::Literal => parse_literal(version, &binary[HEADER_SIZE..]),
            TypeId::Operator(operation) => {
                parse_operator(version, operation, &binary[HEADER_SIZE..])
            }
        };
        (packet, size)
    }

    fn get_version_sum(&self) -> usize {
        match self {
            Packet::Literal(version, _value) => *version as usize,
            Packet::Operator(version, _operation, subpackets) => {
                *version as usize
                    + subpackets
                        .iter()
                        .map(|subpacket| subpacket.get_version_sum())
                        .sum::<usize>()
            }
        }
    }

    fn get_value(&self) -> u64 {
        match self {
            Packet::Literal(_version, value) => *value,
            Packet::Operator(_version, operation, subpackets) => match operation {
                Operation::Sum => subpackets
                    .iter()
                    .map(|subpacket| subpacket.get_value())
                    .sum(),
                Operation::Product => subpackets
                    .iter()
                    .map(|subpacket| subpacket.get_value())
                    .product(),
                Operation::Minimum => subpackets
                    .iter()
                    .map(|subpacket| subpacket.get_value())
                    .min()
                    .unwrap(),
                Operation::Maximum => subpackets
                    .iter()
                    .map(|subpacket| subpacket.get_value())
                    .max()
                    .unwrap(),
                Operation::GreaterThan => {
                    assert_eq!(subpackets.len(), 2);
                    (subpackets[0].get_value() > subpackets[1].get_value()) as u64
                }
                Operation::LessThan => {
                    assert_eq!(subpackets.len(), 2);
                    (subpackets[0].get_value() < subpackets[1].get_value()) as u64
                }
                Operation::EqualTo => {
                    assert_eq!(subpackets.len(), 2);
                    (subpackets[0].get_value() == subpackets[1].get_value()) as u64
                }
            },
        }
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    // println!(
    //     "Version sum: {}",
    //     Packet::parse_from_binary(&convert_to_binary(reader.lines().next().unwrap().unwrap()))
    //         .0
    //         .get_version_sum()
    // );
    println!(
        "Value: {}",
        Packet::parse_from_binary(&convert_to_binary(reader.lines().next().unwrap().unwrap()))
            .0
            .get_value()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_convert_to_binary() {
        let message = String::from("123456789ABCDE");
        assert_eq!(
            convert_to_binary(message),
            bits![
                0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 1, 1, 1,
                1, 0, 0, 0, 1, 0, 0, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1, 1, 0, 0, 1, 1, 0, 1, 1, 1, 1, 0,
                0, 0, 0, 0, 0, 0, 0, 0
            ]
        )
    }

    #[test]
    fn test_parse_literal() {
        let message = String::from("D2FE28");
        assert_eq!(
            Packet::parse_from_binary(convert_to_binary(message).as_bitslice()),
            (Packet::Literal(6, 2021), 21)
        )
    }

    #[test]
    fn test_parse_operator_with_bit_count() {
        let message = String::from("38006F45291200");
        assert_eq!(
            Packet::parse_from_binary(convert_to_binary(message).as_bitslice()),
            (
                Packet::Operator(
                    1,
                    Operation::LessThan,
                    vec![Packet::Literal(6, 10), Packet::Literal(2, 20)]
                ),
                49
            )
        );
    }

    #[test]
    fn test_parse_operator_with_packet_count() {
        let message = String::from("EE00D40C823060");
        assert_eq!(
            Packet::parse_from_binary(convert_to_binary(message).as_bitslice()),
            (
                Packet::Operator(
                    7,
                    Operation::Maximum,
                    vec![
                        Packet::Literal(2, 1),
                        Packet::Literal(4, 2),
                        Packet::Literal(1, 3)
                    ]
                ),
                51
            )
        )
    }

    #[test]
    fn test_version_sum() {
        assert_eq!(
            Packet::parse_from_binary(
                convert_to_binary(String::from("8A004A801A8002F478")).as_bitslice()
            )
            .0
            .get_version_sum(),
            16
        );
        assert_eq!(
            Packet::parse_from_binary(
                convert_to_binary(String::from("620080001611562C8802118E34")).as_bitslice()
            )
            .0
            .get_version_sum(),
            12
        );
        assert_eq!(
            Packet::parse_from_binary(
                convert_to_binary(String::from("C0015000016115A2E0802F182340")).as_bitslice()
            )
            .0
            .get_version_sum(),
            23
        );
        assert_eq!(
            Packet::parse_from_binary(
                convert_to_binary(String::from("A0016C880162017C3686B18A3D4780")).as_bitslice()
            )
            .0
            .get_version_sum(),
            31
        );
    }

    #[test]
    fn test_value() {
        assert_eq!(
            Packet::parse_from_binary(convert_to_binary(String::from("C200B40A82")).as_bitslice())
                .0
                .get_value(),
            3
        );
        assert_eq!(
            Packet::parse_from_binary(
                convert_to_binary(String::from("04005AC33890")).as_bitslice()
            )
            .0
            .get_value(),
            54
        );
        assert_eq!(
            Packet::parse_from_binary(
                convert_to_binary(String::from("880086C3E88112")).as_bitslice()
            )
            .0
            .get_value(),
            7
        );
        assert_eq!(
            Packet::parse_from_binary(
                convert_to_binary(String::from("CE00C43D881120")).as_bitslice()
            )
            .0
            .get_value(),
            9
        );
        assert_eq!(
            Packet::parse_from_binary(
                convert_to_binary(String::from("D8005AC2A8F0")).as_bitslice()
            )
            .0
            .get_value(),
            1
        );
        assert_eq!(
            Packet::parse_from_binary(convert_to_binary(String::from("F600BC2D8F")).as_bitslice())
                .0
                .get_value(),
            0
        );
        assert_eq!(
            Packet::parse_from_binary(
                convert_to_binary(String::from("9C005AC2F8F0")).as_bitslice()
            )
            .0
            .get_value(),
            0
        );
        assert_eq!(
            Packet::parse_from_binary(
                convert_to_binary(String::from("9C0141080250320F1802104A08")).as_bitslice()
            )
            .0
            .get_value(),
            1
        );
    }

    #[bench]
    fn bench_input(b: &mut Bencher) {
        let file = File::open("input.txt").unwrap();
        let reader = BufReader::new(file);
        let input = reader.lines().next().unwrap().unwrap();

        b.iter(|| {
            assert_eq!(
                Packet::parse_from_binary(&convert_to_binary(input.clone()))
                    .0
                    .get_value(),
                1675198555015
            )
        })
    }
}
