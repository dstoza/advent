use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
};

fn count_increases<I: Iterator<Item = i32>>(measurements: I) -> usize {
    measurements
        .fold((None, 0), |(previous, count), current| match previous {
            None => (Some(current), count),
            Some(previous) => (Some(current), count + (current > previous) as usize),
        })
        .1
}

struct SumIterator<I: Iterator<Item = i32>> {
    inner_iterator: I,
    window_size: usize,
    window: VecDeque<i32>,
    sum: i32,
}

impl<I: Iterator<Item = i32>> Iterator for SumIterator<I> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner_iterator.next() {
            Some(i) => {
                self.sum += i;
                self.window.push_back(i);

                if self.window.len() > self.window_size {
                    self.sum -= self.window.front().unwrap();
                    self.window.pop_front();
                }

                Some(self.sum)
            }
            None => None,
        }
    }
}

impl<I: Iterator<Item = i32>> SumIterator<I> {
    fn new(mut inner_iterator: I, window_size: usize) -> Self {
        let mut window = VecDeque::new();
        let mut sum = 0;
        while window.len() < window_size - 1 {
            match inner_iterator.next() {
                Some(i) => {
                    sum += i;
                    window.push_back(i)
                }
                None => break,
            }
        }

        Self {
            inner_iterator,
            window_size,
            window,
            sum,
        }
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let measurements = reader
        .lines()
        .into_iter()
        .map(|line| line.unwrap().parse::<i32>().unwrap());

    // println!("Increases: {}", count_increases(measurements));
    println!(
        "Window increases: {}",
        count_increases(SumIterator::new(measurements, 3))
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_increases() {
        let measurements = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(count_increases(measurements.into_iter()), 7);
    }

    #[test]
    fn test_sum_iterator() {
        let measurements = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let sums: Vec<i32> = SumIterator::new(measurements.into_iter(), 3)
            .into_iter()
            .collect();
        assert_eq!(sums, vec![607, 618, 618, 617, 647, 716, 769, 792])
    }

    #[test]
    fn test_sum_too_small() {
        let measurements = [1, 2];
        let sums: Vec<i32> = SumIterator::new(measurements.into_iter(), 3)
            .into_iter()
            .collect();
        assert_eq!(sums.len(), 0);
    }
}
