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
    let lesser_fuel = if lesser_end > 0 {
        middle * lesser_end as i32 - cumulative_sums[lesser_end - 1]
    } else {
        0
    };

    let sum_before_greater = if greater_start > 0 {
        cumulative_sums[greater_start - 1]
    } else {
        0
    };
    let greater_fuel = if greater_start < cumulative_sums.len() - 1 {
        cumulative_sums[cumulative_sums.len() - 1]
            - sum_before_greater
            - middle * (cumulative_sums.len() - greater_start) as i32
    } else {
        0
    };

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
    let lesser_fuel = positions[0..lesser_end]
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
    let lesser_end = positions.partition_point(|x| *x < middle);
    let greater_start = positions.partition_point(|x| *x <= middle);

    let (mut lesser_fuel, mut greater_fuel) = if use_triangle_fuel_consumption {
        calculate_fuel_triangle(positions, middle, lesser_end, greater_start)
    } else {
        calculate_fuel_linear(&cumulative_sums, middle, lesser_end, greater_start)
    };
    let mut total_fuel = lesser_fuel + greater_fuel;

    loop {
        if lesser_fuel > greater_fuel {
            middle -= 1;
        } else {
            middle += 1;
        }

        let lesser_end = positions.partition_point(|x| *x < middle);
        let greater_start = positions.partition_point(|x| *x <= middle);

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

    #[test]
    fn test_same() {
        assert_eq!(find_minimal_fuel(&[10, 10, 10], false), 0);
        assert_eq!(find_minimal_fuel(&[10, 10, 10], true), 0);
    }

    #[test]
    fn test_single() {
        assert_eq!(find_minimal_fuel(&[0], false), 0);
        assert_eq!(find_minimal_fuel(&[0], true), 0);
    }
}
