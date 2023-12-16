use itertools::Itertools;
use std::hash::Hash;
use std::{collections::HashSet, fs};

use ndarray::{Array2, ArrayBase, Axis, Ix2, RawData};

pub(crate) fn read_input_to_string(day: u32) -> std::io::Result<String> {
    fs::read_to_string(format!("./src/day{day}/input.txt"))
}

#[allow(dead_code)]
pub(crate) fn pretty_print(arr: &Array2<char>) -> String {
    let mut result = String::new();
    for row in arr.rows() {
        for elem in row {
            result.push(*elem);
        }
        result.push('\n');
    }

    result.trim_end().to_owned()
}

#[allow(dead_code)]
/// See: https://github.com/rust-ndarray/ndarray/issues/866
pub(crate) fn rot90<S>(arr: &mut ArrayBase<S, Ix2>)
where
    S: RawData,
{
    arr.swap_axes(0, 1);
    arr.invert_axis(Axis(0));
}

#[allow(dead_code)]
/// See: https://github.com/rust-ndarray/ndarray/issues/866
pub(crate) fn rot270<S>(arr: &mut ArrayBase<S, Ix2>)
where
    S: RawData,
{
    arr.swap_axes(0, 1);
    arr.invert_axis(Axis(1));
}

#[allow(dead_code)]
pub(crate) fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

pub(crate) fn parse_board(input: &str) -> Array2<char> {
    let board_width = input.lines().next().unwrap().len();

    let mut data = Vec::new();
    for line in input.lines() {
        let mut row: Vec<_> = line.trim().chars().collect_vec();
        data.append(&mut row);
    }

    let data_len = data.len();
    let n_rows = data_len / board_width;

    Array2::from_shape_vec((n_rows, board_width), data).unwrap()
}
