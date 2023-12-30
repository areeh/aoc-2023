extern crate test;

use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use petgraph::dot::{Config, Dot};
use petgraph::graph::{NodeIndex, UnGraph};
use petgraph::prelude::Dfs;
use petgraph::Graph;
#[cfg(test)]
use test::Bencher;

use crate::utils::read_input_to_string;

type Component = [char; 3];

fn parse_line(line: &str) -> Result<(Component, Vec<Component>), &'static str> {
    let parts: Vec<&str> = line.split(':').map(str::trim).collect();
    if parts.len() != 2 {
        return Err("Invalid line format");
    }

    let key = to_component(parts[0])?;
    let values = parts[1]
        .split_whitespace()
        .map(to_component)
        .collect::<Result<Vec<_>, _>>()?;

    Ok((key, values))
}

fn to_component(s: &str) -> Result<Component, &'static str> {
    let mut chars = s.chars();
    let c1 = chars.next().ok_or("String is too short")?;
    let c2 = chars.next().ok_or("String is too short")?;
    let c3 = chars.next().ok_or("String is too short")?;

    if chars.next().is_some() {
        return Err("String is too long");
    }

    Ok([c1, c2, c3])
}

type ComponentGraph = UnGraph<String, bool>;

fn build_graph(
    component_connections: Vec<(Component, Vec<Component>)>,
) -> (ComponentGraph, HashMap<Component, NodeIndex>) {
    let mut g: ComponentGraph = Graph::new_undirected();
    let mut component_map: HashMap<Component, NodeIndex> = HashMap::new();

    for (source, destinations) in component_connections {
        component_map
            .entry(source)
            .or_insert_with(|| g.add_node(source.iter().collect::<String>()));
        for dest in destinations {
            component_map
                .entry(dest)
                .or_insert_with(|| g.add_node(dest.iter().collect::<String>()));
            g.add_edge(component_map[&source], component_map[&dest], false);
        }
    }

    println!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel]));

    (g, component_map)
}

fn count_nodes_in_subgraphs(graph: ComponentGraph, test_nodes: Vec<NodeIndex>) -> Vec<usize> {
    let mut visited = HashSet::new();
    let mut counts = Vec::new();

    for node in test_nodes {
        if !visited.contains(&node) {
            let mut dfs = Dfs::new(&graph, node);
            let mut count = 0;

            while let Some(nx) = dfs.next(&graph) {
                if visited.insert(nx) {
                    count += 1;
                }
            }

            counts.push(count);
        }
    }

    counts
}

fn part1(input: &str, separation_nodes: [(&str, &str); 3]) -> usize {
    let component_connections = input
        .lines()
        .map(parse_line)
        .collect::<Result<_, _>>()
        .unwrap();
    let (mut g, component_map) = build_graph(component_connections);

    for (a, b) in separation_nodes {
        match (to_component(a), to_component(b)) {
            (Ok(a), Ok(b)) => {
                g.remove_edge(g.find_edge(component_map[&a], component_map[&b]).unwrap())
            }
            _ => panic!("wut"),
        };
    }

    let separated_nodes = [separation_nodes[0].0, separation_nodes[0].1];
    let nodes = separated_nodes
        .iter()
        .map(|s| component_map[&to_component(s).unwrap()])
        .collect();
    count_nodes_in_subgraphs(g, nodes).into_iter().product()
}

// Found using sfdp layout in Graphviz:  sfdp -Tsvg aoc-2023/src/day25/graph.dot -o aoc-2023/src/day25/graph.svg
const SEPARATION_NODES: [(&str, &str); 3] = [("nvf", "bvz"), ("cbl", "vmq"), ("klk", "xgz")];

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(25)?;
    dbg!(part1(input, SEPARATION_NODES));

    Ok(())
}

#[test]
fn example() {
    let input = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";
    assert_eq!(
        part1(input, [("cmg", "bvb"), ("nvd", "jqt"), ("hfx", "pzl")]),
        54
    );
}

#[test]
fn task() {
    let input = &read_input_to_string(25).unwrap();
    assert_eq!(part1(input, SEPARATION_NODES), 583632);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = &read_input_to_string(25).unwrap();
        part1(input, SEPARATION_NODES);
    })
}
