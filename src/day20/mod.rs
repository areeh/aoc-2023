extern crate test;

use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::iter::once;
use std::str::FromStr;

use num_integer::gcd;
use petgraph::graph::{DiGraph, NodeIndex};
#[cfg(test)]
use test::Bencher;

use crate::utils::read_input_to_string;

#[derive(Debug)]
enum ParseModuleError {
    InvalidFormat(String),
}

impl fmt::Display for ParseModuleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseModuleError::InvalidFormat(input) => write!(f, "Invalid module format: {}", input),
        }
    }
}

impl FromStr for Module {
    type Err = ParseModuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" -> ").collect();
        if parts.len() != 2 {
            return Err(ParseModuleError::InvalidFormat(s.to_string()));
        }

        let label = parts[0].to_string();
        let destinations: Vec<String> = parts[1].split(", ").map(String::from).collect();

        match label.chars().next() {
            Some('&') => Ok(Module::C(Conjunction::new(
                label[1..].to_string(),
                destinations,
            ))),
            Some('%') => Ok(Module::F(FlipFlop::new(
                label[1..].to_string(),
                destinations,
            ))),
            _ => Ok(Module::B(Broadcaster::new(label, destinations))),
        }
    }
}

#[derive(Debug)]
struct FlipFlop {
    label: String,
    state: bool,
    destinations: Vec<String>,
}

impl FlipFlop {
    fn new(label: String, destinations: Vec<String>) -> Self {
        Self {
            label,
            state: false,
            destinations,
        }
    }

