use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn bits_from_letters(letters: &str) -> u8 {
    let mut bits = 0;
    for byte in letters.as_bytes() {
        bits |= 1u8 << (byte - b'a');
    }
    bits
}

fn digit_bits_from_configurations(configurations: &str) -> [u8; 10] {
    let mut sorted_by_configuration_length: Vec<_> = {
        let mut vector: Vec<_> = configurations.split(' ').collect();
        vector.sort_by_key(|s| s.len());
        vector.iter().map(|s| bits_from_letters(s)).collect()
    };

    let mut bits_for_digit = [0u8; 10];

    bits_for_digit[1] = sorted_by_configuration_length[0];
    bits_for_digit[7] = sorted_by_configuration_length[1];
    bits_for_digit[4] = sorted_by_configuration_length[2];
    bits_for_digit[8] = sorted_by_configuration_length[9];

    let candidates_for_069 = &mut sorted_by_configuration_length[6..=8];
    // Of 0, 6, and 9, only 9 is the same when ORed with 4
    let (position_of_9, _) = candidates_for_069
        .iter()
        .enumerate()
        .find(|(_position, bits)| **bits | bits_for_digit[4] == **bits)
        .unwrap();
    // Move it into the correct position in the candidates list
    candidates_for_069.swap(position_of_9, 2);
    // This leaves 0 and 6 in positions 0 and 1 in some order
    // Of these, only 0 is the same when ORed with 7, so if it's in position 1, swap it into position 0
    if candidates_for_069[1] | bits_for_digit[7] == candidates_for_069[1] {
        candidates_for_069.swap(0, 1);
    }
    // Now we can unpack the candidates into the final array
    bits_for_digit[0] = candidates_for_069[0];
    bits_for_digit[6] = candidates_for_069[1];
    bits_for_digit[9] = candidates_for_069[2];

    let candidates_for_235 = &mut sorted_by_configuration_length[3..=5];
    // Of 2, 3, and 5, only 3 is the same when ORed with 1
    let (position_of_3, _) = candidates_for_235
        .iter()
        .enumerate()
        .find(|(_position, bits)| **bits | bits_for_digit[1] == **bits)
        .unwrap();
    // Move it into the correct position in the candidates list
    candidates_for_235.swap(position_of_3, 1);
    // This leaves 2 and 5 in positions 0 and 2 in some order
    // Of these, only 5 is the same when ANDed with 6, so if it's in position 0, swap it into position 2
    if candidates_for_235[0] & bits_for_digit[6] == candidates_for_235[0] {
        candidates_for_235.swap(0, 2);
    }
    // Now we can unpack the candidates into the final array
    bits_for_digit[2] = candidates_for_235[0];
    bits_for_digit[3] = candidates_for_235[1];
    bits_for_digit[5] = candidates_for_235[2];

    bits_for_digit
}

fn parse_line(line: &str) -> ([u8; 10], Vec<u8>) {
    let mut split = line.split(" | ");
    let digit_bits = digit_bits_from_configurations(split.next().unwrap());
    let output_bits: Vec<_> = split
        .next()
        .unwrap()
        .split(' ')
        .map(|digit| bits_from_letters(digit))
        .collect();
    (digit_bits, output_bits)
}

fn count_easy_digits(line: &str) -> usize {
    let (digit_bits, output_bits) = parse_line(line);
    output_bits
        .iter()
        .filter(|digit| {
            let (digit, _) = digit_bits
                .iter()
                .enumerate()
                .find(|(_position, bits)| **bits == **digit)
                .unwrap();
            digit == 1 || digit == 4 || digit == 7 || digit == 8
        })
        .count()
}

fn get_output_value(line: &str) -> i32 {
    let (digit_bits, output_bits) = parse_line(line);
    output_bits.iter().fold(0, |value, bits| {
        let (digit, _bits) = digit_bits
            .iter()
            .enumerate()
            .find(|(_position, b)| **b == *bits)
            .unwrap();
        let digit: i32 = digit.try_into().unwrap();
        value * 10 + digit
    })
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    // print!(
    //     "Easy digits: {}",
    //     reader
    //         .lines()
    //         .map(|line| count_easy_digits(&line.unwrap()))
    //         .sum::<usize>()
    // );
    print!(
        "Sum: {}",
        reader
            .lines()
            .map(|line| get_output_value(&line.unwrap()))
            .sum::<i32>()
    );
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_example() -> [String; 10] {
        [
            String::from("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe"),
            String::from("edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc"),
            String::from("fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg"),
            String::from("fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb"),
            String::from("aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea"),
            String::from("fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb"),
            String::from("dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe"),
            String::from("bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef"),
            String::from("egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb"),
            String::from("gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"),
        ]
    }

    #[test]
    fn test_letters_into_bits() {
        assert_eq!(bits_from_letters("acf"), 0b100101);
    }

    #[test]
    fn test_digit_bits_from_configurations() {
        assert_eq!(
            digit_bits_from_configurations(
                "abcefg cf acdeg acdfg bcdf abdfg abdefg acf abcdefg abcdfg"
            ),
            [
                0b1110111, 0b100100, 0b1011101, 0b1101101, 0b101110, 0b1101011, 0b1111011,
                0b100101, 0b1111111, 0b1101111
            ]
        )
    }

    #[test]
    fn test_easy_digits_example() {
        assert_eq!(
            get_example()
                .iter()
                .map(|line| count_easy_digits(line))
                .sum::<usize>(),
            26
        );
    }

    #[test]
    fn test_sum() {
        assert_eq!(
            get_example()
                .iter()
                .map(|line| get_output_value(line))
                .sum::<i32>(),
            61229
        );
    }
}
