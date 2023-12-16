extern crate test;

use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use ahash::AHasher;
use ndarray::{s, Array1, Array2, ArrayView1, ArrayView2, Axis};
#[cfg(test)]
use test::Bencher;

use crate::utils::{parse_board, read_input_to_string, rot270};

type Board = Array2<char>;

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

fn slide(arr: &ArrayView1<char>) -> Array1<char> {
    let mut arr = arr.to_owned();
    arr.invert_axis(Axis(0));
    let (pos, _, _) =
        arr.indexed_iter()
            .fold((vec![], 0, 0), |(mut v, count_rock, count_open), (i, c)| {
                let mut ret = match c {
                    '.' => (v, count_rock, count_open + 1),
                    'O' => (v, count_rock + 1, count_open + 1),
                    '#' => {
                        if count_rock > 0 {
                            v.push((count_rock, count_open, i));
                        }
                        (v, 0, 0)
                    }
                    _ => panic!("bad char {c}"),
                };
                if i == arr.len() - 1 && ret.1 > 0 {
                    ret.0.push((ret.1, ret.2, i + 1));
                }
                ret
            });
    let mut out_arr = arr.into_owned();
    for (count_rock, count_open, end) in pos {
        out_arr.slice_mut(s![end - count_open..end]).fill('.');
        out_arr.slice_mut(s![end - count_rock..end]).fill('O');
    }
    out_arr.invert_axis(Axis(0));
    out_arr
}

fn load(board: &Board) -> usize {
    let sz = board.dim().1;
    board
        .indexed_iter()
        .filter_map(|((y, _), c)| if *c == 'O' { Some(sz - y) } else { None })
        .sum()
}

fn part1(input: &str) -> usize {
    let mut board = parse_board(input);
    for mut lane in board.lanes_mut(Axis(0)) {
        let new_lane = slide(&lane.view());
        lane.assign(&new_lane);
    }
    load(&board)
}

fn spin_board(board: &mut Board) {
    for _ in 0..4 {
        for mut lane in board.lanes_mut(Axis(0)) {
            let new_lane = slide(&lane.view());
            lane.assign(&new_lane);
        }
        rot270(board);
    }
}

fn array_key(arr: &ArrayView2<char>) -> u64 {
    let mut hasher = AHasher::default();
    arr.iter().collect::<String>().hash(&mut hasher);
    hasher.finish()
}

fn part2(input: &str) -> usize {
    let num_spins = 1000000000;
    let mut board = parse_board(input);
    let mut initial_board = board.to_owned();

    let mut seen: HashMap<u64, usize> = HashMap::new();

    let (mut cycle_start, mut cycle_len) = (None, None);
    for cycle in 0..num_spins {
        spin_board(&mut board);

        let key = array_key(&board.view());
        if let Entry::Vacant(e) = seen.entry(key) {
            e.insert(cycle);
        } else {
            (cycle_start, cycle_len) = (Some(seen[&key]), Some(cycle - seen[&key]));
            break;
        }
    }

    let (cycle_start, cycle_len) = (cycle_start.unwrap(), cycle_len.unwrap());
    let cycle_pos = (num_spins - cycle_start) % cycle_len;
    let cycle_return = cycle_pos + cycle_start;

    for _ in 0..cycle_return {
        spin_board(&mut initial_board);
    }

    load(&initial_board)
}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(14)?;
    dbg!(part1(input));
    dbg!(part2(input));

    Ok(())
}

#[test]
fn example_load() {
    let input = "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....";
    assert_eq!(load(&parse_board(input)), 136);
}

#[test]
fn example() {
    let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
    assert_eq!(part1(input), 136);
    assert_eq!(part2(input), 64);
}

#[test]
fn task() {
    let input = &read_input_to_string(14).unwrap();
    assert_eq!(part1(input), 109654);
    assert_eq!(part2(input), 94876);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = &read_input_to_string(14).unwrap();
        part1(input);
        part2(input);
    })
}
