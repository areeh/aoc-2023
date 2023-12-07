extern crate test;

use itertools::Itertools;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;

use crate::day7::HandType::{FiveKind, FourKind, FullHouse, HighCard, OnePair, ThreeKind, TwoPair};
#[cfg(test)]
use test::Bencher;

use crate::utils::read_input_to_string;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Copy, Clone)]
struct Card {
    value: u8,
}

impl Card {
    fn new(value: u8) -> Self {
        Card { value }
    }

    fn from_char(c: char) -> Option<Self> {
        let value = match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            '2'..='9' => c.to_digit(10).unwrap() as u8,
            _ => return None,
        };
        Some(Card { value })
    }

    fn from_char_p2(c: char) -> Option<Self> {
        let value = match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 1,
            'T' => 10,
            '2'..='9' => c.to_digit(10).unwrap() as u8,
            _ => return None,
        };
        Some(Card { value })
    }

    fn to_char(self) -> char {
        match self.value {
            14 => 'A',
            13 => 'K',
            12 => 'Q',
            11 => 'J',
            10 => 'T',
            n if (2u8..=9).contains(&n) => std::char::from_digit(n as u32, 10).unwrap(),
            _ => panic!("Unexpected value {0}", self.value),
        }
    }

    fn to_char_p2(self) -> char {
        match self.value {
            14 => 'A',
            13 => 'K',
            12 => 'Q',
            1 => 'J',
            10 => 'T',
            n if (2u8..=9).contains(&n) => std::char::from_digit(n as u32, 10).unwrap(),
            _ => panic!("Unexpected value {0}", self.value),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

impl HandType {
    fn from_counts(counts: &HashMap<Card, u8>) -> Self {
        match counts.len() {
            5 => HighCard,
            4 => OnePair,
            3 => {
                let max_count = counts.values().max().unwrap();
                if max_count == &3 {
                    ThreeKind
                } else if max_count == &2 {
                    TwoPair
                } else {
                    panic!("Problem min count")
                }
            }
            2 => {
                let min_count = counts.values().min().unwrap();
                if min_count == &2 {
                    FullHouse
                } else if min_count == &1 {
                    FourKind
                } else {
                    panic!("Problem min count")
                }
            }
            1 => FiveKind,
            _ => panic!("Problem uniques"),
        }
    }

    fn joker_upgrade(kind: HandType, joker_count: u8) -> HandType {
        match (kind, joker_count) {
            (kind, 0) => kind,
            (HighCard, 1) => OnePair,
            (OnePair, 1) => ThreeKind,
            (OnePair, 2) => ThreeKind,
            (TwoPair, 1) => FullHouse,
            (TwoPair, 2) => FourKind,
            (ThreeKind, 3) => FourKind,
            (ThreeKind, 1) => FourKind,
            (FullHouse, 3) => FiveKind,
            (FullHouse, 2) => FiveKind,
            (FourKind, 4) => FiveKind,
            (FourKind, 1) => FiveKind,
            (kind, count) => {
                println!("No upgrade for {kind:?} with count {count}");
                kind
            }
        }
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct Hand {
    typ: HandType,
    cards: [Card; 5],
}

impl fmt::Debug for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let card_chars: String = self.cards.iter().map(|card| card.to_char()).collect();

        write!(f, "Hand: {0} type {1:?}", card_chars, self.typ)
    }
}

impl FromStr for Hand {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards: [Card; 5] = s
            .chars()
            .map(|c| Card::from_char(c).ok_or("Got None card for {c}"))
            .collect::<Result<Vec<Card>, &str>>()
            .unwrap()
            .try_into()
            .unwrap();
        Ok(Hand::from_cards(cards))
    }
}

impl Hand {
    fn new(typ: HandType, cards: [Card; 5]) -> Self {
        Hand { typ, cards }
    }

    fn from_cards(cards: [Card; 5]) -> Self {
        let mut counts = HashMap::new();

        for number in cards {
            *counts.entry(number).or_insert(0) += 1;
        }

        let typ = HandType::from_counts(&counts);

        Hand::new(typ, cards)
    }

    fn from_cards_p2(cards: [Card; 5]) -> Self {
        let hand = Self::from_cards(cards);

        let joker_count = cards.iter().filter(|card| card == &&Card::new(1)).count();
        let kind = HandType::joker_upgrade(hand.typ, joker_count as u8);

        Hand::new(kind, cards)
    }

    fn from_str_p2(s: &str) -> Self {
        let cards: [Card; 5] = s
            .chars()
            .map(|c| Card::from_char_p2(c).ok_or("Got None card for {c}"))
            .collect::<Result<Vec<Card>, &str>>()
            .unwrap()
            .try_into()
            .unwrap();
        Hand::from_cards_p2(cards)
    }
}

fn part1(input: &str) -> usize {
    let mut hands: Vec<(Hand, u32)> = input
        .lines()
        .map(|line| {
            if let Some((hand, bid)) = line.split_whitespace().collect_tuple() {
                (hand.parse().unwrap(), bid.parse().unwrap())
            } else {
                panic!("bad hand")
            }
        })
        .collect();

    hands.sort_by(|(hand_a, _), (hand_b, _)| hand_a.cmp(hand_b));

    hands
        .iter()
        .enumerate()
        .map(|(rank, (_, bid))| (rank + 1) * (*bid as usize))
        .sum()
}

fn part2(input: &str) -> usize {
    let mut hands: Vec<(Hand, u32)> = input
        .lines()
        .map(|line| {
            if let Some((hand, bid)) = line.split_whitespace().collect_tuple() {
                (Hand::from_str_p2(hand), bid.parse().unwrap())
            } else {
                panic!("bad hand")
            }
        })
        .collect();

    hands.sort_by(|(hand_a, _), (hand_b, _)| hand_a.cmp(hand_b));

    hands
        .iter()
        .enumerate()
        .map(|(rank, (_, bid))| (rank + 1) * (*bid as usize))
        .sum()
}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(7)?;
    dbg!(part1(input));
    dbg!(part2(input));

    Ok(())
}

#[test]
fn test_parse_hand() {
    let input = "32T3K 765";
    let (hand, bid): (Hand, u32) =
        if let Some((hand, bid)) = input.split_whitespace().collect_tuple() {
            (hand.parse().unwrap(), bid.parse().unwrap())
        } else {
            panic!("ohno")
        };
    assert_eq!(
        (hand, bid),
        (
            Hand::new(
                HandType::OnePair,
                [
                    Card::new(3),
                    Card::new(2),
                    Card::new(10),
                    Card::new(3),
                    Card::new(13)
                ]
            ),
            765
        )
    );
}

#[test]
fn example() {
    let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
    assert_eq!(part1(input), 6440);
    assert_eq!(part2(input), 5905);
}

#[test]
fn task() {
    let input = &read_input_to_string(7).unwrap();
    assert_eq!(part1(input), 253313241);
    assert_eq!(part2(input), 1);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = &read_input_to_string(7).unwrap();
        part1(input);
        part2(input);
    })
}
