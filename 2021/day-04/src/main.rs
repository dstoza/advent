use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

use bit_set::BitSet;

#[derive(Debug)]
struct Board {
    lines: [BitSet; 10],
}

impl Board {
    fn from_lines<I: Iterator<Item = String>>(lines: &mut I) -> Option<Self> {
        let mut line_sets = vec![BitSet::with_capacity(100); 10];

        for (row, line) in lines.enumerate() {
            if line.is_empty() {
                break;
            }

            let split = line.split_whitespace();
            for (column, value) in split.map(|value| value.parse::<u8>().unwrap()).enumerate() {
                line_sets[row + 5].insert(value as usize);
                line_sets[column].insert(value as usize);
            }
        }

        if line_sets[0].is_empty() {
            None
        } else {
            Some(Self {
                lines: line_sets.try_into().unwrap(),
            })
        }
    }

    fn mark_number(&mut self, number: u8) -> bool {
        let mut line_completed = false;
        for line in &mut self.lines {
            line.remove(number as usize);
            if line.is_empty() {
                line_completed = true;
            }
        }
        line_completed
    }

    fn get_unmarked_sum(&self) -> u16 {
        self.lines[..5]
            .iter()
            .map(|b| b.iter().sum::<usize>())
            .sum::<usize>() as u16
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines().map(|line| line.unwrap());

    let calls = lines.next().unwrap();
    let calls = calls.split(',').map(|c| c.parse::<u8>().unwrap());
    lines.next();

    let mut boards = Vec::new();
    while let Some(board) = Board::from_lines(&mut lines) {
        boards.push(board)
    }

    // for called in calls {
    //     for board in &mut boards {
    //         let line_completed = board.mark_number(called);
    //         if line_completed {
    //             println!("Final score: {}", board.get_unmarked_sum() * called as u16);
    //             return;
    //         }
    //     }
    // }
    let mut completed_boards = HashSet::new();
    for called in calls {
        let num_boards = boards.len();
        for (index, board) in &mut boards.iter_mut().enumerate() {
            if completed_boards.contains(&index) {
                continue;
            }

            let line_completed = board.mark_number(called);
            if line_completed {
                completed_boards.insert(index);
                if completed_boards.len() == num_boards {
                    println!("Final score: {}", board.get_unmarked_sum() * called as u16);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_calls() -> [u8; 27] {
        [
            7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19,
            3, 26, 1,
        ]
    }

    fn get_boards() -> [String; 18] {
        [
            String::from("22 13 17 11  0"),
            String::from("8  2 23  4 24"),
            String::from("21  9 14 16  7"),
            String::from("6 10  3 18  5"),
            String::from("1 12 20 15 19"),
            String::from(""),
            String::from("3 15  0  2 22"),
            String::from("9 18 13 17  5"),
            String::from("19  8  7 25 23"),
            String::from("20 11 10 24  4"),
            String::from("14 21 16 12  6"),
            String::from(""),
            String::from("14 21 17 24  4"),
            String::from("10 16 15  9 19"),
            String::from("18  8 23 26 20"),
            String::from("22 11 13  6  5"),
            String::from("2  0 12  3  7"),
            String::from(""),
        ]
    }

    fn get_board_from_lines() -> Board {
        let lines = [
            String::from("22 13 17 11  0"),
            String::from("8  2 23  4 24"),
            String::from("21  9 14 16  7"),
            String::from("6 10  3 18  5"),
            String::from("1 12 20 15 19"),
            String::from(""),
        ];
        Board::from_lines(&mut lines.into_iter()).unwrap()
    }

    #[test]
    fn test_clear_row() {
        let mut board = get_board_from_lines();
        assert_eq!(board.mark_number(22), false);
        assert_eq!(board.mark_number(13), false);
        assert_eq!(board.mark_number(17), false);
        assert_eq!(board.mark_number(11), false);
        assert_eq!(board.mark_number(0), true);
    }

    #[test]
    fn test_clear_column() {
        let mut board = get_board_from_lines();
        assert_eq!(board.mark_number(22), false);
        assert_eq!(board.mark_number(8), false);
        assert_eq!(board.mark_number(21), false);
        assert_eq!(board.mark_number(6), false);
        assert_eq!(board.mark_number(1), true);
    }

    #[test]
    fn test_unmarked_sum() {
        let mut board = get_board_from_lines();
        assert_eq!(board.get_unmarked_sum(), 300);
        board.mark_number(22);
        assert_eq!(board.get_unmarked_sum(), 278);
    }

    #[test]
    fn test_sample() {
        let mut boards = Vec::new();

        let mut lines = get_boards().into_iter();
        loop {
            match Board::from_lines(&mut lines) {
                Some(board) => boards.push(board),
                None => break,
            }
        }

        assert_eq!(boards.len(), 3);

        for called in get_calls() {
            for board in &mut boards {
                let line_completed = board.mark_number(called);
                if line_completed {
                    assert_eq!(board.get_unmarked_sum() * called as u16, 4512);
                    return;
                }
            }
        }

        unreachable!()
    }
}
