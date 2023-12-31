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

fn bfs(board: &mut Array2<char>, starts: &[Position], record_steps: &[usize]) -> Vec<usize> {
    let board_dim = board.dim();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    let s_idx = board
        .indexed_iter()
        .find_map(|(ix, c)| if *c == 'S' { Some(ix) } else { None })
        .unwrap();
    board[s_idx] = PLOT;

    for start in starts {
        queue.push_back(*start);
    }

    let steps = record_steps.iter().max().unwrap();

    let mut out = vec![];
    let mut level_size;
    for step in 1..steps + 1 {
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
        if record_steps.contains(&step) {
            out.push(visited.len())
        }
    }
    out
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
    *bfs(&mut board, starts, &[steps]).first().unwrap()
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

struct StepIterator<T> {
    initial: T,
    step: T,
}

impl<T> StepIterator<T> {
    fn new(start: T, step: T) -> Self {
        StepIterator {
            initial: start,
            step,
        }
    }
}

impl<T> Iterator for StepIterator<T>
where
    T: std::ops::Add<Output = T> + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.initial;
        self.initial = self.initial + self.step;
        Some(result)
    }
}

fn step_by<T>(start: T, step: T) -> StepIterator<T> {
    StepIterator::new(start, step)
}

fn solve_quadratic(points: [(f64, f64); 3]) -> Option<(f64, f64, f64)> {
    let (x1, y1) = points[0];
    let (x2, y2) = points[1];
    let (x3, y3) = points[2];

    let matrix = [[x1 * x1, x1, 1.0], [x2 * x2, x2, 1.0], [x3 * x3, x3, 1.0]];
    let mut y_values = [y1, y2, y3];

    gauss_jordan(&matrix, &mut y_values).map(|coefs| (coefs[0], coefs[1], coefs[2]))
}

fn gauss_jordan(matrix: &[[f64; 3]; 3], y_values: &mut [f64; 3]) -> Option<[f64; 3]> {
    let mut mat = *matrix;

    // Forward elimination
    for i in 0..3 {
        if mat[i][i] == 0.0 {
            return None;
        }

        for j in (i + 1)..3 {
            let ratio = mat[j][i] / mat[i][i];
            for k in 0..3 {
                mat[j][k] -= ratio * mat[i][k];
            }
            y_values[j] -= ratio * y_values[i];
        }
    }

    // Backward substitution
    let mut solution = [0.0; 3];
    for i in (0..3).rev() {
        solution[i] = y_values[i];
        for j in (i + 1)..3 {
            solution[i] -= mat[i][j] * solution[j];
        }
        solution[i] /= mat[i][i];
    }

    Some(solution)
}

fn zip_to_three_point(vec1: Vec<usize>, vec2: Vec<usize>) -> Option<[(f64, f64); 3]> {
    vec1.into_iter()
        .zip(vec2)
        .map(|(a, b)| (a as f64, b as f64))
        .collect::<Vec<_>>() // Convert iterator to Vec
        .try_into() // Try to convert Vec to array
        .ok() // Convert Result to Option
}

fn cross_open(array: &Array2<char>, point: [usize; 2]) -> bool {
    let horizontal_open = array.row(point[0]).iter().all(|&c| c != '#');
    let vertical_open = array.column(point[1]).iter().all(|&c| c != '#');

    horizontal_open && vertical_open
}

fn part2(input: &str, steps: usize) -> usize {
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

    // We're in the center of a square board, with free passages to the edges
    assert!(board.is_square());
    let board_side = board.dim().0;
    assert_eq!(board_side % 2, 1); // Odd size -> has a single center
    let center = Position::new((board_side / 2) as isize, (board_side / 2) as isize);
    assert_eq!(center, start);
    assert!(cross_open(&board, start.to_index())); // Free passage to the edges

    // Three point method for solving a quadratic
    let to_edge_steps = board_side - 1 - start.x as usize;
    let sample_points = step_by(to_edge_steps, board_side).take(3).collect_vec();
    let sample_points_reachable = bfs(&mut board, &[start], &sample_points);
    let samples = zip_to_three_point(sample_points, sample_points_reachable).unwrap();
    let out = solve_quadratic(samples).unwrap();

    (out.0 * (steps as f64).powi(2) + out.1 * steps as f64 + out.2).round() as usize
}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(21)?;
    dbg!(part1(input, 64));
    dbg!(part2(input, P2_STEPS));

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
    assert_eq!(part2(input, 100), 6536);
    assert_eq!(part2(input, 500), 167004);
    assert_eq!(part2(input, 1000), 668697);
}

#[test]
fn task() {
    let input = &read_input_to_string(21).unwrap();
    assert_eq!(part1(input, 64), 3724);
    assert_eq!(part2(input, P2_STEPS), 620348631910321);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = &read_input_to_string(21).unwrap();
        part1(input, 64);
        part2(input, P2_STEPS);
    })
}
