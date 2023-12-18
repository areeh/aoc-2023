extern crate test;

use crate::day18::Direction::{Down, Left, Right, Up};
use itertools::Itertools;
use std::fmt;
use std::ops::{Add, AddAssign};
use std::str::FromStr;

#[cfg(test)]
use test::Bencher;

use crate::utils::read_input_to_string;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    fn from_digit(digit: u32) -> Self {
        match digit {
            0 => Right,
            1 => Down,
            2 => Left,
            3 => Up,
            _ => panic!("Bad direction digit {digit}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    fn move_n_steps(self, dir: Direction, n: usize) -> Self {
        let mut pos = self;
        for _ in 0..n {
            pos += dir;
        }
        pos
    }
}

impl Add<Direction> for Position {
    type Output = Position;

    fn add(self, dir: Direction) -> Position {
        match dir {
            Up => Position {
                x: self.x,
                y: self.y - 1,
            },
            Left => Position {
                x: self.x - 1,
                y: self.y,
            },
            Down => Position {
                x: self.x,
                y: self.y + 1,
            },
            Right => Position {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

impl AddAssign<Direction> for Position {
    fn add_assign(&mut self, other: Direction) {
        *self = *self + other;
    }
}

impl FromStr for Direction {
    type Err = ParseDirectionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "U" => Ok(Up),
            "L" => Ok(Left),
            "D" => Ok(Down),
            "R" => Ok(Right),
            _ => Err(ParseDirectionError(s.to_string())),
        }
    }
}

#[derive(Debug, Clone)]
struct ParseDirectionError(String);

impl fmt::Display for ParseDirectionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid direction: {}", self.0)
    }
}

impl std::error::Error for ParseDirectionError {}

fn parse_step_p1(input: &str) -> (Direction, usize) {
    if let Some((dir, n, _)) = input.split_whitespace().collect_tuple() {
        (dir.parse().unwrap(), n.parse().unwrap())
    } else {
        panic!("bad step {input}")
    }
}

fn parse_step_p2(input: &str) -> (Direction, usize) {
    if let Some((_, _, hexa)) = input.split_whitespace().collect_tuple() {
        let number = &hexa[2..7];
        let direction = hexa.chars().nth(7).unwrap();
        (
            Direction::from_digit(
                direction
                    .to_digit(10)
                    .unwrap_or_else(|| panic!("bad direction {direction}")),
            ),
            usize::from_str_radix(number, 16).unwrap_or_else(|_| panic!("Bad hexa {number}")),
        )
    } else {
        panic!("bad step {input}")
    }
}

fn shoelace(start_pos: Position, edges: &[(Direction, usize)]) -> usize {
    let mut area: isize = 2;

    let mut current_position = start_pos;
    for (dir, n) in edges.iter() {
        let next_pos = current_position.move_n_steps(*dir, *n);

        area += current_position.x * next_pos.y;
        area -= next_pos.x * current_position.y;
        area += *n as isize;
        current_position = next_pos;
    }

    (area / 2) as usize
}

fn part1(input: &str) -> usize {
    let moves = input.trim().split('\n').map(parse_step_p1).collect_vec();
    shoelace(Position::new(0, 0), &moves)
}

fn part2(input: &str) -> usize {
    let moves = input.trim().split('\n').map(parse_step_p2).collect_vec();
    shoelace(Position::new(0, 0), &moves)
}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(18)?;
    dbg!(part1(input));
    dbg!(part2(input));

    Ok(())
}

#[test]
fn example() {
    let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
    assert_eq!(part1(input), 62);
    assert_eq!(part2(input), 952408144115);
}

#[test]
fn task() {
    let input = &read_input_to_string(18).unwrap();
    assert_eq!(part1(input), 62500);
    assert_eq!(part2(input), 122109860712709);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = &read_input_to_string(18).unwrap();
        part1(input);
        part2(input);
    })
}
