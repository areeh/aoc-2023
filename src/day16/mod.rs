extern crate test;

use std::collections::{HashSet, VecDeque};
use std::ops::{Add, AddAssign, Sub};

use crate::day16::Direction::{Down, Left, Right, Up};
use ndarray::{s, Array2, Dim};
#[cfg(test)]
use test::Bencher;

use crate::utils::{parse_board, read_input_to_string};

type Board = Array2<char>;

impl Direction {
    fn opposite_direction(&self) -> Direction {
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }

    fn left_mirror(&self) -> Direction {
        // This mirror:  \
        match self {
            Up => Right,
            Left => Down,
            Down => Left,
            Right => Up,
        }
    }

    fn right_mirror(&self) -> Direction {
        // This mirror:  /
        match self {
            Up => Left,
            Left => Up,
            Down => Right,
            Right => Down,
        }
    }

    fn vertical_splitter(&self) -> Exit {
        match self {
            Up => Exit::Single(Down),
            Left => Exit::Split(Up, Down),
            Down => Exit::Single(Up),
            Right => Exit::Split(Up, Down),
        }
    }

    fn horizontal_splitter(&self) -> Exit {
        match self {
            Up => Exit::Split(Left, Right),
            Left => Exit::Single(Right),
            Down => Exit::Split(Left, Right),
            Right => Exit::Single(Left),
        }
    }
}

fn pad(arr: &Board, value: char) -> Board {
    // janky pad implementation
    let mut board = Array2::from_elem(arr.raw_dim() + Dim([2, 2]), ' ');
    board.fill(value);
    board
        .slice_mut(s![1..board.shape()[0] - 1, 1..board.shape()[1] - 1])
        .assign(arr);
    board
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

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

#[derive(Debug)]
enum Exit {
    Single(Direction),
    Split(Direction, Direction),
    OutOfBounds,
}

fn beam_encounter(entry_dir: Direction, c: char) -> Exit {
    match c {
        '.' => Exit::Single(entry_dir.opposite_direction()),
        '/' => Exit::Single(entry_dir.right_mirror()),
        '\\' => Exit::Single(entry_dir.left_mirror()),
        '-' => entry_dir.horizontal_splitter(),
        '|' => entry_dir.vertical_splitter(),
        ' ' => Exit::OutOfBounds,
        _ => panic!("bad char in cell {c}"),
    }
}

type BeamExits = Array2<HashSet<Direction>>;

fn maybe_push(
    pos: Position,
    exit_dir: Direction,
    beam_exits: &mut BeamExits,
    beams: &mut VecDeque<(Position, Direction)>,
) {
    let visited = &mut beam_exits[pos.to_index()];
    if !visited.contains(&exit_dir) {
        visited.insert(exit_dir);
        beams.push_back((pos + exit_dir, exit_dir.opposite_direction()))
    }
}

fn beaming_to_energized(start: (Position, Direction), board: &Board) -> usize {
    let mut beam_exits: BeamExits = Array2::from_elem(board.raw_dim(), HashSet::new());
    let mut beams: VecDeque<(Position, Direction)> = VecDeque::new();

    beams.push_back(start);

    while let Some((pos, dir)) = beams.pop_front() {
        let exit = beam_encounter(dir, board[pos.to_index()]);

        match exit {
            Exit::Split(a, b) => {
                maybe_push(pos, a, &mut beam_exits, &mut beams);
                maybe_push(pos, b, &mut beam_exits, &mut beams);
            }
            Exit::Single(dir) => maybe_push(pos, dir, &mut beam_exits, &mut beams),
            Exit::OutOfBounds => (),
        }
    }

    beam_exits.iter().filter(|e| !e.is_empty()).count()
}

fn part1(input: &str) -> usize {
    let board = parse_board(input);
    let board = pad(&board, ' ');
    beaming_to_energized((Position::new(1, 1), Left), &board)
}

fn edge_positions(board: &Board) -> impl Iterator<Item = Position> + '_ {
    (0..board.dim().0)
        .map(|v| Position::new(0, v))
        .chain((0..board.dim().0).map(|v| Position::new(board.dim().1 - 1, v)))
        .chain((0..board.dim().1).map(|v| Position::new(v, 0)))
        .chain((0..board.dim().1).map(|v| Position::new(v, board.dim().0 - 1)))
}

fn start_directions(start_pos: Position, board: &Board) -> Vec<Direction> {
    let mut out = Vec::new();

    if start_pos.x == 1 {
        out.push(Left);
    } else if start_pos.x == board.dim().1 - 2 {
        out.push(Right);
    }

    if start_pos.y == 1 {
        out.push(Up);
    } else if start_pos.y == board.dim().0 - 2 {
        out.push(Down);
    }

    out
}

fn part2(input: &str) -> usize {
    let board = parse_board(input);
    let edge_pos_iter = edge_positions(&board);

    let board = pad(&board, ' ');
    let mut energized: Vec<(Position, usize)> = Vec::new();

    for pos in edge_pos_iter.map(|pos| pos + Position::new(1, 1)) {
        for dir in start_directions(pos, &board) {
            energized.push((pos, beaming_to_energized((pos, dir), &board)));
        }
    }

    *energized.iter().map(|(_, v)| v).max().unwrap()
}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(16)?;
    dbg!(part1(input));
    dbg!(part2(input));

    Ok(())
}

#[test]
fn example() {
    let input = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|...."#;
    assert_eq!(part1(input), 46);
    assert_eq!(part2(input), 51);
}

#[test]
fn task() {
    let input = &read_input_to_string(16).unwrap();
    assert_eq!(part1(input), 6883);
    assert_eq!(part2(input), 7228);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = &read_input_to_string(16).unwrap();
        part1(input);
        part2(input);
    })
}
