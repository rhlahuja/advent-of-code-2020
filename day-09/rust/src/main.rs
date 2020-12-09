use std::cmp::Ordering;
use std::fs;
use std::path::Path;

use itertools::Itertools;

fn find_inconsistent_number(numbers: &[i64], preamble_length: usize) -> i64 {
    let mut current_index = preamble_length;
    for number in numbers[preamble_length..].iter() {
        if numbers[current_index - preamble_length..current_index]
            .iter()
            .combinations(2)
            .find(|pair| *number == pair.clone().into_iter().sum::<i64>())
            .is_none()
        {
            return *number;
        }
        current_index += 1;
    }
    0
}

fn sum_min_max_operands(numbers: &[i64], target_sum: i64) -> i64 {
    let mut min_index = 0;
    let mut max_index = 0;

    while min_index < numbers.len() {
        let operands = &numbers[min_index..max_index];
        match operands.iter().sum::<i64>().cmp(&target_sum) {
            Ordering::Less => max_index += 1,
            Ordering::Greater => min_index += 1,
            Ordering::Equal => {
                return operands.iter().min().unwrap() + operands.iter().max().unwrap()
            }
        }
    }
    0
}

fn main() {
    let input = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("input.txt"),
    )
    .unwrap();

    let mut numbers = Vec::new();
    for line in input.lines() {
        numbers.push(line.parse().unwrap());
    }

    let part_one_solution = find_inconsistent_number(&numbers, 25);
    let part_two_solution = sum_min_max_operands(&numbers, part_one_solution);

    println!("Part One: {}", &part_one_solution);
    println!("Part Two: {}", &part_two_solution);

    assert_eq!(part_one_solution, 15353384);
    assert_eq!(part_two_solution, 2466556);
}
