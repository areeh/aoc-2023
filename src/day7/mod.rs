extern crate test;

use itertools::Itertools;
use std::collections::HashMap;
use std::hash::Hash;

use crate::day7::HandType::{FiveKind, FourKind, FullHouse, HighCard, OnePair, ThreeKind, TwoPair};
#[cfg(test)]
use test::Bencher;

use crate::utils::read_input_to_string;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Hash, Copy, Clone)]
struct Card {
    value: u8,
}

struct CardMap {
    map: HashMap<char, u8>,
    inv: HashMap<u8, char>,
}

impl CardMap {
    fn from_string(cards: &str) -> Self {
        let inv = cards
            .chars()
            .enumerate()
            .map(|(v, c)| (v as u8, c))
            .collect();
        let map = cards
            .chars()
            .enumerate()
            .map(|(v, c)| (c, v as u8))
            .collect();

        CardMap { map, inv }
    }
}

impl Card {
    fn new(value: u8) -> Self {
        Card { value }
    }

    fn from_char_map(c: char, card_map: &CardMap) -> Self {
        Card::new(card_map.map[&c])
    }

    #[allow(dead_code)]
    fn to_char_map(self, card_map: &CardMap) -> char {
        card_map.inv[&self.value]
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone)]
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

    fn joker_upgrade(kind: &HandType, joker_count: u8) -> HandType {
        match (kind, joker_count) {
            (kind, 0) => kind.clone(),
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
            _ => kind.clone(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Hand {
    typ: HandType,
    cards: [Card; 5],
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

    fn from_str_map(s: &str, card_map: &CardMap) -> Self {
        let cards: [Card; 5] = s
            .chars()
            .map(|c| Card::from_char_map(c, card_map))
            .collect_vec()
            .try_into()
            .unwrap();
        Hand::from_cards(cards)
    }

    fn joker_count(&self) -> u8 {
        // This is only true in p2, where J is 0
        self.cards.iter().filter(|card| card.value == 0).count() as u8
    }
}

fn parse_hands(input: &str, card_map: &CardMap) -> Vec<(Hand, u32)> {
    input
        .lines()
        .map(|line| {
            if let Some((hand, bid)) = line.split_whitespace().collect_tuple() {
                (Hand::from_str_map(hand, card_map), bid.parse().unwrap())
            } else {
                panic!("bad hand")
            }
        })
        .collect()
}

fn winnings(hands: &mut [(Hand, u32)]) -> usize {
    hands.sort_by(|(hand_a, _), (hand_b, _)| hand_a.cmp(hand_b));

    hands
        .iter()
        .enumerate()
        .map(|(rank, (_, bid))| (rank + 1) * (*bid as usize))
        .sum()
}

fn part1(input: &str) -> usize {
    let card_map = CardMap::from_string("23456789TJQKA");
    let mut hands = parse_hands(input, &card_map);
    winnings(&mut hands)
}

fn part2(input: &str) -> usize {
    let card_map = CardMap::from_string("J23456789TQKA");
    let mut hands = parse_hands(input, &card_map);
    hands
        .iter_mut()
        .for_each(|(hand, _)| hand.typ = HandType::joker_upgrade(&hand.typ, hand.joker_count()));

    winnings(&mut hands)
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
    let card_map = CardMap::from_string("23456789TJQKA");

    let (hand, bid): (Hand, u32) =
        if let Some((hand, bid)) = input.split_whitespace().collect_tuple() {
            (Hand::from_str_map(hand, &card_map), bid.parse().unwrap())
        } else {
            panic!("ohno")
        };
    assert_eq!(
        (hand, bid),
        (
            Hand::new(
                HandType::OnePair,
                [
                    Card::new(1),
                    Card::new(0),
                    Card::new(8),
                    Card::new(1),
                    Card::new(11)
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
    assert_eq!(part2(input), 253362743);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = &read_input_to_string(7).unwrap();
        part1(input);
        part2(input);
    })
}
