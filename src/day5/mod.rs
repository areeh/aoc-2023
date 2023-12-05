extern crate test;

use itertools::Itertools;
use std::ops::Range;

#[cfg(test)]
use test::Bencher;

use crate::utils::read_input_to_string;

#[derive(Debug, Eq, PartialEq)]
struct NumberMap {
    offset: i64,
    range: Range<i64>,
}

impl NumberMap {
    fn from_almanac(destination_start: i64, source_start: i64, range_length: u64) -> Self {
        NumberMap {
            offset: (destination_start - source_start),
            range: (source_start..(source_start + range_length as i64)),
        }
    }
}

fn parse_map(input: &str) -> NumberMap {
    if let Some((destination_start, source_start, range_length)) =
        input.split_whitespace().collect_tuple()
    {
        NumberMap::from_almanac(
            destination_start.parse().unwrap(),
            source_start.parse().unwrap(),
            range_length.parse().unwrap(),
        )
    } else {
        panic!("Expected the format `[destination] [source] [range_len]`, got {input}")
    }
}

fn parse_seeds(input: &str) -> Vec<i64> {
    if let Some((_, seeds)) = input.split(": ").collect_tuple() {
        seeds
            .split_whitespace()
            .map(str::parse)
            .collect::<Result<_, _>>()
            .unwrap()
    } else {
        panic!("Invalid seeds format, got {input}")
    }
}

fn parse_seed_ranges(input: &str) -> Vec<Range<i64>> {
    if let Some((_, seeds)) = input.split(": ").collect_tuple() {
        let numbers: Vec<i64> = seeds
            .split_whitespace()
            .map(str::parse)
            .collect::<Result<_, _>>()
            .unwrap();
        numbers
            .into_iter()
            .tuples()
            .map(|(start, length)| start..start + length)
            .collect()
    } else {
        panic!("Invalid seeds format, got {input}")
    }
}

fn map_to_next(num: i64, mapping: &[NumberMap]) -> i64 {
    let offset = mapping
        .iter()
        .find_map(|map| {
            if map.range.contains(&num) {
                Some(map.offset)
            } else {
                None
            }
        })
        .unwrap_or(0);
    num + offset
}

fn part1(input: &str) -> i64 {
    let mut blocks = input.split("\n\n");
    let seeds = parse_seeds(blocks.next().unwrap());

    let mut maps: Vec<Vec<NumberMap>> = vec![];

    for block in blocks {
        let mut lines = block.lines();
        lines.next().unwrap();
        maps.push(lines.map(parse_map).collect_vec())
    }

    let mut seed_changes = seeds;

    for map in &maps {
        for v in &mut seed_changes {
            *v = map_to_next(*v, map)
        }
    }

    *seed_changes.iter().min().unwrap()
}

fn range_intersection(a: &Range<i64>, b: &Range<i64>) -> Option<Range<i64>> {
    let start = a.start.max(b.start);
    let end = a.end.min(b.end);

    if start < end {
        Some(start..end)
    } else {
        None
    }
}

fn separate_intersection(a: Range<i64>, b: &Range<i64>) -> (Option<Range<i64>>, Vec<Range<i64>>) {
    let maybe_intersection = range_intersection(&a, b);

    if let Some(intersection) = maybe_intersection {
        let mut result = Vec::new();

        if a.start < intersection.start {
            result.push(a.start..intersection.start);
        }

        if intersection.end < a.end {
            result.push(intersection.end..a.end);
        }

        (Some(intersection), result)
    } else {
        (maybe_intersection, vec![a])
    }
}

fn add_to_range(range: Range<i64>, value: i64) -> Range<i64> {
    (range.start + value)..(range.end + value)
}

fn part2(input: &str) -> i64 {
    let mut blocks = input.split("\n\n");
    let mut seeds = parse_seed_ranges(blocks.next().unwrap());

    let mut maps: Vec<Vec<NumberMap>> = vec![];

    for block in blocks {
        let mut lines = block.lines();
        lines.next().unwrap();
        maps.push(lines.map(parse_map).collect_vec())
    }

    let mut seeds_transferred: Vec<Range<i64>> = vec![];
    let mut seeds_processing: Vec<Range<i64>> = vec![];

    for map in &maps {
        seeds.append(&mut seeds_transferred);
        for map_range in map {
            seeds.append(&mut seeds_processing);
            while let Some(seed) = seeds.pop() {
                let (maybe_intersection, mut remainder) =
                    separate_intersection(seed.clone(), &map_range.range);
                if let Some(intersection) = maybe_intersection {
                    seeds_processing.append(&mut remainder);
                    seeds_transferred.push(add_to_range(intersection, map_range.offset))
                } else {
                    seeds_processing.push(seed.clone());
                }
            }
        }
        seeds_transferred.append(&mut seeds_processing);
        dbg!(&seeds_transferred);
    }

    seeds_transferred.iter().map(|v| v.start).min().unwrap()
}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(5)?;
    dbg!(part1(input));
    dbg!(part2(input));

    Ok(())
}

#[test]
fn test_parse_seed_ranges() {
    let input = "seeds: 79 14 55 13";
    assert_eq!(parse_seed_ranges(input), vec![79..93, 55..68]);
}

#[test]
fn seed_to_soil() {
    let input = "50 98 2
52 50 48";

    let number_maps = input.lines().map(parse_map).collect_vec();
    assert_eq!(
        number_maps,
        vec![
            NumberMap {
                offset: -48,
                range: 98..100
            },
            NumberMap {
                offset: 2,
                range: 50..98
            }
        ]
    );
}

#[test]
fn test_range_intersection() {
    assert_eq!(range_intersection(&(1..9), &(7..12)), Some(7..9));
    assert_eq!(range_intersection(&(1..9), &(9..12)), None);
}

#[test]
fn test_separate_intersection() {
    assert_eq!(
        separate_intersection(1..9, &(7..12)),
        (Some(7..9), vec![(1..7)])
    );
}

#[test]
fn example() {
    let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
    assert_eq!(part1(input), 35);
    assert_eq!(part2(input), 46);
}

#[test]
fn task() {
    let input = &read_input_to_string(5).unwrap();
    assert_eq!(part1(input), 382895070);
    assert_eq!(part2(input), 17729182);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = &read_input_to_string(5).unwrap();
        part1(input);
        part2(input);
    })
}
