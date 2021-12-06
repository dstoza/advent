use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const GESTATION_PERIOD: i32 = 7;
const ADOLESCENCE_DELAY: i32 = 2;

fn count_descendents(cache: &mut Vec<Option<usize>>, days: i32) -> usize {
    if days < 1 {
        return 0;
    }

    let day_index: usize = days.try_into().unwrap();
    if day_index < cache.len() {
        if let Some(cached) = cache[day_index] {
            return cached;
        }
    } else {
        cache.resize(day_index + 1, None);
    }

    let descendants = 1
        + count_descendents(cache, days - GESTATION_PERIOD)
        + count_descendents(cache, days - GESTATION_PERIOD - ADOLESCENCE_DELAY);
    cache[day_index] = Some(descendants);
    descendants
}

const DAYS: i32 = 256;

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut cache = Vec::new();
    println!(
        "Fish: {}",
        reader
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .split(',')
            .map(|phase| 1 + count_descendents(&mut cache, DAYS - phase.parse::<i32>().unwrap()))
            .sum::<usize>()
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample_input() {
        let mut cache = Vec::new();
        let descendants: usize = [3, 4, 3, 1, 2]
            .into_iter()
            .map(|phase| 1 + count_descendents(&mut cache, 18 - phase))
            .sum();
        assert_eq!(descendants, 26)
    }
}
