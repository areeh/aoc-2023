extern crate test;

use std::collections::hash_map::Entry;
use std::str::FromStr;
#[cfg(test)]
use test::Bencher;

use crate::utils::read_input_to_string;

fn capital_hash(input: &str) -> usize {
    input.chars().fold(0, |mut acc, c| {
        acc += c as usize;
        acc *= 17;
        acc %= 256;
        acc
    })
}

fn part1(input: &str) -> usize {
    input.trim().split(',').map(capital_hash).sum()
}

type Boxes = [MapWithInsertionOrder];

fn focusing_power(boxes: &Boxes) -> impl Iterator<Item = usize> + '_ {
    boxes
        .iter()
        .enumerate()
        .flat_map(move |(box_number, lens_box)| {
            lens_box
                .iterate()
                .enumerate()
                .map(move |(slot_number, focal_length)| {
                    (box_number + 1) * (slot_number + 1) * focal_length as usize
                })
        })
}

#[derive(Debug, PartialEq)]
enum Op {
    Sub(String),
    Eq(String, u8),
}

impl FromStr for Op {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if let Some(eq_index) = value.find('=') {
            let (label, focal) = value.split_at(eq_index);
            if let Ok(focal) = focal[1..].parse::<u8>() {
                Ok(Op::Eq(label.into(), focal))
            } else {
                Err(format!("bad eq focal {focal}"))
            }
        } else if let Some(label) = value.strip_suffix('-') {
            Ok(Op::Sub(label.to_string()))
        } else {
            Err(format!("bad op {value}"))
        }
    }
}

use std::collections::HashMap;

struct MapWithInsertionOrder {
    map: HashMap<String, (u8, usize)>,
    insertion_count: usize,
}

impl MapWithInsertionOrder {
    fn new() -> Self {
        MapWithInsertionOrder {
            map: HashMap::new(),
            insertion_count: 0,
        }
    }

    #[allow(dead_code)]
    fn from_vec(vec: Vec<(String, u8)>) -> Self {
        let mut structure = MapWithInsertionOrder::new();
        for (label, focal) in vec {
            structure.insert_or_swap(label, focal);
        }
        structure
    }

    fn insert_or_swap(&mut self, label: String, focal: u8) {
        match self.map.entry(label) {
            Entry::Vacant(e) => {
                self.insertion_count += 1;
                e.insert((focal, self.insertion_count));
            }
            Entry::Occupied(mut e) => {
                e.insert((focal, e.get().1));
            }
        }
    }

    fn delete(&mut self, label: &str) {
        self.map.remove(label);
    }

    fn iterate(&self) -> impl Iterator<Item = u8> {
        let mut values: Vec<(u8, usize)> = self.map.clone().into_values().collect();
        values.sort_by_key(|&(_, count)| count);
        values.into_iter().map(|(focal, _)| focal)
    }
}

fn remove_from_boxes(label: String, boxes: &mut Boxes) {
    let lens_box = &mut boxes[capital_hash(&label)];
    lens_box.delete(&label)
}

fn add_to_boxes(label: String, focal: u8, boxes: &mut Boxes) {
    let lens_box = &mut boxes[capital_hash(&label)];
    lens_box.insert_or_swap(label, focal)
}

fn part2(input: &str) -> usize {
    let lens_ops: Vec<Op> = input
        .trim()
        .split(',')
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap();
    let mut boxes: [MapWithInsertionOrder; 256] =
        std::array::from_fn(|_| MapWithInsertionOrder::new());

    for op in lens_ops {
        match op {
            Op::Eq(label, focal) => add_to_boxes(label, focal, &mut boxes),
            Op::Sub(label) => remove_from_boxes(label, &mut boxes),
        }
    }

    focusing_power(&boxes).sum()
}

pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(15)?;
    dbg!(part1(input));
    dbg!(part2(input));

    Ok(())
}

#[test]
fn example_hash() {
    let input = "HASH";
    assert_eq!(capital_hash(input), 52);
}

#[test]
fn test_focusing_power() {
    let input = [
        MapWithInsertionOrder::from_vec(vec![("rn".into(), 1), ("cm".into(), 2)]),
        MapWithInsertionOrder::from_vec(vec![]),
        MapWithInsertionOrder::from_vec(vec![]),
        MapWithInsertionOrder::from_vec(vec![("ot".into(), 7), ("ab".into(), 5), ("pc".into(), 6)]),
    ];

    assert_eq!(
        focusing_power(&input).collect::<Vec<usize>>(),
        [1, 4, 28, 40, 72]
    )
}

#[test]
fn example() {
    let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    assert_eq!(part1(input), 1320);
    assert_eq!(part2(input), 145);
}

#[test]
fn test_lens_op_parse() {
    assert_eq!("fszj=6".parse::<Op>(), Ok(Op::Eq("fszj".into(), 6)));
    assert_eq!("fszj-".parse::<Op>(), Ok(Op::Sub("fszj".into())));
}

#[test]
fn task() {
    let input = &read_input_to_string(15).unwrap();
    assert_eq!(part1(input), 504036);
    assert_eq!(part2(input), 295719);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = &read_input_to_string(15).unwrap();
        part1(input);
        part2(input);
    })
}
