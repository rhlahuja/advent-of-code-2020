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

print('Part One: ', num_trees_encountered(tree_map, (3, 1)))
print(
    'Part Two: ',
    num_trees_encountered(tree_map, (1, 1))
    * num_trees_encountered(tree_map, (3, 1))
    * num_trees_encountered(tree_map, (5, 1))
    * num_trees_encountered(tree_map, (7, 1))
    * num_trees_encountered(tree_map, (1, 2)),
)
