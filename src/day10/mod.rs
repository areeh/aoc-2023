extern crate test;

use itertools::Itertools;
use std::collections::HashSet;
use std::ops::{Add, AddAssign, Sub};

use ndarray::Array2;
#[cfg(test)]
use test::Bencher;

use crate::utils::{parse_board, pretty_print, read_input_to_string};

const DIRS: [Direction; 4] = [
    Direction::Up,
    Direction::Left,
    Direction::Down,
    Direction::Right,
];

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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    fn is_perpendicular(&self, other: Direction) -> bool {
        let (a, b) = match self {
            Direction::Up => (Direction::Left, Direction::Right),
            Direction::Left => (Direction::Up, Direction::Down),
            Direction::Down => (Direction::Left, Direction::Right),
            Direction::Right => (Direction::Up, Direction::Down),
        };
        other == a || other == b
    }
}

impl Add<Direction> for Position {
    type Output = Position;

    fn add(self, dir: Direction) -> Position {
        match dir {
            Direction::Up => Position {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Left => Position {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Down => Position {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Right => Position {
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

fn to_2d_pos(pos_1d: usize, line_length: usize) -> Position {
    let y = pos_1d / line_length;
    Position::new(pos_1d - y * line_length - y, y)
}

fn pipe_char_to_directions(c: char) -> Option<(Direction, Direction)> {
    match c {
        '|' => Some((Direction::Up, Direction::Down)),
        '-' => Some((Direction::Left, Direction::Right)),
        'L' => Some((Direction::Up, Direction::Right)),
        'J' => Some((Direction::Up, Direction::Left)),
        '7' => Some((Direction::Down, Direction::Left)),
        'F' => Some((Direction::Down, Direction::Right)),
        _ => None,
    }
}

fn directions_to_pipe_char(directions: (Direction, Direction)) -> Option<char> {
    match directions {
        (Direction::Up, Direction::Down) | (Direction::Down, Direction::Up) => Some('|'),
        (Direction::Left, Direction::Right) | (Direction::Right, Direction::Left) => Some('-'),
        (Direction::Up, Direction::Right) | (Direction::Right, Direction::Up) => Some('L'),
        (Direction::Up, Direction::Left) | (Direction::Left, Direction::Up) => Some('J'),
        (Direction::Down, Direction::Left) | (Direction::Left, Direction::Down) => Some('7'),
        (Direction::Down, Direction::Right) | (Direction::Right, Direction::Down) => Some('F'),
        _ => None,
    }
}

fn traverse_pipe(incoming: Direction, (d1, d2): (Direction, Direction)) -> Direction {
    if d1 == incoming {
        d2
    } else if d2 == incoming {
        d1
    } else {
        panic!("Bad pipe traversal incoming {incoming:?} pipe ({d1:?}, {d2:?})")
    }
}

fn opposite_direction(direction: Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Down,
        Direction::Down => Direction::Up,
        Direction::Left => Direction::Right,
        Direction::Right => Direction::Left,
    }
}

fn pipe_step(pos: Position, entry_dir: Direction, board: &Board) -> (Position, Direction) {
    if let Some(pipe) = pipe_char_to_directions(board[pos.to_index()]) {
        let exit_direction = traverse_pipe(entry_dir, pipe);
        (pos + exit_direction, opposite_direction(exit_direction))
    } else {
        panic!(
            "Bad pipe char at Position {:?} {}",
            pos,
            board[pos.to_index()]
        )
    }
}

#[allow(dead_code)]
fn visualize(board: &Board, inside: &[Position]) -> String {
    let mut board = board.clone();

    for pos in inside {
        board[pos.to_index()] = 'I'
    }
    pretty_print(&board)
}

fn part1(input: &str) -> usize {
    let lines = input.lines().collect_vec();
    let start_pos = to_2d_pos(input.find(|v| v == 'S').unwrap(), lines[0].len());
    let board = parse_board(input);

    let mut moves: Vec<(Position, Direction)> = DIRS
        .into_iter()
        .filter_map(|dir| {
            if (start_pos.x == 0 && dir == Direction::Left)
                || (start_pos.y == 0 && dir == Direction::Up)
            {
                return None;
            }

            let new_pos = start_pos + dir;
            if let Some(c) = board.get(new_pos.to_index()) {
                if let Some(pipe) = pipe_char_to_directions(*c) {
                    let entry_dir = opposite_direction(dir);
                    if entry_dir == pipe.0 || entry_dir == pipe.1 {
                        return Some((new_pos, opposite_direction(dir)));
                    }
                }
            }
            None
        })
        .collect();

    let mut step = 1;
    while moves.iter().duplicates_by(|(pos, _)| pos).next().is_none() {
        moves
            .iter_mut()
            .for_each(|mov| *mov = pipe_step(mov.0, mov.1, &board));
        step += 1;
    }
    step
}

fn direction_to_nearest_edge(pos: Position, grid_dimensions: Position) -> Direction {
    let distance_to_top = pos.y;
    let distance_to_bottom = grid_dimensions.y - pos.y - 1;
    let distance_to_left = pos.x;
    let distance_to_right = grid_dimensions.x - pos.x - 1;

    let min_distance = [
        distance_to_bottom,
        distance_to_left,
        distance_to_right,
        distance_to_left,
    ]
    .into_iter()
    .min()
    .unwrap();

    if min_distance == distance_to_top {
        Direction::Up
    } else if min_distance == distance_to_bottom {
        Direction::Down
    } else if min_distance == distance_to_left {
        Direction::Left
    } else {
        Direction::Right
    }
}

fn is_inside(position: Position, board: &Board, grid_dimensions: Position, dir: Direction) -> bool {
    let mut position = position;
    let mut intersections = 0;
    let mut par_entry_dir: Option<Direction> = None;
    while position.x > 0
        && position.x < (grid_dimensions.x - 1)
        && position.y > 0
        && position.y < (grid_dimensions.y - 1)
    {
        position += dir;
        if let Some(entry_dir) = par_entry_dir {
            if let Some(pipe) = pipe_char_to_directions(board[position.to_index()]) {
                match (
                    entry_dir.is_perpendicular(pipe.0),
                    entry_dir.is_perpendicular(pipe.1),
                ) {
                    (true, true) => (),
                    (true, false) => {
                        par_entry_dir = None;
                        if pipe.1 == opposite_direction(entry_dir) {
                            intersections += 1;
                        }
                    }
                    (false, true) => {
                        par_entry_dir = None;
                        if pipe.0 == opposite_direction(entry_dir) {
                            intersections += 1;
                        }
                    }
                    (false, false) => {
                        panic!("wot intersecting pipe entry {entry_dir:?}, pipe {pipe:?}")
                    }
                }
            } else {
                panic!(
                    "wot should have pipes until we exit par, got {}",
                    board[position.to_index()]
                )
            }
        } else if let Some(pipe) = pipe_char_to_directions(board[position.to_index()]) {
            match (dir.is_perpendicular(pipe.0), dir.is_perpendicular(pipe.1)) {
                (true, true) => intersections += 1,
                (true, false) => par_entry_dir = Some(pipe.0),
                (false, true) => par_entry_dir = Some(pipe.1),
                (false, false) => panic!("Should have entered par logic"),
            }
        }
    }
    intersections % 2 != 0
}

fn part2(input: &str) -> usize {
    let lines = input.lines().collect_vec();
    let start_pos = to_2d_pos(input.find(|v| v == 'S').unwrap(), lines[0].len());
    let board = parse_board(input);

    let moves: Vec<(Position, Direction)> = DIRS
        .into_iter()
        .filter_map(|dir| {
            if (start_pos.x == 0 && dir == Direction::Left)
                || (start_pos.y == 0 && dir == Direction::Up)
            {
                return None;
            }

            let new_pos = start_pos + dir;
            if let Some(c) = board.get(new_pos.to_index()) {
                if let Some(pipe) = pipe_char_to_directions(*c) {
                    let entry_dir = opposite_direction(dir);
                    if entry_dir == pipe.0 || entry_dir == pipe.1 {
                        return Some((new_pos, opposite_direction(dir)));
                    }
                }
            }
            None
        })
        .collect();

    let mut moves = moves.into_iter().map(|v| vec![v]).collect_vec();

    while moves
        .iter()
        .map(|v| v.last().unwrap().0)
        .chain(moves.iter().filter_map(|v| {
            if v.len() >= 2 {
                Some(v[v.len() - 2].0)
            } else {
                None
            }
        }))
        .duplicates()
        .next()
        .is_none()
    {
        moves.iter_mut().for_each(|v| {
            let mov = v.last().unwrap();
            v.push(pipe_step(mov.0, mov.1, &board));
        });
    }

    let final_position = moves
        .iter()
        .map(|v| v.last().unwrap().0)
        .duplicates()
        .next()
        .unwrap();

    let cycle = if let Some((mut a_moves, b_moves)) = moves
        .into_iter()
        .filter(|v| v.last().unwrap().0 == final_position)
        .collect_tuple()
    {
        let mut b_moves = b_moves
            .into_iter()
            .rev()
            .map(|(pos, dir)| (pos, opposite_direction(dir)))
            .collect_vec();
        a_moves.append(&mut b_moves);
        a_moves
    } else {
        panic!("wut")
    };

    let mut board = board;

    board[start_pos.to_index()] = directions_to_pipe_char((
        opposite_direction(cycle.first().unwrap().1),
        cycle.last().unwrap().1,
    ))
    .unwrap();

    let mut cycle_set: HashSet<Position> = cycle.into_iter().map(|(pos, _)| pos).collect();
    cycle_set.insert(start_pos);

    for (idx, v) in board.indexed_iter_mut() {
        if !cycle_set.contains(&Position::new(idx.1, idx.0)) {
            *v = '.';
        }
    }

    let inside_positions = board
        .indexed_iter()
        .filter_map(|(pos, c)| {
            if *c == '.'
                && is_inside(
                    Position::new(pos.1, pos.0),
                    &board,
                    Position::new(board.dim().1, board.dim().0),
                    direction_to_nearest_edge(
                        Position::new(pos.1, pos.0),
                        Position::new(board.dim().1, board.dim().0),
                    ),
                )
            {
                Some(Position::new(pos.1, pos.0))
            } else {
                None
            }
        })
        .collect_vec();

    // println!("{}", visualize(&board, &inside_positions));

    inside_positions.len()
}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(10)?;
    dbg!(part1(input));
    dbg!(part2(input));

    Ok(())
}

#[test]
fn example() {
    let input = ".....
.S-7.
.|.|.
.L-J.
.....";
    assert_eq!(part1(input), 4);
    assert_eq!(part2(input), 1);
}

#[test]
fn example2() {
    let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
    assert_eq!(part1(input), 8);
    assert_eq!(part2(input), 1);
}

#[test]
fn example1_p2() {
    let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
    assert_eq!(part2(input), 4);
}
#[test]
fn example2_p2() {
    let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
    assert_eq!(part2(input), 8);
}
#[test]
fn example3_p2() {
    let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";
    assert_eq!(part2(input), 10);
}

#[test]
fn task() {
    let input = &read_input_to_string(10).unwrap();
    assert_eq!(part1(input), 6815);
    assert_eq!(part2(input), 269);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = &read_input_to_string(10).unwrap();
        part1(input);
        part2(input);
    })
}
