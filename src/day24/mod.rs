extern crate nalgebra as na;
extern crate test;

use itertools::Itertools;
use std::fmt;
use std::num::ParseFloatError;
use std::str::FromStr;

#[cfg(test)]
use test::Bencher;

use crate::utils::read_input_to_string;

#[derive(Debug, Clone)]
struct Vector2D {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone, Copy)]
struct Vector3D {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3D {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Clone, Copy)]
struct Ray {
    origin: Vector3D,
    direction: Vector3D,
}

#[derive(Debug)]
enum ParseError {
    InvalidFormat,
    FloatError(ParseFloatError),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::InvalidFormat => write!(f, "Invalid input format"),
            ParseError::FloatError(ref err) => err.fmt(f),
        }
    }
}

impl From<ParseFloatError> for ParseError {
    fn from(err: ParseFloatError) -> Self {
        ParseError::FloatError(err)
    }
}

impl FromStr for Vector3D {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<f64> = s
            .split(',')
            .map(str::trim)
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        if parts.len() == 3 {
            Ok(Vector3D {
                x: parts[0],
                y: parts[1],
                z: parts[2],
            })
        } else {
            Err(ParseError::InvalidFormat)
        }
    }
}

impl FromStr for Ray {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('@').map(str::trim).collect();

        if parts.len() == 2 {
            let origin = parts[0].parse()?;
            let direction = parts[1].parse()?;
            Ok(Ray { origin, direction })
        } else {
            Err(ParseError::InvalidFormat)
        }
    }
}

impl Ray {
    fn xy_intersection(&self, other: &Ray) -> Option<Vector2D> {
        let matrix = na::Matrix2::new(
            self.direction.x,
            -other.direction.x,
            self.direction.y,
            -other.direction.y,
        );
        let rhs = na::Vector2::new(
            other.origin.x - self.origin.x,
            other.origin.y - self.origin.y,
        );

        let lu = na::linalg::LU::new(matrix);
        if let Some(solution) = lu.solve(&rhs) {
            let (t_self, t_other) = (solution[0], solution[1]);
            if t_self >= 0.0 && t_other >= 0.0 {
                return Some(Vector2D {
                    x: self.direction.x * t_self + self.origin.x,
                    y: self.direction.y * t_self + self.origin.y,
                });
            }
        }
        None
    }

    fn xy_intersects(&self, other: &Ray, bounds: &(Vector2D, Vector2D)) -> bool {
        if let Some(intersection) = self.xy_intersection(other) {
            // println!("{self:?} {other:?} intersect at {intersection:?}");

            intersection.x >= bounds.0.x
                && intersection.x <= bounds.1.x
                && intersection.y >= bounds.0.y
                && intersection.y <= bounds.1.y
        } else {
            false
        }
    }
}

const EXAMPLE_BOUND: (f64, f64) = (7.0, 27.0);
const TASK_BOUND: (f64, f64) = (200000000000000.0, 400000000000000.0);

fn part1(input: &str, bound: (f64, f64)) -> usize {
    let hailstones: Vec<Ray> = input
        .lines()
        .map(str::parse)
        .collect::<Result<_, _>>()
        .unwrap();
    let bounds = (
        Vector2D {
            x: bound.0,
            y: bound.0,
        },
        Vector2D {
            x: bound.1,
            y: bound.1,
        },
    );

    hailstones
        .iter()
        .tuple_combinations()
        .filter(|(a, b)| a.xy_intersects(b, &bounds))
        .count()
}


pub fn main() -> std::io::Result<()> {
    let input = &read_input_to_string(24)?;
    dbg!(part1(input, TASK_BOUND),);
    // dbg!(part2(input));

    Ok(())
}

#[test]
fn example() {
    let input = "19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3";
    assert_eq!(part1(input, EXAMPLE_BOUND), 2);
    // assert_eq!(part2(input), 47);
}

#[test]
fn task() {
    let input = &read_input_to_string(24).unwrap();
    assert_eq!(part1(input, TASK_BOUND), 13892);
    // assert_eq!(part2(input), 1);
}

#[bench]
fn task_bench(b: &mut Bencher) {
    b.iter(|| {
        let input = &read_input_to_string(24).unwrap();
        part1(input, TASK_BOUND);
        // part2(input);
    })
}
