import math
import pathlib


def num_trees_encountered(tree_map: list[str], slope: tuple[int, int]) -> int:
    num_cols = len(tree_map[0])
    num_rows = len(tree_map)

    col_idx = 0
    row_idx = 0

    trees_encountered = 0

    while row_idx < num_rows:
        if tree_map[row_idx][col_idx] == '#':
            trees_encountered += 1

        col_idx += slope[0]
        row_idx += slope[1]

        if col_idx >= num_cols:
            col_idx -= num_cols

    return trees_encountered


with open(pathlib.Path(__file__).parent.parent / 'input.txt') as f:
    tree_map = f.read().splitlines()

part_one_solution = num_trees_encountered(tree_map, (3, 1))
part_two_solution = math.prod(
    num_trees_encountered(tree_map, slope)
    for slope in ((1, 1), (3, 1), (5, 1), (7, 1), (1, 2))
)

print('Part One:', part_one_solution)
print('Part Two:', part_two_solution)

assert part_one_solution == 265
assert part_two_solution == 3154761400
