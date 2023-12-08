extern crate test;

use num_integer::gcd;
use std::collections::HashMap;
use std::iter::Cycle;
use std::str::FromStr;

use itertools::Itertools;
#[cfg(test)]
use test::Bencher;

use crate::utils::read_input_to_string;

#[derive(Debug)]
struct Graph {
    edges: HashMap<String, (String, String)>,
}

impl Graph {
    fn new() -> Graph {
        Graph {
            edges: HashMap::new(),
        }
    }

    fn add_edge(&mut self, node: String, left_child: String, right_child: String) {
        self.edges.insert(node, (left_child, right_child));
    }
}

fn extract_codes(input: &str) -> (String, String, String) {
    (
        input[0..3].into(),
        input[7..10].into(),
        input[12..15].into(),
    )
}

impl FromStr for Graph {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut graph = Graph::new();

        s.lines()
            .map(extract_codes)
            .for_each(|(node, left, right)| graph.add_edge(node, left, right));

        Ok(graph)
    }
}

fn skip_first_two_lines(input: &str) -> &str {
    let mut lines_skipped = 0;
    let mut start_index = 0;

    for (index, character) in input.char_indices() {
        if character == '\n' {
            lines_skipped += 1;
            if lines_skipped == 2 {
                start_index = index + 1;
                break;
            }
        }
    }

    &input[start_index..]
}

struct ExitStepsIter<'a> {
    current_node: &'a str,
    graph: &'a Graph,
    directions: Cycle<std::str::Chars<'a>>,
    steps: usize,
}

impl<'a> ExitStepsIter<'a> {
    fn new(node: &'a str, graph: &'a Graph, directions: &'a str) -> Self {
        ExitStepsIter {
            current_node: node,
            graph,
            directions: directions.chars().cycle(),
            steps: 0,
        }
    }
}

impl<'a> Iterator for ExitStepsIter<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let direction = self.directions.next().unwrap();

            match self.graph.edges.get(self.current_node) {
                Some((left, right)) => {
                    self.current_node = match direction {
                        'L' => left,
                        'R' => right,
                        _ => panic!("bad direction"),
                    };
                }
                None => panic!("bad node {}", self.current_node),
            }

            self.steps += 1;

            if self.current_node.ends_with('Z') {
                return Some(self.steps);
            }
        }
    }
}

fn part1(input: &str) -> usize {
    let directions = input.lines().next().unwrap();
    let graph: Graph = skip_first_two_lines(input.trim()).parse().unwrap();

    let mut steps_iter = ExitStepsIter::new("AAA", &graph, directions);
    steps_iter.next().unwrap()
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn part2(input: &str) -> usize {
    let directions = input.lines().next().unwrap();
    let graph: Graph = skip_first_two_lines(input.trim()).parse().unwrap();

    let start_nodes: Vec<&str> = graph
        .edges
        .keys()
        .filter_map(|v| {
            if v.ends_with('A') {
                Some(AsRef::as_ref(v))
            } else {
                None
            }
        })
        .collect();

    let exit_deltas = start_nodes
        .iter()
        .map(|node| {
            let mut step_iter = ExitStepsIter::new(node, &graph, directions);
            let a = step_iter.next().unwrap();
            let b = step_iter.next().unwrap();
            b - a
        })
        .collect_vec();

    exit_deltas.iter().fold(1, |acc, num| lcm(acc, *num))
}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(8)?;
    dbg!(part1(input));
    dbg!(part2(input));

    Ok(())
}

#[test]
fn example() {
    let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
    assert_eq!(part1(input), 2);
}

#[test]
fn example2() {
    let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
    assert_eq!(part1(input), 6);
}

#[test]
fn example3() {
    let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
    assert_eq!(part2(input), 6);
}

#[test]
fn task() {
    let input = &read_input_to_string(8).unwrap();
    assert_eq!(part1(input), 22357);
    assert_eq!(part2(input), 10371555451871);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = &read_input_to_string(8).unwrap();
        part1(input);
        part2(input);
    })
}
