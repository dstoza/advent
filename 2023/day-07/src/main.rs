#![warn(clippy::pedantic)]
use std::{
    cmp::{Ord, Ordering},
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    iter::Iterator,
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Label {
    #[cfg(feature = "joker")]
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    #[cfg(not(feature = "joker"))]
    Jack,
    Queen,
    King,
    Ace,
}

impl Label {
    fn parse(b: u8) -> Self {
        match b {
            #[cfg(feature = "joker")]
            b'J' => Self::Joker,
            b'2' => Self::Two,
            b'3' => Self::Three,
            b'4' => Self::Four,
            b'5' => Self::Five,
            b'6' => Self::Six,
            b'7' => Self::Seven,
            b'8' => Self::Eight,
            b'9' => Self::Nine,
            b'T' => Self::Ten,
            #[cfg(not(feature = "joker"))]
            b'J' => Self::Jack,
            b'Q' => Self::Queen,
            b'K' => Self::King,
            b'A' => Self::Ace,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Clone, Copy, Debug)]
struct Hand([Label; 5]);

impl Hand {
    fn count_labels(self) -> (usize, usize) {
        #[cfg(feature = "joker")]
        let mut jokers = 0;

        let mut label_counts = HashMap::new();
        for card in self.0 {
            #[cfg(feature = "joker")]
            if card == Label::Joker {
                jokers += 1;
                continue;
            }

            label_counts
                .entry(card)
                .and_modify(|count: &mut usize| *count += 1)
                .or_insert(1);
        }
        let distinct_labels = label_counts.keys().count();
        let max_label = label_counts.values().max().copied().unwrap_or_default();

        #[cfg(feature = "joker")]
        let max_label = max_label + jokers;

        (distinct_labels, max_label)
    }

    fn get_type(self) -> Type {
        let (distinct_labels, max_label) = self.count_labels();
        match (distinct_labels, max_label) {
            (_, 5) => Type::FiveOfAKind,
            (2, 4) => Type::FourOfAKind,
            (2, 3) => Type::FullHouse,
            (3, 3) => Type::ThreeOfAKind,
            (3, 2) => Type::TwoPair,
            (4, _) => Type::OnePair,
            (5, _) => Type::HighCard,
            _ => unreachable!(),
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_type = self.get_type();
        let other_type = other.get_type();
        if self_type == other_type {
            for (s, o) in self.0.iter().zip(other.0.iter()) {
                if s != o {
                    return s.cmp(o);
                }
            }
            Ordering::Equal
        } else {
            self_type.cmp(&other_type)
        }
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut hands = reader
        .lines()
        .map(std::result::Result::unwrap)
        .map(|line| {
            let mut split = line.split_whitespace();
            let hand = split
                .next()
                .unwrap()
                .as_bytes()
                .iter()
                .map(|b| Label::parse(*b))
                .collect::<Vec<_>>();
            let hand: [Label; 5] = hand.as_slice().try_into().unwrap();
            let hand = Hand(hand);
            let bid: u32 = split.next().unwrap().parse().unwrap();
            (hand, bid)
        })
        .collect::<Vec<_>>();

    hands.sort_unstable_by_key(|(hand, _)| *hand);

    #[allow(clippy::cast_possible_truncation)]
    let winnings: u32 = hands
        .iter()
        .enumerate()
        .map(|(index, (_, bid))| *bid * (index + 1) as u32)
        .sum();

    println!("{winnings}");
}
