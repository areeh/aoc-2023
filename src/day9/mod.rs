extern crate test;

use itertools::Itertools;

#[cfg(test)]
use test::Bencher;

use crate::utils::read_input_to_string;

fn diff<I>(iter: I) -> impl Iterator<Item = i64>
where
    I: Iterator<Item = i64>,
{
    iter.tuple_windows().map(|(a, b)| b - a)
}

fn predict_next(seq: Vec<i64>) -> i64 {
    let mut history = vec![seq];
    while !(history.last().unwrap().iter().all(|v| v == &0)) {
        let next = diff(history.last().unwrap().clone().into_iter()).collect_vec();
        history.push(next);
    }

    history
        .into_iter()
        .fold(0, |acc, next| acc + next.last().unwrap())
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let seq = line
                .split_whitespace()
                .map(str::parse)
                .collect::<Result<_, _>>()
                .unwrap();
            predict_next(seq)
        })
        .sum()
}

fn part2(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let seq = line
                .split_whitespace()
                .rev()
                .map(str::parse)
                .collect::<Result<_, _>>()
                .unwrap();
            predict_next(seq)
        })
        .sum()
}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(9)?;
    dbg!(part1(input));
    dbg!(part2(input));

    Ok(())
}

#[test]
fn example() {
    let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
    assert_eq!(part1(input), 114);
    assert_eq!(part2(input), 2);
}

#[test]
fn task() {
    let input = &read_input_to_string(9).unwrap();
    assert_eq!(part1(input), 1974913025);
    assert_eq!(part2(input), 884);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = &read_input_to_string(9).unwrap();
        part1(input);
        part2(input);
    })
}
