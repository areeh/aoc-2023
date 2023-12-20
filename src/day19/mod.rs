extern crate test;

use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::fmt;
use std::ops::RangeInclusive;

use petgraph::algo::is_cyclic_directed;
use petgraph::graph::{DiGraph, NodeIndex};
#[cfg(test)]
use test::Bencher;

use crate::utils::read_input_to_string;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
enum ParseConditionError {
    TooShort,
    InvalidFormat,
    InvalidNumber,
}

#[derive(Debug)]
enum ParsePartError {
    MissingField(String),
    ParseIntError(String, ParseIntError),
    InvalidFormat(String),
}

impl fmt::Display for Destination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Destination::Workflow(s) => write!(f, "{}", s),
            Destination::Terminal(true) => write!(f, "A"),
            Destination::Terminal(false) => write!(f, "R"),
        }
    }
}

impl fmt::Display for ParseDestinationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid format")
    }
}

impl From<(String, ParseIntError)> for ParsePartError {
    fn from(err: (String, ParseIntError)) -> Self {
        ParsePartError::ParseIntError(err.0, err.1)
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Op::Greater => write!(f, ">"),
            Op::Less => write!(f, "<"),
        }
    }
}

#[derive(Debug)]
enum ParseDestinationError {
    InvalidFormat,
}

impl Error for ParseDestinationError {}

#[derive(Debug, Clone)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

impl FromStr for Part {
    type Err = ParsePartError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut x = None;
        let mut m = None;
        let mut a = None;
        let mut s = None;

        for pair in input.trim_matches(|c| c == '{' || c == '}').split(',') {
            let mut split = pair.split('=');
            match (split.next(), split.next()) {
                (Some("x"), Some(value)) => {
                    x = Some(value.parse().map_err(|e| ("x".to_string(), e))?)
                }
                (Some("m"), Some(value)) => {
                    m = Some(value.parse().map_err(|e| ("m".to_string(), e))?)
                }
                (Some("a"), Some(value)) => {
                    a = Some(value.parse().map_err(|e| ("a".to_string(), e))?)
                }
                (Some("s"), Some(value)) => {
                    s = Some(value.parse().map_err(|e| ("s".to_string(), e))?)
                }
                (Some(field), _) => return Err(ParsePartError::InvalidFormat(field.to_string())),
                _ => return Err(ParsePartError::InvalidFormat("Unknown format".to_string())),
            }
        }

        Ok(Part {
            x: x.ok_or_else(|| ParsePartError::MissingField("x".to_string()))?,
            m: m.ok_or_else(|| ParsePartError::MissingField("m".to_string()))?,
            a: a.ok_or_else(|| ParsePartError::MissingField("a".to_string()))?,
            s: s.ok_or_else(|| ParsePartError::MissingField("s".to_string()))?,
        })
    }
}

#[derive(Debug, Copy, Clone)]
enum Op {
    Greater,
    Less,
}

impl FromStr for Op {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            ">" => Ok(Op::Greater),
            "<" => Ok(Op::Less),
            _ => Err("Invalid operator".into()),
        }
    }
}

impl Op {
    fn from_char(c: char) -> Option<Op> {
        match c {
            '>' => Some(Op::Greater),
            '<' => Some(Op::Less),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
struct Condition {
    variable: char,
    condition: Op,
    value: usize,
}

impl FromStr for Condition {
    type Err = ParseConditionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes = s.as_bytes();

        if bytes.len() < 3 {
            return Err(ParseConditionError::TooShort);
        }

        let variable = bytes[0] as char;
        let condition = bytes[1] as char;

        if !variable.is_alphabetic() || !(condition == '>' || condition == '<') {
            return Err(ParseConditionError::InvalidFormat);
        }

        let condition = Op::from_char(condition).unwrap();
        let value_str = &s[2..];
        let value = value_str
            .parse()
            .map_err(|_| ParseConditionError::InvalidNumber)?;

        Ok(Condition {
            variable,
            condition,
            value,
        })
    }
}

impl Condition {
    fn check(&self, part: &Part) -> bool {
        let part_value = match self.variable {
            'x' => part.x,
            'm' => part.m,
            'a' => part.a,
            's' => part.s,
            _ => panic!("bad variable {}", self.variable),
        };

        match self.condition {
            Op::Greater => part_value > self.value,
            Op::Less => part_value < self.value,
        }
    }

    fn negate(&self) -> Self {
        match self.condition {
            Op::Greater => Self {
                variable: self.variable,
                condition: Op::Less,
                value: self.value + 1,
            },
            Op::Less => Self {
                variable: self.variable,
                condition: Op::Greater,
                value: self.value - 1,
            },
        }
    }
}

#[derive(Hash, Clone, Eq, PartialEq)]
enum Destination {
    Workflow(String),
    Terminal(bool),
}

impl Destination {
    fn start() -> Self {
        Destination::Workflow("in".to_string())
    }
}

impl FromStr for Destination {
    type Err = ParseDestinationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Destination::Terminal(true)),
            "R" => Ok(Destination::Terminal(false)),
            _ if s.chars().all(|c| c.is_ascii_lowercase()) => {
                Ok(Destination::Workflow(s.to_string()))
            }
            _ => Err(ParseDestinationError::InvalidFormat),
        }
    }
}

