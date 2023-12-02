use std::hash::Hash;
use std::{collections::HashSet, fs};

use ndarray::{Array2, ArrayBase, Axis, Ix2, RawData};

pub(crate) fn read_input_to_string(day: u32) -> std::io::Result<String> {
    fs::read_to_string(format!("./src/day{day}/input.txt"))
}

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

/// See: https://github.com/rust-ndarray/ndarray/issues/866
pub(crate) fn rot90<S>(arr: &mut ArrayBase<S, Ix2>)
where
    S: RawData,
{
    arr.swap_axes(0, 1);
    arr.invert_axis(Axis(0));
}

pub(crate) fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}
