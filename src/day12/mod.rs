extern crate test;

use itertools::Itertools;

#[cfg(test)]
use test::Bencher;

use crate::utils::read_input_to_string;

fn print_chars(chars: &[char]) {
    println!("{}", chars.iter().collect::<String>())
}

fn missing_damaged(chars: &[char], damaged: &[usize]) -> usize {
    damaged.iter().sum::<usize>() - chars.iter().filter(|c| **c == '#').count()
}

fn unknown_positions(chars: &[char]) -> Vec<usize> {
    chars
        .iter()
        .enumerate()
        .filter_map(|(i, c)| if *c == '?' { Some(i) } else { None })
        .collect()
}

fn contiguous_damaged(chars: &[char]) -> Vec<usize> {
    let mut damaged: Vec<usize> = Vec::new();
    let mut count = 0;
    for char in chars {
        if *char == '#' {
            count += 1;
        } else if count > 0 {
            damaged.push(count);
            count = 0;
        }
    }
    if count > 0 {
        damaged.push(count);
    }
    damaged
}

fn count_arrangements(chars: &[char], damaged: &[usize]) -> usize {
    let positions = unknown_positions(chars);
    let count_missing = missing_damaged(chars, damaged);
    let mut arrangements = 0;

    let mut chars = chars.to_vec();
    for arr in positions.clone().into_iter().combinations(count_missing) {
        for pos in &positions {
            chars[*pos] = '.'
        }
        for pos in arr {
            chars[pos] = '#'
        }
        if contiguous_damaged(&chars) == damaged {
            arrangements += 1;
        }
    }
    arrangements
}

fn parse_line(line: &str) -> (Vec<char>, Vec<usize>) {
    if let Some((record, control)) = line.split_whitespace().collect_tuple() {
        let record: Vec<char> = record.chars().collect();
        let control: Vec<usize> = control
            .split(',')
            .map(str::parse)
            .collect::<Result<_, _>>()
            .unwrap();
        (record, control)
    } else {
        panic!("Bad input line {line}")
    }
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (record, control) = parse_line(line);
            count_arrangements(&record, &control)
        })
        .sum()
}

fn part2(input: &str) -> usize {
    1
}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(12)?;
    dbg!(part1(input));
    dbg!(part2(input));

    Ok(())
}

#[test]
fn mini() {
    let input = "???.### 1,1,3";
    let (record, control) = parse_line(input);
    count_arrangements(&record, &control);
    assert_eq!(count_arrangements(&record, &control), 1);
}

#[test]
fn mini2() {
    let input = ".??..??...?##. 1,1,3";
    let (record, control) = parse_line(input);
    count_arrangements(&record, &control);
    assert_eq!(count_arrangements(&record, &control), 4);
}

#[test]
fn example() {
    let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
    let arrangement_counts: Vec<usize> = input
        .lines()
        .map(|line| {
            let (record, control) = parse_line(line);
            count_arrangements(&record, &control)
        })
        .collect();
    assert_eq!(arrangement_counts, vec![1, 4, 1, 1, 4, 10]);
    assert_eq!(part1(input), 21);
    assert_eq!(part2(input), 1);
}

#[test]
fn task() {
    let input = &read_input_to_string(12).unwrap();
    assert_eq!(part1(input), 6488);
    assert_eq!(part2(input), 1);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = &read_input_to_string(12).unwrap();
        part1(input);
        part2(input);
    })
}