fn parse_condition(input: &str) -> (Condition, String) {
    if let Some((condition, destination)) = input.split(':').collect_tuple() {
        (condition.parse().unwrap(), destination.to_owned())
    } else {
        panic!("bad workflow {input}")
    }
}

type WorkflowItem = (Option<Condition>, Destination);
type WorkflowMap = HashMap<Destination, Vec<WorkflowItem>>;

fn parse_workflow(input: &str) -> (Destination, Vec<WorkflowItem>) {
    if let Some((name, workflow_str)) = input.split('{').collect_tuple() {
        let workflow_str = &workflow_str[..workflow_str.len() - 1];
        let workflow_str = workflow_str.split(',').collect_vec();
        let mut workflow = workflow_str[..workflow_str.len() - 1]
            .iter()
            .map(|s| {
                let (condition, destination) = parse_condition(s);
                (Some(condition), destination.parse().unwrap())
            })
            .collect_vec();
        workflow.push((None, workflow_str.last().unwrap().parse().unwrap()));

        (name.parse().unwrap(), workflow)
    } else {
        panic!("bad workflow {input}")
    }
}

fn check_part(part: &Part, workflows: &WorkflowMap) -> bool {
    let mut destination = Destination::start();

    while let Destination::Workflow(_) = destination {
        let workflow = &workflows[&destination];

        for (cond, dest) in workflow.clone().into_iter() {
            if let Some(condition) = cond {
                if condition.check(part) {
                    destination = dest.clone();
                    break;
                }
            } else {
                destination = dest.clone();
                break;
            }
        }
    }

    match destination {
        Destination::Terminal(accept) => accept,
        _ => panic!("Bad final destination {destination}"),
    }
}

fn part1(input: &str) -> usize {
    let mut lines_iter = input.lines();

    let workflows: WorkflowMap = lines_iter
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(parse_workflow)
        .collect();
    let parts: Vec<Part> = lines_iter
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap();

    parts
        .iter()
        .filter_map(|part| {
            if check_part(part, &workflows) {
                Some(part.sum())
            } else {
                None
            }
        })
        .sum()
}

#[derive(Clone)]
enum Node {
    Cond(Condition),
    Exit(bool),
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Node::Cond(condition) => {
                write!(
                    f,
                    "{} {} {}",
                    condition.variable, condition.condition, condition.value
                )
            }
            Node::Exit(value) => write!(f, "Exit({})", value),
        }
    }
}

#[derive(Debug, Clone)]
struct ValidRatings {
    range: Option<RangeInclusive<usize>>,
}

impl ValidRatings {
    fn initial() -> Self {
        Self {
            range: Some(1..=4000),
        }
    }

    fn apply_condition(&mut self, cond: Condition) {
        if let Some(range) = &mut self.range {
            match cond.condition {
                Op::Greater => {
                    if *range.end() <= cond.value {
                        self.range = None;
                    } else {
                        *range = cond.value + 1..=*range.end();
                    }
                }
                Op::Less => {
                    if *range.start() >= cond.value {
                        self.range = None;
                    } else {
                        *range = *range.start()..=cond.value - 1;
                    }
                }
            }
        }
    }

    fn count(&self) -> usize {
        if let Some(range) = self.range.as_ref() {
            let (start, end) = (range.start(), range.end());
            if start <= end {
                end - start + 1
            } else {
                0
            }
        } else {
            0
        }
    }
}

fn find_paths(
    graph: &DiGraph<Node, bool>,
    start: NodeIndex,
    end: NodeIndex,
) -> Vec<Vec<NodeIndex>> {
    let mut paths = dfs(graph, start, end);
    for inner_vec in paths.iter_mut() {
        inner_vec.reverse();
    }
    paths.dedup();
    paths
}

