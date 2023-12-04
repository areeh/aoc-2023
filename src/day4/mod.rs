extern crate test;

use itertools::Itertools;
use std::collections::HashSet;

#[cfg(test)]
use test::Bencher;

use crate::utils::read_input_to_string;

fn winners(input: &str) -> Vec<u32> {
    if let Some((winning, numbers)) = input.split('|').collect_tuple() {
        let win_set: HashSet<u32> = winning
            .split_whitespace()
            .map(str::parse)
            .collect::<Result<HashSet<u32>, _>>()
            .unwrap();
        let our_numbers: Vec<u32> = numbers
            .split_whitespace()
            .map(str::parse)
            .collect::<Result<Vec<u32>, _>>()
            .unwrap();
        our_numbers
            .into_iter()
            .filter(|v| win_set.contains(v))
            .collect()
    } else {
        panic!("Did not find the pattern [winning]|[numbers], got {input}")
    }
}

fn part1(input: &str) -> u32 {
    let mut score = 0;
    for line in input.lines() {
        let winning_numbers = if let Some((_, numbers)) = line.split(':').collect_tuple() {
            winners(numbers)
        } else {
            panic!("Did not find pattern Card [n]:[numbers]")
        };
        if !winning_numbers.is_empty() {
            score += 2_u32.pow(winning_numbers.len().saturating_sub(1) as u32);
        }
    }
    score
}

fn part2(input: &str) -> u32 {
    let win_counts: Vec<usize> = input
        .lines()
        .map(|line| {
            if let Some((_, numbers)) = line.split(':').collect_tuple() {
                winners(numbers).len()
            } else {
                panic!("Did not find pattern Card [n]:[numbers]")
            }
        })
        .collect();
    let mut card_counts = vec![1; win_counts.len()];
    for (i, count) in win_counts.iter().enumerate() {
        let n = card_counts[i];
        for v in &mut card_counts[i + 1..i + 1 + *count] {
            *v += n;
        }
    }
    card_counts.iter().sum()
}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(4)?;
    dbg!(part1(input));
    dbg!(part2(input));

    Ok(())
}

#[test]
fn card1_winners() {
    let input = "41 48 83 86 17 | 83 86  6 31 17  9 48 53";

    assert_eq!(winners(input), [83, 86, 17, 48]);
}

#[test]
fn example() {
    let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    assert_eq!(part1(input), 13);
    assert_eq!(part2(input), 30);
}

#[test]
fn task() {
    let input = &read_input_to_string(4).unwrap();
    assert_eq!(part1(input), 18519);
    assert_eq!(part2(input), 11787590);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = &read_input_to_string(4).unwrap();
        part1(input);
        part2(input);
    })
}
