use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn get_cumulative_sums(slice: &[i32]) -> Vec<i32> {
    let mut sum = 0;
    let mut sums = Vec::new();
    for element in slice {
        sums.push(element + sum);
        sum += element;
    }
    sums
}

fn calculate_fuel_linear(
    cumulative_sums: &[i32],
    middle: i32,
    lesser_end: usize,
    greater_start: usize,
) -> (i32, i32) {
    let lesser_fuel = middle * (lesser_end + 1) as i32 - cumulative_sums[lesser_end];
    let greater_fuel = cumulative_sums[cumulative_sums.len() - 1]
        - cumulative_sums[greater_start - 1]
        - middle * (cumulative_sums.len() - greater_start) as i32;
    (lesser_fuel, greater_fuel)
}

fn get_triangle_value(value: i32) -> i32 {
    (value * (value + 1)) / 2
}

fn calculate_fuel_triangle(
    positions: &[i32],
    middle: i32,
    lesser_end: usize,
    greater_start: usize,
) -> (i32, i32) {
    let lesser_fuel = positions[0..=lesser_end]
        .iter()
        .map(|position| get_triangle_value(middle - position))
        .sum();
    let greater_fuel = positions[greater_start..]
        .iter()
        .map(|position| get_triangle_value(position - middle))
        .sum();
    (lesser_fuel, greater_fuel)
}

fn find_minimal_fuel(positions: &[i32], use_triangle_fuel_consumption: bool) -> i32 {
    let cumulative_sums = get_cumulative_sums(positions);

    let mut middle = positions[positions.len() / 2];
    let split_point = positions.len() / 2;

    let mut lesser_end = split_point;
    while positions[lesser_end] >= middle {
        lesser_end -= 1;
    }

    let mut greater_start = split_point;
    while positions[greater_start] <= middle {
        greater_start += 1;
    }

    let (mut lesser_fuel, mut greater_fuel) = if use_triangle_fuel_consumption {
        calculate_fuel_triangle(positions, middle, lesser_end, greater_start)
    } else {
        calculate_fuel_linear(&cumulative_sums, middle, lesser_end, greater_start)
    };
    let mut total_fuel = lesser_fuel + greater_fuel;

    loop {
        if lesser_fuel > greater_fuel {
            middle -= 1;

            while positions[lesser_end] >= middle {
                lesser_end -= 1;
            }

            while positions[greater_start] > middle {
                greater_start -= 1;
            }
            // Fix up overshoot
            greater_start += 1;
        } else {
            middle += 1;

            while positions[lesser_end] < middle {
                lesser_end += 1;
            }
            // Fix up overshoot
            lesser_end -= 1;

            while positions[greater_start] <= middle {
                greater_start += 1;
            }
        }

        let (lesser, greater) = if use_triangle_fuel_consumption {
            calculate_fuel_triangle(positions, middle, lesser_end, greater_start)
        } else {
            calculate_fuel_linear(&cumulative_sums, middle, lesser_end, greater_start)
        };
        // Necessary because destructuring assignments are not yet stable
        lesser_fuel = lesser;
        greater_fuel = greater;

        let new_total_fuel = lesser_fuel + greater_fuel;
        if new_total_fuel > total_fuel {
            return total_fuel;
        }

        total_fuel = new_total_fuel;
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut positions: Vec<_> = reader
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .split(',')
        .map(|position| position.parse::<i32>().unwrap())
        .collect();
    positions.sort_unstable();
    println!(
        "Minimal fuel: {}",
        find_minimal_fuel(positions.as_ref(), true)
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cumulative_sums() {
        let input = [-1, 0, 1, 2, 3, 4];
        assert_eq!(get_cumulative_sums(&input), vec![-1, -1, 0, 2, 5, 9]);
    }

    #[test]
    fn test_sample_input() {
        let mut positions = [16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
        positions.sort_unstable();
        assert_eq!(find_minimal_fuel(positions.as_ref(), false), 37);
        assert_eq!(find_minimal_fuel(positions.as_ref(), true), 168);
    }
}
