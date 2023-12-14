extern crate test;

use itertools::Itertools;
use std::collections::HashMap;
use std::ops::Range;

use crate::day12::ChunkResult::{Candidate, ForcedFail, ForcedSucc, Skip};
use crate::day12::StepResult::Complete;
#[cfg(test)]
use test::Bencher;

use crate::utils::read_input_to_string;

fn parse_line(line: &str) -> (Vec<char>, Vec<usize>) {
    if let Some((record, control)) = line.split_whitespace().collect_tuple() {
        let record: Vec<char> = "."
            .chars()
            .chain(record.chars())
            .chain(".".chars())
            .collect();
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
            count(&record, &control)
        })
        .sum()
}

struct ContiguousBlockIter<'a> {
    chars: &'a [char],
    current_pos: usize,
}

impl<'a> ContiguousBlockIter<'a> {
    fn new(chars: &'a [char]) -> Self {
        ContiguousBlockIter {
            chars,
            current_pos: 0,
        }
    }
}

impl<'a> Iterator for ContiguousBlockIter<'a> {
    type Item = Range<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_pos >= self.chars.len() {
            return None;
        }

        let mut start = None;

        while self.current_pos < self.chars.len() {
            match self.chars[self.current_pos] {
                '?' | '#' => {
                    if start.is_none() {
                        start = Some(self.current_pos);
                    }
                }
                '.' => {
                    if start.is_some() {
                        break;
                    }
                }
                _ => panic!("bad character {}", self.chars[self.current_pos]),
            }
            self.current_pos += 1;
        }

        if let Some(start) = start {
            Some(start..self.current_pos + 1)
        } else {
            None
        }
    }
}

enum ChunkResult {
    Candidate,
    ForcedSucc,
    ForcedFail,
    Skip,
}

fn classify_chunk(chunk: &[char]) -> ChunkResult {
    let contains_dot = chunk[1..chunk.len() - 1].iter().any(|&c| c == '.');
    let is_first_char_damaged = chunk.get(1) == Some(&'#');
    let are_boundaries_valid = matches!(chunk.first(), Some('?') | Some('.'))
        && matches!(chunk.last(), Some('?') | Some('.'));

    match (contains_dot, is_first_char_damaged, are_boundaries_valid) {
        (true, false, _) => Skip,
        (true, true, _) => ForcedFail,
        (false, true, true) => ForcedSucc,
        (false, true, false) => ForcedFail,
        (false, false, false) => Skip,
        (false, false, true) => Candidate,
    }
}

fn chunk_upper_bound(springs: &[char], damaged: usize) -> Option<usize> {
    let window_size = damaged + 2;
    let mut possible_end = None;

    for (window_start, window) in springs.windows(window_size).enumerate() {
        match classify_chunk(window) {
            Candidate => possible_end = Some(window_start + window_size),
            ForcedFail => return possible_end,
            ForcedSucc => return Some(window_start + window_size),
            Skip => continue,
        }
    }

    possible_end
}

fn sliding_windows(
    outer_range: Range<usize>,
    window_size: usize,
) -> impl Iterator<Item = Range<usize>> {
    let end = outer_range.end;
    (outer_range.start..end.saturating_sub(window_size - 1))
        .map(move |start| start..start + window_size)
}

fn arrangement_positions(chars: &[char], damaged: usize) -> Vec<usize> {
    let mut arrangements = Vec::new();

    for r in ContiguousBlockIter::new(chars) {
        for chunk in sliding_windows(r.clone(), damaged + 1) {
            let starts_on_damaged = chars[chunk.start] == '#';
            if chars[chunk.end - 1] == '#' {
                if starts_on_damaged {
                    return arrangements;
                } else {
                    continue;
                }
            }

            arrangements.push(chunk.end);

            if starts_on_damaged {
                return arrangements;
            }
        }
        if chars[r.clone()].contains(&'#') {
            return arrangements;
        }
    }
    arrangements
}

#[derive(Clone)]
enum StepResult {
    Complete(usize),
    Positions(Vec<usize>),
}

fn step(input: &[char], start_idx: usize, damaged: &[usize]) -> StepResult {
    let mut springs = input.get(start_idx..).unwrap_or(&[]);

    if damaged.is_empty() {
        if springs.iter().filter(|c| **c == '#').count() == 0 {
            return Complete(1);
        }
        return Complete(0);
    } else if springs.is_empty() {
        return Complete(0);
    }

    if let Some(cutoff) = chunk_upper_bound(springs, damaged[0]) {
        springs = &springs[0..cutoff];
    }
    let springs_len = springs.len();
    let mut springs = springs.to_vec();
    if springs[springs_len - 1] == '?' {
        springs[springs_len - 1] = '.';
    }
    let positions = arrangement_positions(&springs, damaged[0]);
    StepResult::Positions(positions)
}

fn count(input: &[char], damaged: &[usize]) -> usize {
    let mut queue: HashMap<(usize, &[usize]), usize> = HashMap::new();
    queue.insert((0, damaged), 1);

    let mut arrangements = 0;
    while !queue.is_empty() {
        let mut current_queue = std::mem::take(&mut queue);

        for ((start_idx, damaged), multiplier) in current_queue.drain() {
            match step(input, start_idx, damaged) {
                Complete(n) => arrangements += n * multiplier,
                StepResult::Positions(positions) => {
                    for pos in positions {
                        *queue
                            .entry((start_idx + pos, &damaged.get(1..).unwrap_or(&[])))
                            .or_insert(0) += multiplier;
                    }
                }
            }
        }
    }
    arrangements
}

#[allow(unstable_name_collisions)]
fn repeat(input: &str, pad_char: char, n: usize) -> String {
    std::iter::repeat(input)
        .take(n)
        .intersperse(&pad_char.to_string())
        .collect()
}

fn repeat_input(input: &str, n: usize) -> String {
    if let Some((record, control)) = input.split_whitespace().collect_tuple() {
        format!("{} {}", repeat(record, '?', n), repeat(control, ',', n))
    } else {
        panic!("bad input {input}")
    }
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (record, control) = parse_line(&repeat_input(line, 5));
            count(&record, &control)
        })
        .sum()
}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(12)?;
    dbg!(part1(input));
    dbg!(part2(input));

    Ok(())
}

#[test]
fn test_repeat() {
    let input = ".# 1";
    assert_eq!(repeat_input(input, 5), ".#?.#?.#?.#?.# 1,1,1,1,1")
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
            count(&record, &control)
        })
        .collect();
    assert_eq!(arrangement_counts, vec![1, 4, 1, 1, 4, 10]);
    assert_eq!(part1(input), 21);
    assert_eq!(part2(input), 525152);
}

#[test]
fn example_p2() {
    let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
    let arrangement_counts: Vec<usize> = input
        .lines()
        .map(|line| {
            let (record, control) = parse_line(&repeat_input(line, 5));
            count(&record, &control)
        })
        .collect();
    assert_eq!(arrangement_counts, vec![1, 16384, 1, 16, 2500, 506250]);
}

#[test]
fn task() {
    let input = &read_input_to_string(12).unwrap();
    assert_eq!(part1(input), 6488);
    // assert_eq!(part2(input), 815364548481);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = &read_input_to_string(12).unwrap();
        // part1(input);
        part2(input);
    })
}
