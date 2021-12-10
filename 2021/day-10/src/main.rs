use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, Eq, PartialEq)]
enum ParseStatus {
    Corrupted(u8),
    Incomplete(Vec<u8>),
    Valid,
}

fn parse_line(line: &str) -> ParseStatus {
    let closing_characters =
        HashMap::from([(b'(', b')'), (b'[', b']'), (b'{', b'}'), (b'<', b'>')]);

    let mut stack = Vec::new();
    for char in line.as_bytes() {
        match char {
            b'(' | b'[' | b'{' | b'<' => stack.push(*char),
            b')' | b']' | b'}' | b'>' => {
                if *char != closing_characters[stack.last().unwrap_or(&0u8)] {
                    return ParseStatus::Corrupted(*char);
                }
                stack.pop();
            }
            _ => unreachable!(),
        }
    }

    if !stack.is_empty() {
        stack.reverse();
        ParseStatus::Incomplete(stack)
    } else {
        ParseStatus::Valid
    }
}

fn get_corrupted_score<I: Iterator<Item = String>>(lines: I) -> i32 {
    let score_table = HashMap::from([(b')', 3), (b']', 57), (b'}', 1197), (b'>', 25137)]);
    lines
        .map(|line| match parse_line(&line) {
            ParseStatus::Corrupted(illegal) => score_table[&illegal],
            _ => 0,
        })
        .sum()
}

fn get_incomplete_score<I: Iterator<Item = String>>(lines: I) -> i64 {
    let score_table = HashMap::from([(b'(', 1), (b'[', 2), (b'{', 3), (b'<', 4)]);
    let mut scores: Vec<_> = lines
        .filter_map(|line| match parse_line(&line) {
            ParseStatus::Incomplete(closing_characters) => Some(
                closing_characters
                    .into_iter()
                    .fold(0, |sum, b| sum * 5 + score_table[&b]),
            ),
            _ => None,
        })
        .collect();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    // println!(
    //     "Corrupted score: {}",
    //     get_corrupted_score(reader.lines().map(|l| l.unwrap()))
    // );
    println!(
        "Incomplete score: {}",
        get_incomplete_score(reader.lines().map(|l| l.unwrap()))
    );
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_example() -> [String; 10] {
        [
            String::from("[({(<(())[]>[[{[]{<()<>>"),
            String::from("[(()[<>])]({[<{<<[]>>("),
            String::from("{([(<{}[<>[]}>{[]{[(<()>"),
            String::from("(((({<>}<{<{<>}{[]{[]{}"),
            String::from("[[<[([]))<([[{}[[()]]]"),
            String::from("[{[{({}]{}}([{[{{{}}([]"),
            String::from("{<[[]]>}<{[{[{[]{()[[[]"),
            String::from("[<(<(<(<{}))><([]([]()"),
            String::from("<{([([[(<>()){}]>(<<{{"),
            String::from("<{([{{}}[<[[[<>{}]]]>[]]"),
        ]
    }

    #[test]
    fn test_legal() {
        assert_eq!(parse_line("()"), ParseStatus::Valid);
        assert_eq!(parse_line("[]"), ParseStatus::Valid);
        assert_eq!(parse_line("([])"), ParseStatus::Valid);
        assert_eq!(parse_line("{()()()}"), ParseStatus::Valid);
        assert_eq!(parse_line("<([{}])>"), ParseStatus::Valid);
        assert_eq!(parse_line("[<>({}){}[([])<>]]"), ParseStatus::Valid);
        assert_eq!(parse_line("(((((((((())))))))))"), ParseStatus::Valid);
    }

    #[test]
    fn test_corrupted() {
        assert_eq!(parse_line("(]"), ParseStatus::Corrupted(b']'));
        assert_eq!(parse_line("{()()()>"), ParseStatus::Corrupted(b'>'));
        assert_eq!(parse_line("(((()))}"), ParseStatus::Corrupted(b'}'));
        assert_eq!(parse_line("<([]){()}[{}])"), ParseStatus::Corrupted(b')'));
    }

    #[test]
    fn test_corrupted_score() {
        assert_eq!(get_corrupted_score(get_example().into_iter()), 26397);
    }

    #[test]
    fn test_incomplete_score_single() {
        assert_eq!(
            get_incomplete_score([String::from("<{([")].into_iter()),
            294
        );
    }

    #[test]
    fn test_incomplete_score() {
        assert_eq!(get_incomplete_score(get_example().into_iter()), 288957);
    }
}
