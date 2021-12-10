use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
};

fn get_low_points(lines: &[Vec<u8>]) -> Vec<(usize, usize)> {
    lines
        .iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.iter().enumerate().filter_map(move |(column, byte)| {
                // 9s can never be a low point
                if *byte == b'9' {
                    return None;
                }

                // Check above
                if row > 0 && lines[row - 1][column] <= *byte {
                    return None;
                }

                // Check left
                if column > 0 && *byte >= line[column - 1] {
                    return None;
                }

                // Check right
                if column + 1 < line.len() && *byte >= line[column + 1] {
                    return None;
                }

                // Check below
                if row + 1 < lines.len() && lines[row + 1][column] <= *byte {
                    return None;
                }

                Some((row, column))
            })
        })
        .collect()
}

fn get_low_point_risk_level(lines: &[Vec<u8>]) -> i32 {
    get_low_points(lines)
        .iter()
        .map(|(row, column)| 1 + (lines[*row][*column] - b'0') as i32)
        .sum()
}

fn get_basin_size(lines: &[Vec<u8>], low_point_row: usize, low_point_column: usize) -> usize {
    let mut basin = HashSet::new();
    let mut queue = VecDeque::new();

    basin.insert((low_point_row, low_point_column));
    queue.push_back((low_point_row, low_point_column));

    while let Some((row, column)) = queue.pop_front() {
        let value = lines[row][column];

        // Check above
        if row > 0 && lines[row - 1][column] != b'9' && lines[row - 1][column] > value {
            basin.insert((row - 1, column));
            queue.push_back((row - 1, column));
        }

        // Check left
        if column > 0 && lines[row][column - 1] != b'9' && lines[row][column - 1] > value {
            basin.insert((row, column - 1));
            queue.push_back((row, column - 1));
        }

        // Check right
        if column + 1 < lines[row].len()
            && lines[row][column + 1] != b'9'
            && lines[row][column + 1] > value
        {
            basin.insert((row, column + 1));
            queue.push_back((row, column + 1));
        }

        // Check below
        if row + 1 < lines.len() && lines[row + 1][column] != b'9' && lines[row + 1][column] > value
        {
            basin.insert((row + 1, column));
            queue.push_back((row + 1, column));
        }
    }

    basin.len()
}

fn get_basin_size_product(lines: &[Vec<u8>]) -> usize {
    let mut sizes: Vec<_> = get_low_points(lines)
        .iter()
        .map(|(row, column)| get_basin_size(lines, *row, *column))
        .collect();
    sizes.sort_unstable();
    sizes[sizes.len() - 3..].iter().product()
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<_> = reader
        .lines()
        .map(|l| l.unwrap())
        .into_iter()
        .map(|s| s.into_bytes())
        .collect();
    // println!("Risk level: {}", get_low_point_risk_level(&lines));
    println!("Largest basin product: {}", get_basin_size_product(&lines));
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_example() -> Vec<Vec<u8>> {
        [
            String::from("2199943210"),
            String::from("3987894921"),
            String::from("9856789892"),
            String::from("8767896789"),
            String::from("9899965678"),
        ]
        .into_iter()
        .map(|s| s.into_bytes())
        .collect()
    }

    #[test]
    fn test_risk_level() {
        assert_eq!(get_low_point_risk_level(&get_example()), 15);
    }

    #[test]
    fn test_basin_size() {
        let example = get_example();
        assert_eq!(get_basin_size(&example, 0, 1), 3);
        assert_eq!(get_basin_size(&example, 0, 9), 9);
        assert_eq!(get_basin_size(&example, 2, 2), 14);
        assert_eq!(get_basin_size(&example, 4, 6), 9);
    }

    #[test]
    fn test_basin_size_product() {
        assert_eq!(get_basin_size_product(&get_example()), 1134);
    }
}
