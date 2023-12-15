extern crate test;

use itertools::Itertools;

use ndarray::{s, Array2, ArrayView2, Zip};
#[cfg(test)]
use test::Bencher;

use crate::utils::read_input_to_string;

#[allow(dead_code)]
fn pretty_string(arr: &ArrayView2<char>) -> String {
    let mut result = String::new();
    for row in arr.rows() {
        for elem in row {
            result.push(*elem);
        }
        result.push('\n');
    }

    result.trim_end().to_owned()
}
#[allow(dead_code)]
fn pretty_print(arr: &ArrayView2<char>) {
    println!("{}", pretty_string(arr));
}

fn parse_board(input: &str) -> Array2<char> {
    let board_width = input.lines().next().unwrap().len();

    let mut data = Vec::new();
    for line in input.lines() {
        let mut row: Vec<_> = line.trim().chars().collect_vec();
        data.append(&mut row);
    }

    let data_len = data.len();
    let n_rows = data_len / board_width;

    Array2::from_shape_vec((n_rows, board_width), data).unwrap()
}

fn find_mirror(board: &ArrayView2<char>, diff_count: usize) -> usize {
    let mut v = 0;
    for i in 1..board.dim().1 {
        let len = i.min(board.dim().1 - i);
        let left = board.slice(s![.., i - len..i]);
        let right = board.slice(s![.., i..i + len; -1]);

        if Zip::from(&left).and(&right).fold(
            0,
            |count, a, b| {
                if a != b {
                    count + 1
                } else {
                    count
                }
            },
        ) == diff_count
        {
            v += i;
        }
    }
    v
}

fn parts(input: &str, diff_count: usize) -> usize {
    let boards = input.split("\n\n").map(parse_board).collect_vec();
    let mut val = 0;

    for board in boards {
        val += find_mirror(&board.view(), diff_count);
        val += find_mirror(&board.t(), diff_count) * 100;
    }

    val
}

fn part1(input: &str) -> usize {
    parts(input, 0)
}

fn part2(input: &str) -> usize {
    parts(input, 1)
}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(13)?;
    dbg!(part1(input));
    dbg!(part2(input));

    Ok(())
}

#[test]
fn example() {
    let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
    assert_eq!(part1(input), 405);
    assert_eq!(part2(input), 400);
}

#[test]
fn task() {
    let input = &read_input_to_string(13).unwrap();
    assert_eq!(part1(input), 37025);
    assert_eq!(part2(input), 32854);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = &read_input_to_string(13).unwrap();
        part1(input);
        part2(input);
    })
}
