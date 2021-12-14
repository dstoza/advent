use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    mem::swap,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Token(u8);

type Template = Vec<usize>; // Counts of occurrences of each token
type Rules = Vec<(char, [Token; 2])>; // First character and two descendants
type RulesSlice = [(char, [Token; 2])];

fn parse_input<I: Iterator<Item = String>>(mut lines: I) -> (Template, Rules, char) {
    let mut token_map = HashMap::new();
    let mut next_token = 0u8;
    let mut get_next_token = || {
        let token = next_token;
        next_token += 1;
        Token(token)
    };

    let template = lines.next().unwrap();
    let last_character = template.chars().last().unwrap();

    let pairs: Vec<_> = template
        .chars()
        .collect::<Vec<char>>()
        .as_slice()
        .windows(2)
        .map(|window| [window[0], window[1]])
        .collect();

    let mut template = Vec::new();
    for pair in pairs {
        let token = *token_map.entry(pair).or_insert_with(&mut get_next_token);

        if token.0 as usize >= template.len() {
            template.resize(token.0 as usize + 1, 0);
        }

        template[token.0 as usize] += 1;
    }

    // Skip the blank line
    lines.next();

    let mut rules = Vec::new();
    for rule_line in lines {
        let mut split = rule_line.split(" -> ");
        let from: [char; 2] = split
            .next()
            .unwrap()
            .chars()
            .take(2)
            .collect::<Vec<char>>()
            .as_slice()
            .try_into()
            .unwrap();
        let to = split.next().unwrap().chars().next().unwrap();

        let first = *token_map
            .entry([from[0], to])
            .or_insert_with(&mut get_next_token);
        let second = *token_map
            .entry([to, from[1]])
            .or_insert_with(&mut get_next_token);

        let from_token = *token_map.entry(from).or_insert_with(&mut get_next_token);

        if from_token.0 as usize >= rules.len() {
            rules.resize(from_token.0 as usize + 1, ('x', [Token(255), Token(255)]));
        }

        rules[from_token.0 as usize] = (from[0], [first, second]);
    }

    (template, rules, last_character)
}

fn run_step(rules: &RulesSlice, template: &mut Template) {
    let mut output = Vec::new();

    for (index, count) in template.iter().enumerate() {
        let (_first_character, descendants) = &rules[index];
        for descendant in descendants {
            if descendant.0 as usize >= output.len() {
                output.resize(descendant.0 as usize + 1, 0);
            }

            output[descendant.0 as usize] += count;
        }
    }

    swap(template, &mut output);
}

fn get_difference(
    rules: &RulesSlice,
    template: &mut Template,
    last_character: char,
    steps: usize,
) -> usize {
    for _ in 0..steps {
        run_step(rules, template);
    }

    let mut letter_counts = HashMap::new();
    for (index, count) in template.iter().enumerate() {
        let (first_character, _descendants) = &rules[index];
        letter_counts
            .entry(first_character)
            .and_modify(|value| *value += *count)
            .or_insert(*count);
    }

    letter_counts
        .entry(&last_character)
        .and_modify(|value| *value += 1)
        .or_insert(1);

    let min = letter_counts.values().cloned().min().unwrap();
    let max = letter_counts.values().cloned().max().unwrap();
    max - min
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let (mut template, rules, last_character) =
        parse_input(reader.lines().map(|line| line.unwrap()));
    println!(
        "Difference: {}",
        get_difference(&rules, &mut template, last_character, 40)
    )
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_example() -> [String; 18] {
        [
            String::from("NNCB"),
            String::new(),
            String::from("CH -> B"),
            String::from("HH -> N"),
            String::from("CB -> H"),
            String::from("NH -> C"),
            String::from("HB -> C"),
            String::from("HC -> B"),
            String::from("HN -> C"),
            String::from("NN -> C"),
            String::from("BH -> H"),
            String::from("NC -> B"),
            String::from("NB -> B"),
            String::from("BN -> B"),
            String::from("BB -> N"),
            String::from("BC -> B"),
            String::from("CC -> N"),
            String::from("CN -> C"),
        ]
    }

    #[test]
    fn test_parse_input() {
        let input = [String::from("ABC"), String::new(), String::from("AC -> B")];

        let (template, rules, _last_character) = parse_input(input.into_iter());
        assert_eq!(template, vec![1, 1]);
        assert_eq!(rules[2], ('A', [Token(0), Token(1)]))
    }

    #[test]
    fn test_get_difference() {
        let (mut template, rules, last_character) = parse_input(get_example().into_iter());
        assert_eq!(
            get_difference(&rules, &mut template.clone(), last_character, 10),
            1588
        );
        assert_eq!(
            get_difference(&rules, &mut template, last_character, 40),
            2188189693529
        );
    }
}
