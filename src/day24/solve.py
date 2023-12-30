# "ChatGPT, rewrite this in Sympy"

from dataclasses import dataclass
from sympy import symbols, Eq, solve

@dataclass
class ThreeDim:
    x: int
    y: int
    z: int


@dataclass
class Hailstone:
    pos: ThreeDim
    delta: ThreeDim


def parse_hailstone(s: str) -> Hailstone:
    pos_str, delta_str = s.split(" @ ")
    pos_vals = [int(x) for x in pos_str.split(", ")]
    delta_vals = [int(x) for x in delta_str.split(", ")]

    return Hailstone(ThreeDim(*pos_vals), ThreeDim(*delta_vals))


hailstones_all = [parse_hailstone(s) for s in hailstone_strs]
hailstones = hailstones_all[:3]

a, b, c, d, e, f = symbols('a b c d e f', integer=True)
t = symbols('t0:3', integer=True, positive=True)

equations = []
for i, hailstone in enumerate(hailstones):
    equations.append(Eq(a * t[i] + b, hailstone.pos.x + hailstone.delta.x * t[i]))
    equations.append(Eq(c * t[i] + d, hailstone.pos.y + hailstone.delta.y * t[i]))
    equations.append(Eq(e * t[i] + f, hailstone.pos.z + hailstone.delta.z * t[i]))

solution = solve(equations)
print(solution)


