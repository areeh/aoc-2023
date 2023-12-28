extern crate test;

use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::ops::{Add, AddAssign, Sub};

use crate::day23::Direction::{Down, Left, Right, Up};
use ndarray::{s, Array2, Dim};
use petgraph::algo::all_simple_paths;
use petgraph::dot::Dot;
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::Graph;
#[cfg(test)]
use test::Bencher;

use crate::utils::{parse_board, pretty_string, read_input_to_string};

type Board = Array2<char>;

const DIRS: [Direction; 4] = [Up, Left, Down, Right];

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

fn arrow_dir(c: char) -> Option<Direction> {
    match c {
        '^' => Some(Up),
        '>' => Some(Right),
        '<' => Some(Left),
        'v' => Some(Down),
        _ => None,
    }
}

const STEP_CHARS: [char; 5] = ['.', '^', '<', '>', 'v'];
const ARROWS: [char; 4] = ['^', '<', '>', 'v'];

fn valid_neighbors(pos: Position, board: &Board) -> Vec<Position> {
    let dirs = if let Some(dir) = arrow_dir(board[pos.to_index()]) {
        vec![dir]
    } else {
        DIRS.to_vec()
    };

    dirs.iter()
        .filter_map(|dir| {
            let next_pos = pos + *dir;
            let next_char = board[next_pos.to_index()];
            if STEP_CHARS.contains(&next_char) {
                Some(next_pos)
            } else if next_char == '#' {
                None
            } else {
                panic!("bad next char {next_char}")
            }
        })
        .collect()
}

fn fast_forward(pos: Position, from_pos: Position, board: &Board) -> (Position, usize) {
    let mut prev_pos = from_pos;
    let mut current_pos = pos;
    let mut distance = 0;
    while let [pos_a, pos_b] = valid_neighbors(current_pos, board)[..] {
        let tmp = current_pos;
        current_pos = if prev_pos == pos_a { pos_b } else { pos_a };
        prev_pos = tmp;
        distance += 1;
    }
    (current_pos, distance)
}

fn valid_destinations(
    pos: Position,
    board: &Board,
    visited: &HashSet<Position>,
) -> Vec<(Position, usize)> {
    let dirs = if let Some(dir) = arrow_dir(board[pos.to_index()]) {
        vec![dir]
    } else {
        DIRS.to_vec()
    };

    dirs.iter()
        .filter_map(|dir| {
            let next_pos = pos + *dir;
            let next_char = board[next_pos.to_index()];
            if STEP_CHARS.contains(&next_char) {
                let (next_pos, distance) = fast_forward(next_pos, pos, board);
                if visited.contains(&next_pos) {
                    None
                } else {
                    Some((next_pos, distance))
                }
            } else if next_char == '#' {
                None
            } else {
                panic!("bad next char {next_char}")
            }
        })
        .collect()
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

#[allow(dead_code)]
fn visualize(board: &Board, path: &[Position]) {
    let mut board = board.clone();

    for pos in path {
        board[pos.to_index()] = 'O'
    }
    println!("{}", pretty_string(&board.view()));
}


fn path_length(path: &[NodeIndex], graph: &UnGraph<Position, usize>) -> usize {
    path.iter().tuple_windows().fold(0, |acc, (a, b)| {
        if let Some(edge) = graph.find_edge(*a, *b) {
            acc + graph[edge]
        } else {
            panic!("wut")
        }
    })
}

fn longest_path(start: Position, goal: Position, board: &Board) -> usize {
    let mut visited: HashSet<Position> = HashSet::new();
    let mut node_map: HashMap<Position, NodeIndex> = HashMap::new();
    let mut g: UnGraph<Position, usize> = Graph::new_undirected();

    let mut queue: VecDeque<Position> = VecDeque::new();
    queue.push_back(start);

    while let Some(pos) = queue.pop_front() {
        if visited.contains(&pos) {
            continue;
        }

        visited.insert(pos);

        node_map.entry(pos).or_insert_with(|| g.add_node(pos));
        for (dest, distance) in valid_destinations(pos, board, &visited) {
            node_map.entry(dest).or_insert_with(|| g.add_node(dest));
            g.add_edge(node_map[&pos], node_map[&dest], distance + 1);
            queue.push_back(dest)
        }
    }

    println!("{:?}", Dot::with_config(&g, &[]));

    all_simple_paths::<Vec<_>, _>(
        &g,
        node_map[&start],
        node_map[&goal],
        g.node_count() - 4,
        None,
    )
    .map(|path| path_length(&path, &g))
    .max()
    .unwrap()
}

fn remove_cells_with_one_neighbor(board: &Board, start: &Position, goal: &Position) -> Board {
    let mut board_out = board.clone();
    for (ix, c) in board.indexed_iter() {
        let pos = Position::new(ix.1, ix.0);
        if *c == '.' && &pos != start && &pos != goal && valid_neighbors(pos, board).len() == 1 {
            board_out[ix] = '#'
        }
    }
    board_out
}

fn remove_dead_ends(board: Board, start: &Position, goal: &Position) -> Board {
    let mut prev_board = board;
    let mut next_board = remove_cells_with_one_neighbor(&prev_board, start, goal);
    while prev_board != next_board {
        prev_board = next_board;
        next_board = remove_cells_with_one_neighbor(&prev_board, start, goal);
    }
    next_board
}

fn parts(board: Board) -> usize {
    let board = pad(&board, '#');

    let start = Position::new(2, 1);
    let goal = Position::new(board.dim().1 - 3, board.dim().0 - 2);

    let board = remove_dead_ends(board, &start, &goal);

    longest_path(start, goal, &board)
}

fn part1(input: &str) -> usize {
    let board = parse_board(input);
    parts(board)
}

fn part2(input: &str) -> usize {
    let board = {
        let mut board = parse_board(input);
        board.map_inplace(|c| {
            if ARROWS.contains(c) {
                *c = '.'
            }
        });
        board
    };

    parts(board)
}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(23)?;
    dbg!(part1(input));
    dbg!(part2(input));

    Ok(())
}

#[test]
fn example() {
    let input = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#";
    // assert_eq!(part1(input), 94);
    assert_eq!(part2(input), 154);
}

#[test]
fn task() {
    let input = &read_input_to_string(23).unwrap();
    // assert_eq!(part1(input), 2154);
    assert_eq!(part2(input), 4686);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = &read_input_to_string(23).unwrap();
        part1(input);
        part2(input);
    })
}
