extern crate test;

use itertools::Itertools;

#[cfg(test)]
use test::Bencher;

use crate::utils::read_input_to_string;

fn distance(total_time: i64, hold_time: i64) -> i64 {
    if hold_time >= total_time {
        panic!("You tried to hold for all the time")
    } else {
        hold_time * (total_time - hold_time)
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Race {
    time: i64,
    record: i64,
}

impl Race {
    fn new(time: i64, record: i64) -> Self {
        Race { time, record }
    }
}

fn parse_races(input: &str) -> Vec<Race> {
    if let Some((times, distances)) = input.lines().collect_tuple() {
        times
            .split_whitespace()
            .skip(1)
            .zip(distances.split_whitespace().skip(1))
            .map(|(time, record)| match (time.parse(), record.parse()) {
                (Ok(time), Ok(record)) => Race { time, record },
                _ => panic!("Could not parse time {time} record {record}"),
            })
            .collect()
    } else {
        panic!("Unexpected race format {input}")
    }
}

fn binary_search_left<F>(func: F, target: i64, low: i64, peak: i64) -> i64
where
    F: Fn(i64) -> i64,
{
    let mut low = low;
    let mut high = peak;

    while low < high {
        let mid = low + (high - low) / 2;

        if func(mid) > target {
            high = mid;
        } else {
            low = mid + 1;
        }
    }

    low
}

fn binary_search_right<F>(func: F, target: i64, peak: i64, high: i64) -> i64
where
    F: Fn(i64) -> i64,
{
    let mut low = peak;
    let mut high = high;

    while low < high {
        let mid = low + (high - low + 1) / 2;

        if func(mid) > target {
            low = mid;
        } else {
            high = mid - 1;
        }
    }

    if func(low) < target {
        low -= 1;
    }

    low
}

fn find_threshold_range<F>(func: F, peak: i64, threshold: i64, low: i64, high: i64) -> (i64, i64)
where
    F: Fn(i64) -> i64,
{
    if func(peak) < threshold {
        panic!("we're spooked")
    }

    let left = binary_search_left(&func, threshold, low, peak);
    let right = binary_search_right(&func, threshold, peak, high);

    (left, right)
}

fn parse_races2(input: &str) -> Vec<Race> {
    if let Some((times, distances)) = input.lines().collect_tuple() {
        vec![Race::new(
            times
                .replace("Time: ", "")
                .replace(' ', "")
                .parse()
                .unwrap(),
            distances
                .replace("Distance: ", "")
                .replace(' ', "")
                .parse()
                .unwrap(),
        )]
    } else {
        panic!("Unexpected race format {input}")
    }
}

fn parts(races: Vec<Race>) -> usize {
    let mut acc = 1;
    for race in races {
        let range = find_threshold_range(
            |hold_time| distance(race.time, hold_time),
            race.time / 2,
            race.record,
            1,
            race.time - 1,
        );
        acc *= (range.0..=range.1).try_len().unwrap();
    }

    acc
}

fn part1(input: &str) -> usize {
    let races = parse_races(input);

    parts(races)
}

fn part2(input: &str) -> usize {
    let races = parse_races2(input);

    parts(races)
}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(6)?;
    dbg!(part1(input));
    dbg!(part2(input));

    Ok(())
}

#[test]
fn example_parse() {
    let input = "Time:      7  15   30
Distance:  9  40  200";
    assert_eq!(
        parse_races(input),
        vec![Race::new(7, 9), Race::new(15, 40), Race::new(30, 200)]
    );
}

#[test]
fn example_parse2() {
    let input = "Time:      7  15   30
Distance:  9  40  200";
    assert_eq!(
        parse_races(input),
        vec![Race::new(7, 9), Race::new(15, 40), Race::new(30, 200)]
    );
}

#[test]
fn example() {
    let input = "Time:      7  15   30
Distance:  9  40  200";
    assert_eq!(part1(input), 288);
    assert_eq!(part2(input), 71503);
}

#[test]
fn task() {
    let input = &read_input_to_string(6).unwrap();
    assert_eq!(part1(input), 293046);
    assert_eq!(part2(input), 35150181);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = &read_input_to_string(6).unwrap();
        part1(input);
        part2(input);
    })
}
