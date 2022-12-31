#![warn(clippy::pedantic)]

use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
};

fn snafu_to_decimal(snafu: &str) -> i64 {
    let mut decimal = 0;

    for snidget in snafu.chars() {
        decimal *= 5;

        match snidget {
            '2' => decimal += 2,
            '1' => decimal += 1,
            '0' => (),
            '-' => decimal -= 1,
            '=' => decimal -= 2,
            _ => unimplemented!(),
        }
    }

    decimal
}

fn decimal_to_snafu(mut decimal: i64) -> String {
    let mut place_value = 1;
    let mut min_to_right = 0;
    let mut max_to_right = 0;
    while 2 * place_value + max_to_right < decimal {
        min_to_right = min_to_right * 5 - 2;
        max_to_right = max_to_right * 5 + 2;
        place_value *= 5;
    }

    let mut snafu = String::new();
    while place_value > 0 {
        if decimal > place_value + max_to_right {
            snafu.push('2');
            decimal -= 2 * place_value;
        } else if decimal > max_to_right {
            snafu.push('1');
            decimal -= place_value;
        } else if decimal < -place_value + min_to_right {
            snafu.push('=');
            decimal += 2 * place_value;
        } else if decimal < min_to_right {
            snafu.push('-');
            decimal += place_value;
        } else {
            snafu.push('0');
        }

        min_to_right = (min_to_right + 2) / 5;
        max_to_right = (max_to_right - 2) / 5;
        place_value /= 5;
    }

    snafu
}

fn main() {
    let filename = std::env::args().nth(1).expect("Filename not found");

    let file =
        File::open(&filename).unwrap_or_else(|_| panic!("Couldn't open {}", filename.as_str()));
    let reader = BufReader::new(file);
    let lines = reader.lines().map(std::result::Result::unwrap);

    println!(
        "SNAFU Sum: {}",
        decimal_to_snafu(lines.map(|line| snafu_to_decimal(&line)).sum())
    );
}
