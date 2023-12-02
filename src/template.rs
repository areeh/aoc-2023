extern crate test;

use itertools::Itertools;

#[cfg(test)]
use test::Bencher;

use crate::utils::read_input_to_string;

fn part1(input: &str) {}

fn part2(input: &str) {}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(5)?;
    dbg!(part1(input));
    dbg!(part2(input));

    Ok(())
}

#[test]
fn example() {
    let input = "";
    assert_eq!(part1(input), ());
    assert_eq!(part2(input), ());
}

#[test]
fn task() {
    let input = &read_input_to_string(5).unwrap();
    assert_eq!(part1(input), ());
    assert_eq!(part2(input), ());
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = &read_input_to_string(5).unwrap();
        part1(input);
        part2(input);
    })
}
