extern crate test;

use itertools::Itertools;
use std::collections::HashMap;

#[cfg(test)]
use test::Bencher;

use crate::utils::read_input_to_string;

fn parse_game_id(input: &str) -> u32 {
    if let Some((_, game_id)) = input.split_whitespace().collect_tuple() {
        game_id
            .parse()
            .unwrap_or_else(|_| panic!("Unexpected game id {game_id}"))
    } else {
        panic!("Expected format Game [id], got {input}")
    }
}

fn block_limit(block: &str) -> u32 {
    match block {
        "red" => 12,
        "green" => 13,
        "blue" => 14,
        _ => panic!("Unexpected block {block}"),
    }
}

fn block_over_limit(input: &str) -> bool {
    if let Some((amount, color)) = input.split_whitespace().collect_tuple() {
        amount.parse::<u32>().unwrap() > block_limit(color)
    } else {
        panic!("Expected format amount color, got {input}")
    }
}

fn part1(input: &str) -> u32 {
    let mut acc = 0;
    for line in input.lines() {
        if let Some((game, record)) = line.split(": ").collect_tuple() {
            if !record
                .split("; ")
                .flat_map(|s| s.split(", "))
                .map(block_over_limit)
                .any(|value| value)
            {
                acc += parse_game_id(game);
            }
        }
    }
    acc
}

type CubeCount = HashMap<String, u32>;

fn update_cube_count(input: &str, cubes_needed: &mut CubeCount) {
    if let Some((amount, color)) = input.split_whitespace().collect_tuple() {
        let amount = amount.parse().unwrap();
        cubes_needed
            .entry(color.to_string())
            .and_modify(|e| *e = (*e).max(amount))
            .or_insert(amount);
    } else {
        panic!("Expected format `[amount] [color]`, got {input}")
    }
}

fn part2(input: &str) -> u32 {
    let mut acc = 0;
    for line in input.lines() {
        let mut blocks: HashMap<String, u32> = HashMap::new();
        if let Some((_, record)) = line.split(": ").collect_tuple() {
            for cube_draw in record.split("; ").flat_map(|s| s.split(", ")) {
                update_cube_count(cube_draw, &mut blocks)
            }
        }
        acc += blocks.values().product::<u32>()
    }
    acc
}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(2)?;
    dbg!(part1(input));
    dbg!(part2(input));

    Ok(())
}

#[test]
fn example() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    assert_eq!(part1(input), 8);
    assert_eq!(part2(input), 2286);
}

#[test]
fn task() {
    let input = &read_input_to_string(2).unwrap();
    assert_eq!(part1(input), 2545);
    assert_eq!(part2(input), 78111);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = &read_input_to_string(2).unwrap();
        part1(input);
        part2(input);
    })
}
