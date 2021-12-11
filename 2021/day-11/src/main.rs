use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn flash_cell(lines: &mut [Vec<u8>], row: usize, column: usize) {
    if lines[row][column] != 10 {
        return;
    }

    for (row_delta, column_delta) in [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ] {
        if row == 0 && row_delta < 0 {
            continue;
        }

        if column == 0 && column_delta < 0 {
            continue;
        }

        if column == lines[0].len() - 1 && column_delta > 0 {
            continue;
        }

        if row == lines.len() - 1 && row_delta > 0 {
            continue;
        }

        let row = ((row as i32) + row_delta) as usize;
        let column = ((column as i32) + column_delta) as usize;
        let neighbor = &mut lines[row][column];
        *neighbor += 1;
        flash_cell(lines, row, column);
    }
}

fn run_generation(lines: &mut [Vec<u8>]) -> i32 {
    for row in 0..lines.len() {
        for column in 0..lines[0].len() {
            lines[row][column] += 1;
            flash_cell(lines, row, column);
        }
    }

    let mut flashes = 0;
    for line in lines {
        for cell in line {
            if *cell > 9 {
                flashes += 1;
                *cell = 0;
            }
        }
    }
    flashes
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines: Vec<Vec<u8>> = reader
        .lines()
        .map(|line| line.unwrap())
        .into_iter()
        .map(|line| line.into_bytes().into_iter().map(|b| b - b'0').collect())
        .collect();

    // let mut flashes = 0;
    // for _ in 0..100 {
    //     flashes += run_generation(&mut lines);
    // }
    // println!("Flashes: {}", flashes)

    let mut step = 1;
    loop {
        if run_generation(&mut lines) == 100 {
            println!("Step {}", step);
            break;
        }
        step += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_simple() -> Vec<Vec<u8>> {
        [
            String::from("11111"),
            String::from("19991"),
            String::from("19191"),
            String::from("19991"),
            String::from("11111"),
        ]
        .into_iter()
        .map(|line| line.into_bytes().into_iter().map(|b| b - b'0').collect())
        .collect()
    }

    fn get_example() -> Vec<Vec<u8>> {
        [
            String::from("5483143223"),
            String::from("2745854711"),
            String::from("5264556173"),
            String::from("6141336146"),
            String::from("6357385478"),
            String::from("4167524645"),
            String::from("2176841721"),
            String::from("6882881134"),
            String::from("4846848554"),
            String::from("5283751526"),
        ]
        .into_iter()
        .map(|line| line.into_bytes().into_iter().map(|b| b - b'0').collect())
        .collect()
    }

    #[test]
    fn test_two_simple_steps() {
        let mut simple = get_simple();
        run_generation(&mut simple);
        run_generation(&mut simple);
        assert_eq!(simple[0], vec![4, 5, 6, 5, 4]);
        assert_eq!(simple[1], vec![5, 1, 1, 1, 5]);
    }

    #[test]
    fn test_ten_steps() {
        let mut example = get_example();
        let mut sum = 0;
        for _ in 0..10 {
            sum += run_generation(&mut example);
        }

        assert_eq!(example[0], vec![0, 4, 8, 1, 1, 1, 2, 9, 7, 6]);
        assert_eq!(example[1], vec![0, 0, 3, 1, 1, 1, 2, 0, 0, 9]);
        assert_eq!(example[2], vec![0, 0, 4, 1, 1, 1, 2, 5, 0, 4]);
        assert_eq!(example[3], vec![0, 0, 8, 1, 1, 1, 1, 4, 0, 6]);
        assert_eq!(example[4], vec![0, 0, 9, 9, 1, 1, 1, 3, 0, 6]);
        assert_eq!(example[5], vec![0, 0, 9, 3, 5, 1, 1, 2, 3, 3]);
        assert_eq!(example[6], vec![0, 4, 4, 2, 3, 6, 1, 1, 3, 0]);
        assert_eq!(example[7], vec![5, 5, 3, 2, 2, 5, 2, 3, 5, 0]);
        assert_eq!(example[8], vec![0, 5, 3, 2, 2, 5, 0, 6, 0, 0]);
        assert_eq!(example[9], vec![0, 0, 3, 2, 2, 4, 0, 0, 0, 0]);
        assert_eq!(sum, 204);
    }
}