fn dfs(graph: &DiGraph<Node, bool>, current: NodeIndex, end: NodeIndex) -> Vec<Vec<NodeIndex>> {
    let mut paths = Vec::new();

    if current == end {
        paths.push(vec![end]);
    } else {
        for neighbor in graph.neighbors(current) {
            for mut path in dfs(graph, neighbor, end) {
                path.push(current);
                paths.push(path);
            }
        }
    }

    paths
}

type OutgoingNode = Option<(NodeIndex<u32>, bool)>;

fn part2(input: &str) -> usize {
    let workflows: WorkflowMap = input
        .lines()
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(parse_workflow)
        .collect();

    let mut g = DiGraph::new();

    let mut queue: VecDeque<(OutgoingNode, Destination)> = VecDeque::new();
    queue.push_back((None, Destination::Workflow("in".to_string())));

    let reject_idx = g.add_node(Node::Exit(false));
    let accept_idx = g.add_node(Node::Exit(true));

    while let Some((maybe_prev, node)) = queue.pop_front() {
        if let Destination::Terminal(accept) = node {
            if accept {
                if let Some((prev, edge_value)) = maybe_prev {
                    g.add_edge(prev, accept_idx, edge_value);
                } else {
                    panic!("Hit accept without a previous node")
                }
            } else if let Some((prev, edge_value)) = maybe_prev {
                g.add_edge(prev, reject_idx, edge_value);
            } else {
                panic!("Hit reject without a previous node")
            }
            continue;
        }

        let workflow = &workflows
            .get(&node)
            .unwrap_or_else(|| panic!("Did not find node {node} in workflow map"));

        let mut maybe_prev = maybe_prev;
        workflow.iter().for_each(|(maybe_cond, dest)| {
            if let Some(cond) = maybe_cond {
                maybe_prev = if let Some((prev, edge_value)) = maybe_prev {
                    let current = g.add_node(Node::Cond(cond.clone()));
                    g.add_edge(prev, current, edge_value);
                    queue.push_back((Some((current, true)), dest.clone()));
                    Some((current, false))
                } else {
                    let current = g.add_node(Node::Cond(cond.clone()));
                    queue.push_back((Some((current, true)), dest.clone()));
                    Some((current, false))
                };
            } else if let Destination::Workflow(_) = dest {
                queue.push_back((maybe_prev, dest.clone()));
            } else {
                let current = match dest {
                    Destination::Terminal(true) => accept_idx,
                    Destination::Terminal(false) => reject_idx,
                    _ => unreachable!(),
                };
                if let Some((prev, edge_value)) = maybe_prev {
                    g.add_edge(prev, current, edge_value);
                } else {
                    panic!("had no previous node when hitting terminal node")
                }
            }
        });
    }

    assert!(!is_cyclic_directed(&g));

    let paths = find_paths(&g, 2u32.into(), accept_idx);

    let mut valid_ratings = 0;
    for path in paths {
        let mut rating_ranges: [ValidRatings; 4] = std::array::from_fn(|_| ValidRatings::initial());
        for (a_idx, b_idx) in path.iter().tuple_windows() {
            let edges = g.edges_connecting(*a_idx, *b_idx).collect_vec();

            let edge_weight = match edges.len() {
                2 => continue,
                1 => edges[0].weight(),
                0 => panic!(
                    "No edges for nodes {:?} {:?}",
                    g.node_weight(*a_idx),
                    g.node_weight(*b_idx)
                ),
                _ => panic!("Too many edges"),
            };

            let cond = g.node_weight(*a_idx).unwrap();
            if let Node::Cond(cond) = cond {
                let rating_range = match cond.variable {
                    'x' => &mut rating_ranges[0],
                    'm' => &mut rating_ranges[1],
                    'a' => &mut rating_ranges[2],
                    's' => &mut rating_ranges[3],
                    _ => panic!("bad variable {}", cond.variable),
                };
                if *edge_weight {
                    rating_range.apply_condition(cond.clone());
                } else {
                    rating_range.apply_condition(cond.negate());
                }
            }
        }
        valid_ratings += rating_ranges.map(|v| v.count()).iter().product::<usize>();
    }

    valid_ratings
}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(19)?;
    dbg!(part1(input));
    dbg!(part2(input));

    Ok(())
}

#[test]
fn example() {
    let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
    assert_eq!(part1(input), 19114);
    assert_eq!(part2(input), 167409079868000);
}

#[test]
fn task() {
    let input = &read_input_to_string(19).unwrap();
    assert_eq!(part1(input), 325952);
    assert_eq!(part2(input), 125744206494820);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = &read_input_to_string(19).unwrap();
        part1(input);
        part2(input);
    })
}
