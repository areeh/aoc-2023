extern crate test;

use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::RangeInclusive;
use std::str::FromStr;

#[cfg(test)]
use test::Bencher;

use crate::utils::read_input_to_string;

use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::Direction;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Position {
    x: usize,
    y: usize,
    z: usize,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Block {
    x: RangeInclusive<usize>,
    y: RangeInclusive<usize>,
    z: RangeInclusive<usize>,
}

impl Block {
    fn slide_z_to(&mut self, new_start: usize) {
        let z_length = self.z.end() - self.z.start();
        self.z = new_start..=new_start + z_length;
    }

    fn intersects_horizontal(&self, other: &Block) -> bool {
        self.x.start() <= other.x.end()
            && self.x.end() >= other.x.start()
            && self.y.start() <= other.y.end()
            && self.y.end() >= other.y.start()
    }
}

impl FromStr for Block {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('~').collect();
        if parts.len() != 2 {
            return Err("Input string must have exactly one '~'".into());
        }

        let start = parse_coordinates(parts[0])?;
        let end = parse_coordinates(parts[1])?;

        Ok(Block {
            x: start.0..=end.0,
            y: start.1..=end.1,
            z: start.2..=end.2,
        })
    }
}

fn parse_coordinates(coord: &str) -> Result<(usize, usize, usize), String> {
    let nums: Vec<&str> = coord.split(',').collect();
    if nums.len() != 3 {
        return Err("Each coordinate part must have exactly three numbers".into());
    }

    let x = nums[0]
        .parse::<usize>()
        .map_err(|_| "Invalid number for x")?;
    let y = nums[1]
        .parse::<usize>()
        .map_err(|_| "Invalid number for y")?;
    let z = nums[2]
        .parse::<usize>()
        .map_err(|_| "Invalid number for z")?;

    Ok((x, y, z))
}
fn can_remove<N, E>(graph: &DiGraph<N, E>, node: NodeIndex) -> bool {
    graph
        .neighbors_directed(node, Direction::Incoming)
        .all(|n| graph.neighbors_directed(n, Direction::Outgoing).count() != 1)
}

fn count_predecessors_with_out_degree_one<N, E>(graph: &DiGraph<N, E>, node: NodeIndex) -> usize {
    // let mut visited = HashSet::new();
    let mut falling = HashSet::new();
    let mut stack = VecDeque::new();
    stack.push_back(node);

    while let Some(nx) = stack.pop_front() {
        if graph
            .neighbors_directed(nx, Direction::Outgoing)
            .all(|nx| falling.contains(&nx))
            || node == nx
        {
            falling.insert(nx);
            for nx in graph.neighbors_directed(nx, Direction::Incoming) {
                stack.push_back(nx);
            }
        }
    }

    falling.remove(&node);
    falling.len()
}
fn parts(input: &str) -> DiGraph<Block, bool> {
    let mut blocks: Vec<Block> = input
        .lines()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap();
    let mut g: DiGraph<Block, bool> = DiGraph::new();
    let mut block_node_map: HashMap<Block, NodeIndex> = HashMap::new();

    blocks.sort_by(|a, b| a.z.start().cmp(b.z.start()));

    for i in 0..blocks.len() {
        let mut maybe_rest_z: Option<usize> = None;
        let mut rests_on: Vec<Block> = vec![];
        for rest_candidate in blocks
            .iter()
            .filter(|candidate| {
                candidate.z.end() < blocks[i].z.start()
                    && blocks[i].intersects_horizontal(candidate)
            })
            .sorted_by(|a, b| b.z.end().cmp(a.z.end()))
        {
            match maybe_rest_z {
                Some(rest_z) => {
                    if *rest_candidate.z.end() + 1 == rest_z {
                        rests_on.push(rest_candidate.clone());
                    } else {
                        break;
                    }
                }
                None => {
                    maybe_rest_z = Some(rest_candidate.z.end() + 1);
                    rests_on.push(rest_candidate.clone());
                }
            }
        }

        if maybe_rest_z.is_none() {
            maybe_rest_z = Some(1)
        }
        let block = blocks.get_mut(i).unwrap();
        block.slide_z_to(maybe_rest_z.unwrap());

        let block_idx = *block_node_map
            .entry(block.clone())
            .or_insert_with(|| g.add_node(block.clone()));
        for rest_block in rests_on.into_iter() {
            let rest_block_idx = *block_node_map
                .entry(rest_block.clone())
                .or_insert_with(|| g.add_node(rest_block.clone()));
            g.add_edge(block_idx, rest_block_idx, false);
        }
    }

    g
}
fn part1(input: &str) -> usize {
    let g = parts(input);

    g.node_indices()
        .filter(|node| can_remove(&g, *node))
        .count()
}

fn part2(input: &str) -> usize {
    let g = parts(input);

    g.node_indices()
        .map(|node| count_predecessors_with_out_degree_one(&g, node))
        .sum()
}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(22)?;
    dbg!(part1(input));
    dbg!(part2(input));

    Ok(())
}

#[test]
fn test_block_parse() {
    let input = "1,0,1~2,3,4";
    let block = input.parse::<Block>();
    assert!(block.is_ok());
    let block = block.unwrap();
    assert_eq!(block.x, 1..=2);
    assert_eq!(block.y, 0..=3);
    assert_eq!(block.z, 1..=4);

    let invalid_input = "1,0~2,3,4";
    assert!(invalid_input.parse::<Block>().is_err());
}

#[test]
fn example() {
    let input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
    assert_eq!(part1(input), 5);
    assert_eq!(part2(input), 7);
}

#[test]
fn task() {
    let input = &read_input_to_string(22).unwrap();
    assert_eq!(part1(input), 465);
    assert_eq!(part2(input), 79042);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = &read_input_to_string(22).unwrap();
        part1(input);
        part2(input);
    })
}
