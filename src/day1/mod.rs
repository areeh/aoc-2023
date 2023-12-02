extern crate test;

use circular_buffer::CircularBuffer;
use itertools::Either;
use std::borrow::Cow;

#[cfg(test)]
use test::Bencher;

use crate::utils::read_input_to_string;

fn part1(input: &str) -> u32 {
    let mut acc = 0;
    for line in input.lines() {
        let first = line
            .chars()
            .filter_map(|c| c.to_digit(10))
            .next()
            .expect("Could not find a digit");
        acc += 10 * first;

        let second = line
            .chars()
            .rev()
            .filter_map(|c| c.to_digit(10))
            .next()
            .expect("Could not find a digit");
        acc += second;
    }
    acc
}

fn ends_with_digit(word: &str) -> Option<u32> {
    Some(match word {
        _ if word.ends_with("one") => 1,
        _ if word.ends_with("two") => 2,
        _ if word.ends_with("three") => 3,
        _ if word.ends_with("four") => 4,
        _ if word.ends_with("five") => 5,
        _ if word.ends_with("six") => 6,
        _ if word.ends_with("seven") => 7,
        _ if word.ends_with("eight") => 8,
        _ if word.ends_with("nine") => 9,
        _ => return None,
    })
}

fn starts_with_digit(word: &str) -> Option<u32> {
    Some(match word {
        _ if word.starts_with("one") => 1,
        _ if word.starts_with("two") => 2,
        _ if word.starts_with("three") => 3,
        _ if word.starts_with("four") => 4,
        _ if word.starts_with("five") => 5,
        _ if word.starts_with("six") => 6,
        _ if word.starts_with("seven") => 7,
        _ if word.starts_with("eight") => 8,
        _ if word.starts_with("nine") => 9,
        _ => return None,
    })
}

fn buffer_ends_with_digit(buffer: &CircularBuffer<5, char>, rev: bool) -> Option<u32> {
    if rev {
        starts_with_digit(&buffer.clone().into_iter().rev().collect::<Cow<'_, str>>())
    } else {
        ends_with_digit(&buffer.clone().into_iter().collect::<Cow<'_, str>>())
    }
}

fn first_digit<I>(chars: I, first: bool) -> u32
where
    I: Iterator<Item = char>,
{
    let mut buffer = CircularBuffer::<5, char>::new();
    for c in chars {
        if let Some(d) = c.to_digit(10) {
            return d;
        } else {
            buffer.push_back(c);
            if buffer.len() >= 3 {
                if let Some(d) = buffer_ends_with_digit(&buffer, !first) {
                    return d;
                }
            }
        }
    }
    0
}

fn part2(input: &str) -> u32 {
    let mut acc = 0;

    for line in input.lines() {
        for first in [true, false] {
            let char_iter = if first {
                Either::Left(line.chars())
            } else {
                Either::Right(line.chars().rev())
            };

            let digit = first_digit(char_iter, first);
            acc += if first { 10 * digit } else { digit };
        }
    }
    acc
}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(1)?;
    dbg!(part1(input));
    dbg!(part2(input));

    Ok(())
}

#[test]
fn example() {
    let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
    assert_eq!(part1(input), 142);
}

#[test]
fn example2() {
    let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
5251fourmtxrpxvvbp4fblrpgtnlgg
ztchfjrmpgsevenzsjqzmsjj8ninehrsbgknine
9twonezv
1dcnsvzrstslsqvcvonetwofour7";
    assert_eq!(part2(input), 281 + 54 + 79 + 91 + 17);
}

#[test]
fn task() {
    let input = &read_input_to_string(1).unwrap();
    assert_eq!(part1(input), 56049);
    assert_eq!(part2(input), 54530);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = &read_input_to_string(1).unwrap();
        part1(input);
        part2(input);
    })
}
