#![feature(iter_array_chunks)]

use core::panic;
use std::usize;

fn main() {
    let file = std::fs::read_to_string("day_seven_input.txt").unwrap();
    let hands = file.lines().map(|line| {
        let (hand, bid) = line.split_once(' ').unwrap();
        let cards = hand
            .chars()
            .map(|c| match c {
                c if c.is_numeric() => c.to_digit(10).unwrap() as usize,
                'T' => 10,
                'J' => 1,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => panic!("unexpected input"),
            })
            .collect::<Vec<_>>();

        let ranks: Vec<usize> = cards
            .clone()
            .into_iter()
            .fold(vec![0; 14], |mut acc, card| {
                acc[card - 1] += 1;
                acc
            });
        let ranks = ranks.into_iter().filter(|&rank| rank > 0);

        let kind = match ranks.clone().max() {
            Some(5) => HandType::FiveOfAKind,
            Some(4) => HandType::FourOfAKind,
            Some(3) if ranks.clone().count() == 2 => HandType::FullHouse,
            Some(3) => HandType::ThreeOfAKind,
            Some(2) if ranks.clone().count() == 3 => HandType::TwoPair,
            Some(2) => HandType::OnePair,
            _ => HandType::HighCard,
        };

        let hand = Hand {
            kind,
            cards,
            bid: bid.parse::<usize>().unwrap(),
        };
        dbg!(hand)
    });

    let mut answer = hands.collect::<Vec<_>>();
    answer.sort();

    answer
        .into_iter()
        .enumerate()
        .fold(0, |acc, (i, hand)| acc + hand.bid * (i + 1))
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Clone, Eq)]
struct Hand {
    kind: HandType,
    cards: Vec<usize>,
    bid: usize,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.cards == other.cards
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self == other {
            return std::cmp::Ordering::Equal;
        }

        if self.kind == other.kind {
            return self.cards.cmp(&other.cards);
        }
        self.kind.cmp(&other.kind)
    }
}
