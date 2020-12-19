#![deny(clippy::all, clippy::pedantic)]

use clap::{crate_name, App, Arg};
use common::LineReader;

fn sum_product2(sorted: &[i32], target: i32) -> Option<i32> {
    let mut candidate_index = sorted.len() - 1;
    for number in sorted {
        while number + sorted[candidate_index] > target {
            if candidate_index == 0 {
                return None;
            }

            candidate_index -= 1;
        }

        if number + sorted[candidate_index] == target {
            return Some(number * sorted[candidate_index]);
        }
    }

    None
}

fn sum_product3(sorted: &[i32], target: i32) -> Option<i32> {
    let mut end = sorted.len() - 1;
    for number in sorted {
        while number + sorted[end] > target {
            end -= 1;
        }

        if let Some(product2) = sum_product2(&sorted[0..end], target - number) {
            return Some(product2 * number);
        }
    }

    None
}

fn main() {
    let args = App::new(crate_name!())
        .arg(Arg::from_usage("<FILE>"))
        .arg(
            Arg::from_usage("-n, --entries <ENTRIES> 'Number of entries to consider'")
                .possible_value("2")
                .possible_value("3"),
        )
        .get_matches();

    let mut reader = LineReader::new(args.value_of("FILE").unwrap());
    let mut array = Vec::<i32>::new();
    reader.read_with(|line| {
        array.push(
            line.parse()
                .unwrap_or_else(|_| panic!("Failed to parse {}", line)),
        )
    });

    array.sort_unstable();
    let result = match args.value_of("entries").unwrap() {
        "2" => sum_product2(&array, 2020),
        "3" => sum_product3(&array, 2020),
        _ => unreachable!("Impossible argument value"),
    };

    println!("Result: {}", result.expect("Failed to find sum product"));
}
