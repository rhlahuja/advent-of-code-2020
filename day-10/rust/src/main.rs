use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn find_jolt_differences_product(adapter_jolts: &[u32]) -> u32 {
    let mut jolt_differences = HashMap::new();
    for (index, jolts) in adapter_jolts[..adapter_jolts.len() - 1].iter().enumerate() {
        *jolt_differences
            .entry(adapter_jolts[index + 1] - jolts)
            .or_insert(0) += 1;
    }
    jolt_differences[&1] * jolt_differences[&3]
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

    println!("Part One: {}", &part_one_solution);

    assert_eq!(part_one_solution, 2112);
}
