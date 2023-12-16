extern crate test;

use itertools::Itertools;
use std::collections::HashSet;

use ndarray::{Array2, Axis, Dim};
#[cfg(test)]
use test::Bencher;

use crate::utils::{parse_board, read_input_to_string};

type Board = Array2<char>;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn to_index(self) -> [usize; 2] {
        [self.y, self.x]
    }

    fn manhattan(&self, other: &Self) -> usize {
        ((self.x as isize - other.x as isize).abs() + (self.y as isize - other.y as isize).abs())
            as usize
    }
}

fn additions(axis: Axis, board: &Board) -> Vec<usize> {
    board
        .lanes(axis)
        .into_iter()
        .enumerate()
        .filter_map(|(i, lane)| {
            if lane.iter().all(|c| *c == '.') {
                Some(i)
            } else {
                None
            }
        })
        .collect_vec()
}

fn get_stars(board: &Board) -> HashSet<Position> {
    board
        .indexed_iter()
        .filter_map(|(pos, c)| {
            if *c == '#' {
                Some(Position::new(pos.1, pos.0))
            } else {
                None
            }
        })
        .collect()
}

fn expand_stars(stars: &HashSet<Position>, board: &Board, multiplier: usize) -> HashSet<Position> {
    let col_additions = additions(Axis(0), board);
    let row_additions = additions(Axis(1), board);
    let stars: HashSet<Position> = stars
        .iter()
        .map(|pos| {
            Position::new(
                pos.x + col_additions.iter().filter(|v| **v < pos.x).count() * multiplier,
                pos.y + row_additions.iter().filter(|v| **v < pos.y).count() * multiplier,
            )
        })
        .collect();
    stars
}

#[allow(dead_code)]
fn pad_board(stars: &HashSet<Position>, board: &Board, extra_height: usize, extra_width: usize) {
    let mut new_board = Array2::from_elem(board.raw_dim() + Dim([extra_height, extra_width]), '.');
    for star in stars {
        new_board[star.to_index()] = '#';
    }
}

fn parts(input: &str, multiplier: usize) -> usize {
    let board = parse_board(input);
    let stars = get_stars(&board);
    let stars = expand_stars(&stars, &board, multiplier);

    stars
        .iter()
        .tuple_combinations()
        .map(|(a, b)| a.manhattan(b))
        .sum()
}

fn part1(input: &str) -> usize {
    parts(input, 1)
}

fn part2(input: &str) -> usize {
    parts(input, 1000000 - 1)
}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(11)?;
    dbg!(part1(input));
    dbg!(part2(input));

    Ok(())
}

#[test]
fn example() {
    let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
    assert_eq!(part1(input), 374);
    assert_eq!(parts(input, 100 - 1), 8410);
}

#[test]
fn task() {
    let input = &read_input_to_string(11).unwrap();
    assert_eq!(part1(input), 9742154);
    assert_eq!(part2(input), 411142919886);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = &read_input_to_string(11).unwrap();
        part1(input);
        part2(input);
    })
}
