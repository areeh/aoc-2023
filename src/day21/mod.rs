extern crate test;

use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::{Add, AddAssign, Sub};

use crate::day21::Direction::{Down, Left, Right, Up};
use ndarray::{s, Array2, Dim};
#[cfg(test)]
use test::Bencher;

use crate::utils::{parse_board, pretty_print, read_input_to_string};

type Board = Array2<char>;

fn pad(arr: &Board, value: char) -> Board {
    // janky pad implementation
    let mut board = Array2::from_elem(arr.raw_dim() + Dim([2, 2]), value);
    board
        .slice_mut(s![1..board.shape()[0] - 1, 1..board.shape()[1] - 1])
        .assign(arr);
    board
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

    fn from_index(ix: (usize, usize)) -> Self {
        Self::new(ix.1 as isize, ix.0 as isize)
    }

    fn to_index(self) -> [usize; 2] {
        [self.y as usize, self.x as usize]
    }

    fn to_index_wrapped(self, board_size: (usize, usize)) -> [usize; 2] {
        let wrapped_x = (self.x.rem_euclid(board_size.0 as isize)) as usize;
        let wrapped_y = (self.y.rem_euclid(board_size.1 as isize)) as usize;
        [wrapped_y, wrapped_x]
    }

    fn board_id(&self, board_size: (usize, usize)) -> (isize, isize) {
        (
            self.x.div_euclid(board_size.1 as isize),
            self.y.div_euclid(board_size.0 as isize),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
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

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Position {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

const ROCK: char = '#';
const PLOT: char = '.';
const P2_STEPS: usize = 26501365;

const DIRS: [Direction; 4] = [Up, Left, Down, Right];

fn compute_at_n(n: usize, deltas: &[usize], start: usize) -> usize {
    if n == 0 {
        return start;
    }

    if deltas[0] == deltas[1] {
        start + deltas[0] * n
    } else {
        let increase = deltas[1] - deltas[0];
        let n = n + 1;
        start + n * (n - 1) / 2 * increase + (n - 1) * (deltas[0] - increase)
    }
}

fn bfs(
    board: &mut Array2<char>,
    starts: &[Position],
    moves: usize,
    visited: &mut HashSet<Position>,
) -> usize {
    let board_dim = board.dim();
    let mut queue = VecDeque::new();

    let s_idx = board
        .indexed_iter()
        .find_map(|(ix, c)| if *c == 'S' { Some(ix) } else { None })
        .unwrap();
    board[s_idx] = PLOT;

    for start in starts {
        queue.push_back(*start);
    }

    let mut level_size = 1;
    for _ in 1..moves + 1 {
        level_size = queue.len();
        visited.clear();
        for _ in 0..level_size {
            let current = queue.pop_front().unwrap();

            for dir in DIRS {
                let next_pos = current + dir;
                if !visited.contains(&next_pos)
                    && board[next_pos.to_index_wrapped(board_dim)] == PLOT
                {
                    visited.insert(next_pos);
                    queue.push_back(next_pos);
                }
            }
        }
    }
    visited.len()
}

fn bfs_use_deltas(
    board: &mut Array2<char>,
    starts: &[Position],
    moves: usize,
    true_moves: usize,
    visited: &mut HashSet<Position>,
    p2: bool,
) -> usize {
    let board_dim = board.dim();
    let true_moves_cycle = true_moves % board_dim.0;
    let mut queue = VecDeque::new();

    let s_idx = board
        .indexed_iter()
        .find_map(|(ix, c)| if *c == 'S' { Some(ix) } else { None })
        .unwrap();
    board[s_idx] = PLOT;

    for start in starts {
        queue.push_back(*start);
    }

    let mut counts_per_count_vec: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();

    let mut level_size = 1;
    for step in 1..moves + 1 {
        level_size = queue.len();
        visited.clear();
        for _ in 0..level_size {
            let current = queue.pop_front().unwrap();

            for dir in DIRS {
                let next_pos = current + dir;
                if !visited.contains(&next_pos)
                    && board[next_pos.to_index_wrapped(board_dim)] == PLOT
                {
                    visited.insert(next_pos);
                    queue.push_back(next_pos);
                }
            }
        }

        let min_repeat = if p2 { 2 } else { 3 };
        if (step / board_dim.0) > min_repeat {
            let mut count_per_board: HashMap<(isize, isize), usize> = HashMap::new();

            visited.iter().for_each(|pos| {
                let board_id = pos.board_id(board.dim());
                *count_per_board.entry(board_id).or_insert(0) += 1;
            });

            let mut counts_per_count = HashMap::new();

            count_per_board.values().for_each(|count| {
                *counts_per_count.entry(count).or_insert(0) += 1;
            });
            if step % board_dim.0 == true_moves_cycle {
                counts_per_count.iter().for_each(|(count, occ)| {
                    counts_per_count_vec
                        .entry(**count)
                        .or_default()
                        .push((*occ, step));
                });
            }
        }
    }

    let res: HashMap<usize, usize> = counts_per_count_vec
        .iter()
        .map(|(count, v)| {
            let deltas = v
                .iter()
                .tuple_windows()
                .map(|((a, _), (b, _))| b - a)
                .collect_vec();

            (
                *count,
                compute_at_n((true_moves - v[0].1) / board_dim.0, &deltas, v[0].0),
            )
        })
        .collect();

    let total_visited: usize = res.iter().map(|(count, repeats)| count * repeats).sum();
    total_visited
}

#[allow(dead_code)]
fn visualize(board: &Board, reached: &[Position]) {
    let mut board = board.clone();

    for pos in reached {
        board[pos.to_index()] = 'O';
    }

    pretty_print(&board.view());
}

fn walled_board(board: &Board, starts: &[Position], steps: usize) -> usize {
    let mut board = pad(board, ROCK);
    let mut visited = HashSet::new();
    bfs(&mut board, starts, steps, &mut visited)
}

fn part1(input: &str, steps: usize) -> usize {
    let board = parse_board(input);
    let start = board
        .indexed_iter()
        .find_map(|(ix, c)| {
            if *c == 'S' {
                Some(Position::from_index(ix))
            } else {
                None
            }
        })
        .unwrap()
        + Down
        + Right;

    walled_board(&board, &[start], steps)
}

fn part2(input: &str, steps: usize, true_steps: usize, p2_map: bool) -> usize {
    let mut board = parse_board(input);
    let start = board
        .indexed_iter()
        .find_map(|(ix, c)| {
            if *c == 'S' {
                Some(Position::from_index(ix))
            } else {
                None
            }
        })
        .unwrap();
    let mut visited = HashSet::new();
    bfs_use_deltas(
        &mut board,
        &[start],
        steps,
        true_steps,
        &mut visited,
        p2_map,
    )
}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(21)?;
    dbg!(part1(input, 64));
    dbg!(part2(input, 721, P2_STEPS, true));

    Ok(())
}

#[test]
fn example() {
    let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";

    assert_eq!(part1(input, 6), 16);
    assert_eq!(part2(input, 100, 100, false), 6536);
    assert_eq!(part2(input, 100, 500, false), 167004);
    assert_eq!(part2(input, 100, 1000, false), 668697);
}

#[test]
fn task() {
    let input = &read_input_to_string(21).unwrap();
    assert_eq!(part1(input, 64), 3724);
    assert_eq!(part2(input, 721, P2_STEPS, true), 620348631910321);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = &read_input_to_string(21).unwrap();
        // part1(input, 64);
        // part2(input, 721, P2_STEPS, true);
    })
}
