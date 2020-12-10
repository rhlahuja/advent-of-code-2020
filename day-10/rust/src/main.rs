use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn find_jolt_differences_product(adapter_jolts: &[i64]) -> i64 {
    let mut jolt_differences = HashMap::new();
    for (index, jolts) in adapter_jolts[..adapter_jolts.len() - 1].iter().enumerate() {
        *jolt_differences
            .entry(adapter_jolts[index + 1] - jolts)
            .or_insert(0) += 1;
    }
    jolt_differences[&1] * jolt_differences[&3]
}

fn find_valid_permutations(adapter_jolts: &[i64]) -> i64 {
    let mut num_valid_permutations: HashMap<_, _> = [(0, 1)].iter().cloned().collect();
    for jolts in adapter_jolts[1..].iter() {
        num_valid_permutations.insert(
            *jolts,
            (1..4)
                .map(|delta| num_valid_permutations.get(&(jolts - delta)).unwrap_or(&0))
                .sum(),
        );
    }
    num_valid_permutations[&adapter_jolts.last().unwrap()]
}

fn main() {
    let mut adapter_jolts: Vec<_> = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("input.txt"),
    )
    .unwrap()
    .lines()
    .map(|l| l.parse().unwrap())
    .collect();

    adapter_jolts.sort_unstable();
    adapter_jolts.insert(0, 0);
    adapter_jolts.push(adapter_jolts.last().unwrap() + 3);

    let part_one_solution = find_jolt_differences_product(&adapter_jolts);
    let part_two_solution = find_valid_permutations(&adapter_jolts);

    println!("Part One: {}", &part_one_solution);
    println!("Part Two: {}", &part_two_solution);

    assert_eq!(part_one_solution, 2112);
    assert_eq!(part_two_solution, 3022415986688);
}
