use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn find_jolt_differences_product(adapters: &[i64]) -> i64 {
    let jolt_differences = adapters.windows(2).map(|pair| pair[1] - pair[0]).fold(
        HashMap::new(),
        |mut map, jolt_difference| {
            *map.entry(jolt_difference).or_insert(0) += 1;
            map
        },
    );
    jolt_differences[&1] * jolt_differences[&3]
}

fn find_num_valid_permutations(adapters: &[i64]) -> i64 {
    let mut num_valid_arrangements: HashMap<_, _> = [(0, 1)].iter().cloned().collect();
    for adapter_jolts in adapters[1..].iter() {
        num_valid_arrangements.insert(
            *adapter_jolts,
            (1..4)
                .map(|delta| {
                    num_valid_arrangements
                        .get(&(adapter_jolts - delta))
                        .unwrap_or(&0)
                })
                .sum(),
        );
    }
    num_valid_arrangements[&adapters.last().unwrap()]
}

fn main() {
    let mut adapters: Vec<_> = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("input.txt"),
    )
    .unwrap()
    .lines()
    .map(|l| l.parse().unwrap())
    .collect();

    adapters.sort_unstable();
    adapters.insert(0, 0);
    adapters.push(adapters.last().unwrap() + 3);

    let part_one_solution = find_jolt_differences_product(&adapters);
    let part_two_solution = find_num_valid_permutations(&adapters);

    println!("Part One: {}", &part_one_solution);
    println!("Part Two: {}", &part_two_solution);

    assert_eq!(part_one_solution, 2112);
    assert_eq!(part_two_solution, 3022415986688);
}
