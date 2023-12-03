extern crate test;

use std::collections::HashMap;
use std::hash::Hash;

#[cfg(test)]
use test::Bencher;

use crate::utils::read_input_to_string;

#[derive(Eq, Hash, PartialEq, Debug)]

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

#[derive(Debug)]

struct EnginePart {
    number: u32,
    start_x: i32,
    end_x: i32,
    y: i32,
}

impl EnginePart {
    fn new(digits: &str, end_x: i32, y: i32) -> Self {
        let number: u32 = digits.parse().unwrap();
        let start_x = end_x - number.checked_ilog10().unwrap_or(0) as i32;
        EnginePart {
            number,
            start_x,
            end_x,
            y,
        }
    }
}

fn store_part(parts: &mut HashMap<Point, EnginePart>, buffer: &mut String, x_pos: i32, y_pos: i32) {
    if !buffer.is_empty() {
        let part = EnginePart::new(buffer, x_pos, y_pos);
        parts.insert(Point::new(x_pos, y_pos), part);
        buffer.clear();
    };
}

fn symbol_adjacent(part: &EnginePart, symbols: &HashMap<Point, char>) -> bool {
    for x in (part.start_x - 1)..=(part.end_x + 1) {
        for y in (part.y - 1)..=(part.y + 1) {
            if symbols.contains_key(&Point::new(x, y)) {
                return true;
            }
        }
    }
    false
}

fn update_adjacencies(
    part: &EnginePart,
    symbols: &HashMap<Point, char>,
    gear_adjacencies: &mut HashMap<Point, Vec<u32>>,
) {
    for x in (part.start_x - 1)..=(part.end_x + 1) {
        for y in (part.y - 1)..=(part.y + 1) {
            if symbols.contains_key(&Point::new(x, y)) {
                gear_adjacencies
                    .entry(Point::new(x, y))
                    .and_modify(|v| v.push(part.number))
                    .or_insert(vec![part.number]);
            }
        }
    }
}

fn parse_schematic(input: &str) -> (HashMap<Point, EnginePart>, HashMap<Point, char>) {
    let mut parts: HashMap<Point, EnginePart> = HashMap::new();
    let mut symbols: HashMap<Point, char> = HashMap::new();
    let mut buffer: String = String::new();

    for (y_pos, line) in input.lines().enumerate() {
        for (x_pos, c) in line.chars().enumerate() {
            match c {
                '.' => store_part(&mut parts, &mut buffer, x_pos as i32 - 1, y_pos as i32),
                '0'..='9' => buffer.push(c),
                _ => {
                    store_part(&mut parts, &mut buffer, x_pos as i32 - 1, y_pos as i32);
                    symbols.insert(
                        Point {
                            x: x_pos as i32,
                            y: y_pos as i32,
                        },
                        c,
                    );
                }
            }
        }
        store_part(&mut parts, &mut buffer, line.len() as i32 - 1, y_pos as i32)
    }
    (parts, symbols)
}

fn part1(input: &str) -> u32 {
    let (parts, symbols) = parse_schematic(input);

    parts
        .values()
        .filter_map(|part| {
            if symbol_adjacent(part, &symbols) {
                Some(part.number)
            } else {
                None
            }
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let (parts, mut symbols) = parse_schematic(input);
    symbols.retain(|_, c| *c == '*');

    let mut gears: HashMap<Point, Vec<u32>> = HashMap::new();
    for part in parts.values() {
        update_adjacencies(part, &symbols, &mut gears);
    }
    gears
        .values()
        .filter_map(|gear_values| {
            if gear_values.len() == 2 {
                Some(gear_values.iter().product::<u32>())
            } else {
                None
            }
        })
        .sum()
}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(3)?;
    dbg!(part1(input));
    dbg!(part2(input));

    Ok(())
}

#[test]
fn adjacent1() {
    let part = EnginePart::new("1234", 8, 2);
    let mut symbols = HashMap::new();
    symbols.insert(Point::new(3, 2), 'c');
    assert!(!symbol_adjacent(&part, &symbols));
    symbols.insert(Point::new(4, 2), 'd');
    assert!(symbol_adjacent(&part, &symbols));
}

#[test]
fn example() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    assert_eq!(part1(input), 4361);
    assert_eq!(part2(input), 467835);
}

#[test]
fn task() {
    let input = &read_input_to_string(3).unwrap();
    assert_eq!(part1(input), 525119);
    assert_eq!(part2(input), 76504829);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = &read_input_to_string(3).unwrap();
        part1(input);
        part2(input);
    })
}
