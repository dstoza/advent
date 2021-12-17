#![feature(test)]
extern crate test;

use std::{
    collections::{BTreeMap, HashMap, HashSet},
    ops::RangeInclusive,
};

fn get_possible_values(
    target_x: RangeInclusive<i32>,
    target_y: RangeInclusive<i32>,
) -> HashSet<(i32, i32)> {
    // The minimum possible vx value would be the first triangle number >= target_x.start()
    let vx_min = {
        let mut distance = 1;
        let mut vx = 1;
        while distance < *target_x.start() {
            vx += 1;
            distance += vx;
        }
        vx
    };

    // The maximum possible vx value would be the right side of the target
    let vx_max = *target_x.end();

    let mut vx_in_target_at_step = HashMap::new();
    let mut vx_in_target_at_or_after_step = BTreeMap::new();

    for initial_vx in vx_min..=vx_max {
        let mut step = 0usize;
        let mut x = 0;
        let mut vx = initial_vx;

        while vx > 1 {
            step += 1;
            x += vx;
            if target_x.contains(&x) {
                vx_in_target_at_step
                    .entry(step)
                    .and_modify(|entry: &mut Vec<i32>| entry.push(initial_vx))
                    .or_insert_with(|| vec![initial_vx]);
            }

            vx -= 1;
        }

        // Now vx is 1, so the next location we hit is where x will remain forever
        step += 1;
        x += vx;
        if target_x.contains(&x) {
            vx_in_target_at_or_after_step
                .entry(step)
                .and_modify(|entry: &mut Vec<i32>| entry.push(initial_vx))
                .or_insert_with(|| vec![initial_vx]);
        }
    }

    // The minimum possible vy value would be shooting downwards and hitting the bottom of the target on the first step
    let vy_min = *target_y.start();

    // When shooting upwards, the probe reaches y = 0 with the same velocity it left (just with the opposite sign),
    // so the maximum value would be such that the probe just hits the bottom of the target after this inversion
    let vy_max = -(*target_y.start()) - 1;

    let mut possible_values = HashSet::new();

    for initial_vy in vy_min..=vy_max {
        let mut step = 0usize;
        let mut y = 0;
        let mut vy = initial_vy;

        while y >= *target_y.start() {
            step += 1;
            y += vy;

            if target_y.contains(&y) {
                if let Some(vxs) = vx_in_target_at_step.get(&step) {
                    possible_values.extend(vxs.iter().map(|vx| (*vx, initial_vy)));
                }

                for (s, vxs) in &vx_in_target_at_or_after_step {
                    if *s > step {
                        break;
                    }
                    possible_values.extend(vxs.iter().map(|vx| (*vx, initial_vy)));
                }
            }

            vy -= 1;
        }
    }

    possible_values
}

fn main() {
    println!(
        "Possibilities: {}",
        get_possible_values(241..=273, -97..=-63).len()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_sample() {
        assert_eq!(get_possible_values(20..=30, -10..=-5).len(), 112);
    }

    #[bench]
    fn bench_input(b: &mut Bencher) {
        b.iter(|| {
            assert_eq!(get_possible_values(241..=273, -97..=-63).len(), 1908);
        })
    }
}
