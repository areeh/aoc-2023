extern crate test;

use std::collections::{HashMap, HashSet};
use std::ops::{Add, AddAssign, RangeInclusive, Sub};

use crate::day17::Direction::{Down, Left, Right, Up};
use ndarray::Array2;
use priority_queue::PriorityQueue;
#[cfg(test)]
use test::Bencher;

use crate::utils::read_input_to_string;

pub(crate) fn parse_board(input: &str) -> Array2<u32> {
    let board_width = input.lines().next().unwrap().len();

    let mut data = Vec::new();
    for line in input.lines() {
        let mut row: Vec<u32> = line
            .trim()
            .chars()
            .map(|c| c.to_digit(10))
            .collect::<Option<_>>()
            .unwrap();
        data.append(&mut row);
    }

    let data_len = data.len();
    let n_rows = data_len / board_width;

    Array2::from_shape_vec((n_rows, board_width), data).unwrap()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl Direction {
    fn opposite_direction(&self) -> Direction {
        match self {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
        }
    }

    fn rot(&self) -> Direction {
        match self {
            Up => Right,
            Left => Up,
            Down => Left,
            Right => Down,
        }
    }

    fn counter_rot(&self) -> Direction {
        match self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }
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

    fn manhattan(&self, other: &Self) -> usize {
        ((self.x as isize - other.x as isize).abs() + (self.y as isize - other.y as isize).abs())
            as usize
    }

    fn distance_lower_bound(&self, other: &Self) -> usize {
        self.manhattan(other)
    }

    fn try_add(self, dir: Direction, bounds: Position) -> Result<Position, &'static str> {
        let (new_x, new_y) = match dir {
            Up => (
                self.x,
                self.y.checked_sub(1).ok_or("Underflow in y-coordinate")?,
            ),
            Left => (
                self.x.checked_sub(1).ok_or("Underflow in x-coordinate")?,
                self.y,
            ),
            Down => (
                self.x,
                self.y.checked_add(1).ok_or("Overflow in y-coordinate")?,
            ),
            Right => (
                self.x.checked_add(1).ok_or("Overflow in x-coordinate")?,
                self.y,
            ),
        };

        if new_x <= bounds.x && new_y <= bounds.y {
            Ok(Position { x: new_x, y: new_y })
        } else {
            Err("New position is out of bounds")
        }
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

fn crucible_movement(
    pos: Position,
    distances: RangeInclusive<usize>,
    headings: [Direction; 2],
    upper_bound: Position,
) -> impl Iterator<Item = (Position, Direction)> {
    headings.into_iter().flat_map(move |dir| {
        distances.clone().filter_map(move |distance| {
            let mut pos = pos;
            for _ in 0..distance {
                if let Ok(new_pos) = pos.try_add(dir, upper_bound) {
                    pos = new_pos;
                } else {
                    return None;
                }
            }
            Some((pos, dir))
        })
    })
}

fn parts(input: &str, ultra: bool) -> usize {
    let distances = if ultra { 4..=10 } else { 1..=3 };

    let costs = &parse_board(input);

    let mut queue: PriorityQueue<(Position, Direction), isize> = PriorityQueue::new();
    let mut closed: HashSet<(Position, Direction)> = HashSet::new();
    let mut best_cost: HashMap<(Position, Direction), usize> = HashMap::new();

    let goal = Position::new(costs.dim().1 - 1, costs.dim().0 - 1);
    let start = Position::new(0, 0);

    for dir in [Right, Down] {
        queue.push((start, dir), 0);
        best_cost.insert((start, dir), 0);
    }

    while let Some(((pos, heading), _)) = queue.pop() {
        if pos == goal {
            return best_cost[&(pos, heading)];
        }

        closed.insert((pos, heading));

        for (new_pos, new_heading) in crucible_movement(
            pos,
            distances.clone(),
            [heading.counter_rot(), heading.rot()],
            goal,
        ) {
            if closed.contains(&(new_pos, new_heading)) {
                continue;
            }

            let mut cost = best_cost[&(pos, heading)];
            let mut tmp_pos = new_pos;
            while tmp_pos != pos {
                cost += costs[tmp_pos.to_index()] as usize;
                tmp_pos += new_heading.opposite_direction();
            }

            if cost
                < *best_cost
                    .get(&(new_pos, new_heading))
                    .unwrap_or(&usize::MAX)
            {
                best_cost.insert((new_pos, new_heading), cost);
                let new_priority = -((cost + new_pos.distance_lower_bound(&goal)) as isize);
                queue.push_increase((new_pos, new_heading), new_priority);
            }
        }
    }
    panic!("Goal not found")
}

fn part1(input: &str) -> usize {
    parts(input, false)
}

fn part2(input: &str) -> usize {
    parts(input, true)
}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(17)?;
    dbg!(part1(input));
    dbg!(part2(input));

    Ok(())
}

#[test]
fn example() {
    let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
    assert_eq!(part1(input), 102);
    assert_eq!(part2(input), 94);
}

#[test]
fn example2() {
    let input = "111111111111
999999999991
999999999991
999999999991
999999999991";
    assert_eq!(part2(input), 71);
}

#[test]
fn task() {
    let input = &read_input_to_string(17).unwrap();
    assert_eq!(part1(input), 870);
    assert_eq!(part2(input), 1063);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = &read_input_to_string(17).unwrap();
        part1(input);
        part2(input);
    })
}