    fn process(
        &mut self,
        pulse: bool,
        iter_map: &mut HashMap<String, Vec<usize>>,
        i: usize,
    ) -> Option<bool> {
        if !pulse {
            if let Some(v) = iter_map.get_mut(&self.label) {
                v.push(i)
            }
            self.state = !self.state;
            Some(self.state)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Conjunction {
    label: String,
    state: HashMap<String, bool>,
    destinations: Vec<String>,
}

impl Conjunction {
    fn new(label: String, destinations: Vec<String>) -> Self {
        Self {
            label,
            state: HashMap::new(),
            destinations,
        }
    }

    fn process(
        &mut self,
        source: &str,
        pulse: bool,
        iter_map: &mut HashMap<String, Vec<usize>>,
        i: usize,
    ) -> bool {
        self.state.insert(source.to_string(), pulse);
        let pulse = !self.state.values().all(|v| *v);
        if !pulse {
            if let Some(v) = iter_map.get_mut(&self.label) {
                v.push(i);
            }
        }
        pulse
    }
}

#[derive(Debug)]
struct Broadcaster {
    label: String,
    destinations: Vec<String>,
}

impl Broadcaster {
    fn new(label: String, destinations: Vec<String>) -> Self {
        Self {
            label,
            destinations,
        }
    }

    fn process(&mut self) -> bool {
        false
    }
}

#[derive(Debug)]
struct Output {
    destinations: Vec<String>,
    label: String,
}

impl Output {
    fn new(label: &str) -> Self {
        Self {
            destinations: vec![],
            label: label.to_string(),
        }
    }

    fn process(&self, pulse: bool, i: usize) {
        if !pulse {
            panic!("Winner at {i}")
        }
    }
}

#[derive(Debug)]
enum Module {
    F(FlipFlop),
    C(Conjunction),
    B(Broadcaster),
    O(Output),
}

impl Module {
    fn destinations(&self) -> &[String] {
        match self {
            Module::B(b) => &b.destinations,
            Module::F(f) => &f.destinations,
            Module::C(c) => &c.destinations,
            Module::O(o) => &o.destinations,
        }
    }
    fn label(&self) -> &str {
        match self {
            Module::B(b) => &b.label,
            Module::F(f) => &f.label,
            Module::C(c) => &c.label,
            Module::O(o) => &o.label,
        }
    }

    fn process(
        &mut self,
        source: &str,
        pulse: bool,
        iter_map: &mut HashMap<String, Vec<usize>>,
        i: usize,
    ) -> Option<bool> {
        match self {
            Module::F(f) => f.process(pulse, iter_map, i),
            Module::C(c) => Some(c.process(source, pulse, iter_map, i)),
            Module::B(b) => Some(b.process()),
            Module::O(o) => {
                o.process(pulse, i);
                None
            }
        }
    }
}

type ModuleGraph = DiGraph<String, bool, u32>;

fn label_to_key(label: &str) -> String {
    if matches!(label.chars().next(), Some('%') | Some('&')) {
        &label[1..]
    } else {
        label
    }
    .to_string()
}

fn label_to_nodeindex(
    label: &str,
    node_map: &mut HashMap<String, NodeIndex>,
    g: &mut ModuleGraph,
) -> NodeIndex {
    *node_map
        .entry(label_to_key(label))
        .or_insert_with(|| g.add_node(label.to_string()))
}

fn make_edges(input: &str, node_map: &mut HashMap<String, NodeIndex>, g: &mut ModuleGraph) {
    if let Some((source, destinations)) = input.split_once(" -> ") {
        let source = node_map[&label_to_key(source)];
        for dest in destinations.split(", ") {
            let dest = node_map
                .get(dest)
                .unwrap_or_else(|| panic!("no entry found for key {dest}"));
            g.add_edge(source, *dest, false);
        }
    } else {
        panic!("bad module {input}")
    }
}

fn make_graph(input: &str) -> ModuleGraph {
    let mut g = DiGraph::new();
    let mut node_map = HashMap::new();

    for line in input.lines() {
        if let Some((source, _)) = line.split_once(" -> ") {
            label_to_nodeindex(source, &mut node_map, &mut g);
        } else {
            panic!("bad module {line}")
        }
    }
    node_map.insert(RX.to_string(), g.add_node(RX.to_string()));

    input
        .lines()
        .for_each(|line| make_edges(line, &mut node_map, &mut g));
    g
}

const RX: &str = "rx";

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn parts(input: &str, part2: bool) -> usize {
    let presses = if part2 { 10000 } else { 1000 };

    let modules: Vec<Module> = input
        .lines()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap();

    let mut modules: HashMap<String, Module> = modules
        .into_iter()
        .map(|m| (m.label().to_string(), m))
        .collect();
    // dbg!(&modules);

    let source_dest_pairs: Vec<(String, String)> = modules
        .values()
        .flat_map(|m| {
            m.destinations()
                .iter()
                .map(|dest| (m.label().to_string(), dest.to_string()))
        })
        .collect();
    for (source, dest) in source_dest_pairs {
        if let Some(Module::C(c)) = modules.get_mut(&dest) {
            c.state.insert(source, false);
        }
    }

    modules.insert(RX.to_string(), Module::O(Output::new(RX)));

    let mut queue: VecDeque<(String, String, bool)> = VecDeque::new();
    let mut pulse_neg = 0;
    let mut pulse_pos = 0;

    let mut iter_map: HashMap<String, Vec<usize>> = HashMap::new();
    iter_map.insert("nl".to_string(), vec![]);
    iter_map.insert("dj".to_string(), vec![]);
    iter_map.insert("rr".to_string(), vec![]);
    iter_map.insert("pb".to_string(), vec![]);

    for i in 0..presses {
        queue.clear();
        queue.push_back(("".to_string(), "broadcaster".to_string(), false));

        while let Some((source, dest, pulse)) = queue.pop_front() {
            if pulse {
                pulse_pos += 1;
            } else {
                pulse_neg += 1;
            }

            if let Some(m) = &mut modules.get_mut(&dest) {
                if let Some(pulse) = m.process(&source, pulse, &mut iter_map, i + 1) {
                    for new_dest in m.destinations() {
                        queue.push_back((dest.to_string(), new_dest.to_string(), pulse))
                    }
                }
            }
        }
    }

    let deltas = iter_map
        .into_values()
        .map(|values| {
            once(0)
                .chain(values)
                .tuple_windows()
                .map(|(a, b)| b - a)
                .collect_vec()
        })
        .collect_vec();

    if part2 {
        deltas.iter().map(|v| v[0]).fold(1, lcm)
    } else {
        pulse_pos * pulse_neg
    }
}

fn part1(input: &str) -> usize {
    parts(input, false)
}

fn part2(input: &str) -> usize {
    parts(input, true)
}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(20)?;
    dbg!(part1(input));
    dbg!(part2(input));

    Ok(())
}

#[test]
fn test_module_parse() {
    let flipflop_str = "%a -> b, c";
    match flipflop_str.parse::<Module>() {
        Ok(Module::F(flipflop)) => assert_eq!(flipflop.destinations, vec!["b", "c"]),
        _ => panic!("Failed to parse FlipFlop module"),
    }

    let conjunction_str = "&x -> y, z";
    match conjunction_str.parse::<Module>() {
        Ok(Module::C(conjunction)) => assert_eq!(conjunction.destinations, vec!["y", "z"]),
        _ => panic!("Failed to parse Conjunction module"),
    }

    let broadcaster_str = "broadcaster -> a, b, c";
    match broadcaster_str.parse::<Module>() {
        Ok(Module::B(broadcaster)) => assert_eq!(broadcaster.destinations, vec!["a", "b", "c"]),
        _ => panic!("Failed to parse Broadcaster module"),
    }

    let invalid_str = "invalid format";
    assert!(matches!(
        invalid_str.parse::<Module>(),
        Err(ParseModuleError::InvalidFormat(_))
    ));
}

#[test]
fn example() {
    let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
    assert_eq!(part1(input), 32000000);
}

#[test]
fn example2() {
    let input = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
    assert_eq!(part1(input), 11687500);
}

#[test]
fn task() {
    let input = &read_input_to_string(20).unwrap();
    assert_eq!(part1(input), 898731036);
    assert_eq!(part2(input), 229414480926893);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = &read_input_to_string(20).unwrap();
        part1(input);
        part2(input);
    })
}
