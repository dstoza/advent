use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn calculate_power_consumption<I: Iterator<Item = String>>(lines: I) -> u32 {
    let mut counters = Vec::new();
    let mut total_lines = 0;
    for line in lines {
        for (position, bit) in line.bytes().enumerate() {
            if counters.len() <= position {
                counters.push(0);
            }

            counters[position] += (bit - b'0') as u16;
        }
        total_lines += 1;
    }

    let num_counters = counters.len();
    let gamma_rate = counters.into_iter().fold(0, |value, place_count| {
        value * 2 + (place_count > total_lines / 2) as u32
    });
    let epsilon_rate = !gamma_rate & ((1u32 << num_counters) - 1);
    gamma_rate * epsilon_rate
}

fn calculate_first_position(values: &[u16]) -> u16 {
    let leading_zeros = values.last().unwrap().leading_zeros();
    1u16 << (15 - leading_zeros)
}

fn calculate_rating(values: &[u16], prefer_high: bool) -> u16 {
    let mut start = 0;
    let mut end = values.len() - 1;
    let mut position = calculate_first_position(values);
    while start != end {
        let middle = start + (end - start) / 2;

        // Check the tiebreaker first (if there are an equal number of elements)
        if (end - start) % 2 == 1 && (values[middle] ^ values[middle + 1]) & position != 0 {
            if prefer_high {
                start = middle + 1;
            } else {
                end = middle;
            }
            position >>= 1;
            continue;
        }

        let desired_value = if prefer_high { position } else { 0 };
        if values[middle] & position == desired_value {
            while (values[start] & position) == 0 && start != end {
                start += 1;
            }
        } else {
            while (values[end] & position) != 0 && end != start {
                end -= 1;
            }
        }

        position >>= 1;
    }

    values[start]
}

fn calculate_life_support_rating<I: Iterator<Item = String>>(lines: I) -> u32 {
    let mut values: Vec<u16> = lines
        .map(|s| u16::from_str_radix(s.as_ref(), 2).unwrap())
        .collect();
    values.sort_unstable();

    let oxygen_generator_rating = calculate_rating(values.as_ref(), true) as u32;
    let co2_scrubber_rating = calculate_rating(values.as_ref(), false) as u32;
    oxygen_generator_rating * co2_scrubber_rating
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    println!(
        "Life support rating: {}",
        calculate_life_support_rating(reader.lines().into_iter().map(|line| line.unwrap()))
    );
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_sample_input() -> [String; 12] {
        [
            String::from("00100"),
            String::from("11110"),
            String::from("10110"),
            String::from("10111"),
            String::from("10101"),
            String::from("01111"),
            String::from("00111"),
            String::from("11100"),
            String::from("10000"),
            String::from("11001"),
            String::from("00010"),
            String::from("01010"),
        ]
    }

    #[test]
    fn test_power_consumption() {
        assert_eq!(
            calculate_power_consumption(get_sample_input().into_iter()),
            198
        );
    }

    #[test]
    fn test_oxygen_generator_rating() {
        let mut values: Vec<u16> = get_sample_input()
            .into_iter()
            .map(|s| u16::from_str_radix(s.as_ref(), 2).unwrap())
            .collect();
        values.sort();
        assert_eq!(calculate_rating(values.as_ref(), true), 23);
    }

    #[test]
    fn test_co2_scrubber_rating() {
        let mut values: Vec<u16> = get_sample_input()
            .into_iter()
            .map(|s| u16::from_str_radix(s.as_ref(), 2).unwrap())
            .collect();
        values.sort();
        assert_eq!(calculate_rating(values.as_ref(), false), 10);
    }
}
