#![deny(clippy::all, clippy::pedantic)]
#![feature(test)]

extern crate test;

use std::collections::{HashMap, HashSet};

use clap::{crate_name, App, Arg};
use common::LineReader;

struct AllergenTracker {
    candidate_ingredients: HashMap<String, HashSet<String>>,
    ingredient_counts: HashMap<String, i32>,
}

impl AllergenTracker {
    fn new() -> Self {
        Self {
            candidate_ingredients: HashMap::new(),
            ingredient_counts: HashMap::new(),
        }
    }

    fn add_food(&mut self, line: &str) {
        let mut split = line.split('(');

        let ingredients: HashSet<String> = split
            .next()
            .expect("Failed to find ingredients")
            .trim()
            .split(' ')
            .map(String::from)
            .collect();

        for ingredient in &ingredients {
            *self
                .ingredient_counts
                .entry(ingredient.clone())
                .or_insert(0) += 1;
        }

        let allergens: Vec<String> = split
            .next()
            .expect("Failed to find allergens")
            .trim_start_matches("contains ")
            .trim_end_matches(')')
            .split(", ")
            .map(String::from)
            .collect();

        for allergen in allergens {
            match self.candidate_ingredients.get_mut(&allergen) {
                Some(candidate_ingredients) => {
                    *candidate_ingredients = candidate_ingredients
                        .intersection(&ingredients)
                        .cloned()
                        .collect()
                }
                None => {
                    self.candidate_ingredients
                        .insert(allergen.clone(), ingredients.clone());
                }
            }
        }
    }

    fn collapse_known_allergens(&mut self) {
        let mut changed = true;
        while changed {
            let known_allergens: HashSet<String> = self
                .candidate_ingredients
                .values()
                .filter_map(|ingredients| {
                    if ingredients.len() == 1 {
                        Some(
                            ingredients
                                .iter()
                                .next()
                                .expect("Failed to get only element")
                                .clone(),
                        )
                    } else {
                        None
                    }
                })
                .collect();

            changed = false;
            for ingredients in self.candidate_ingredients.values_mut() {
                if ingredients.len() > 1 {
                    *ingredients = ingredients.difference(&known_allergens).cloned().collect();

                    changed = true;
                }
            }
        }
    }

    fn get_safe_ingredient_count(&self) -> i32 {
        let allergens: HashSet<String> = self
            .candidate_ingredients
            .values()
            .map(|ingredients| {
                ingredients
                    .iter()
                    .next()
                    .expect("Failed to find only ingredient")
                    .clone()
            })
            .collect();

        self.ingredient_counts
            .iter()
            .map(|(ingredient, count)| {
                if allergens.contains(ingredient) {
                    0
                } else {
                    *count
                }
            })
            .sum()
    }

    fn get_canonical_list(&self) -> String {
        let mut allergens: Vec<(String, String)> = self
            .candidate_ingredients
            .iter()
            .map(|(allergen, ingredients)| {
                (
                    ingredients
                        .iter()
                        .next()
                        .expect("Failed to find only ingredient")
                        .clone(),
                    allergen.clone(),
                )
            })
            .collect();

        allergens.sort_by_key(|(_ingredient, allergen)| allergen.clone());

        let allergens: Vec<String> = allergens
            .iter()
            .map(|(ingredient, _allergen)| ingredient.clone())
            .collect();
        allergens.as_slice().join(",")
    }
}

fn main() {
    let args = App::new(crate_name!())
        .arg(Arg::from_usage("<FILE>"))
        .get_matches();

    let mut tracker = AllergenTracker::new();

    let mut reader = LineReader::new(args.value_of("FILE").unwrap());
    reader.read_with(|line| tracker.add_food(line));

    tracker.collapse_known_allergens();

    println!(
        "Safe ingredient count: {}",
        tracker.get_safe_ingredient_count()
    );
    println!("Canonical list: {}", tracker.get_canonical_list());
}

#[cfg(test)]
mod tests {
    // use test::Bencher;
}
