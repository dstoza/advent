use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    mem::swap,
};

type Pair = [u8; 2];

fn parse_input<I: Iterator<Item = String>>(
    mut lines: I,
) -> (HashMap<Pair, usize>, HashMap<Pair, [Pair; 2]>, u8) {
    let template = lines.next().unwrap();
    let last_character = template.bytes().last().unwrap();

    let pairs: Vec<_> = template
        .bytes()
        .collect::<Vec<u8>>()
        .as_slice()
        .windows(2)
        .map(|window| [window[0], window[1]])
        .collect();

    let mut template = HashMap::new();
    for pair in pairs {
        template
            .entry(pair)
            .and_modify(|value| *value += 1)
            .or_insert(1usize);
    }

    // Skip the blank line
    lines.next();

    let rules = lines
        .into_iter()
        .map(|rule| {
            let mut split = rule.split(" -> ");
            let from: Pair = split.next().unwrap().as_bytes().try_into().unwrap();
            let to = split.next().unwrap().bytes().next().unwrap();

            let first = [from[0], to];
            let second = [to, from[1]];

            (from, [first, second])
        })
        .collect();

    (template, rules, last_character)
}

fn run_step(rules: &HashMap<Pair, [Pair; 2]>, template: &mut HashMap<Pair, usize>) {
    let mut output = HashMap::new();

    for (pair, count) in &*template {
        for rule in &rules[pair] {
            output
                .entry(*rule)
                .and_modify(|value| *value += *count)
                .or_insert(*count);
        }
    }

    swap(template, &mut output);
}

fn get_difference(
    rules: &HashMap<Pair, [Pair; 2]>,
    template: &mut HashMap<Pair, usize>,
    last_character: u8,
    steps: usize,
) -> usize {
    for _ in 0..steps {
        run_step(rules, template);
    }

    let mut letter_counts = HashMap::new();
    let last = [(&[last_character, last_character], &mut 1)];
    for (pair, count) in template.iter_mut().chain(last) {
        let first_character = pair[0];
        letter_counts
            .entry(first_character)
            .and_modify(|value| *value += *count)
            .or_insert(*count);
    }

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
        assert_eq!(
            template,
            HashMap::from([([b'A', b'B'], 1), ([b'B', b'C'], 1),])
        );
        assert_eq!(
            rules,
            HashMap::from([([b'A', b'C'], [[b'A', b'B'], [b'B', b'C']])])
        )
    }

    #[test]
    fn test_run_step() {
        let (mut template, rules, _last_character) = parse_input(get_example().into_iter());
        run_step(&rules, &mut template);
        // NCNBCHB
        assert_eq!(
            template,
            HashMap::from([
                ([b'N', b'C'], 1),
                ([b'C', b'N'], 1),
                ([b'N', b'B'], 1),
                ([b'B', b'C'], 1),
                ([b'C', b'H'], 1),
                ([b'H', b'B'], 1),
            ])
        );

        run_step(&rules, &mut template);
        // NBCCNBBBCBHCB
        assert_eq!(
            template,
            HashMap::from([
                ([b'N', b'B'], 2),
                ([b'B', b'C'], 2),
                ([b'C', b'C'], 1),
                ([b'C', b'N'], 1),
                ([b'B', b'B'], 2),
                ([b'C', b'B'], 2),
                ([b'B', b'H'], 1),
                ([b'H', b'C'], 1),
            ])
        );
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
