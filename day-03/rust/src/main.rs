use std::env;
use std::fs;
use std::path::Path;

fn num_trees_encountered(tree_map: &[&str], slope: (usize, usize)) -> u32 {
    let num_cols = tree_map[0].chars().count();
    let num_rows = tree_map.len();

    let mut col_idx: usize = 0;
    let mut row_idx: usize = 0;

    let mut trees_encountered: u32 = 0;

    while row_idx < num_rows {
        if tree_map[row_idx].chars().nth(col_idx).unwrap() == "#".chars().next().unwrap() {
            trees_encountered += 1;
        }

        col_idx += &slope.0;
        row_idx += &slope.1;

        if col_idx >= num_cols {
            col_idx -= num_cols
        }
    }

    trees_encountered
}

fn main() {
    let input = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("input.txt"),
    )
    .unwrap();
    let tree_map: Vec<_> = input.lines().collect();

    let part_one_solution = num_trees_encountered(&tree_map, (3, 1));
    let part_two_solution = num_trees_encountered(&tree_map, (1, 1))
        * num_trees_encountered(&tree_map, (3, 1))
        * num_trees_encountered(&tree_map, (5, 1))
        * num_trees_encountered(&tree_map, (7, 1))
        * num_trees_encountered(&tree_map, (1, 2));

    println!("Part One: {}", part_one_solution);
    println!("Part Two: {}", part_two_solution);

    assert_eq!(part_one_solution, 265);
    assert_eq!(part_two_solution, 3154761400);
}
