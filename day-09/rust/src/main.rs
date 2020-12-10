use std::cmp::Ordering;
use std::fs;
use std::path::Path;

use itertools::Itertools;

fn find_inconsistent_number_naive(numbers: &[i64], preamble_length: usize) -> i64 {
    let mut current_index = preamble_length;
    for number in numbers[preamble_length..].iter() {
        if numbers[current_index - preamble_length..current_index]
            .iter()
            .tuple_combinations()
            .find(|(a, b)| *number == *a + *b)
            .is_none()
        {
            return *number;
        }
        current_index += 1;
    }
    0
}

fn find_inconsistent_number_windows(numbers: &[i64], preamble_length: usize) -> i64 {
    numbers
        .windows(preamble_length + 1)
        .find(|window| {
            window[0..preamble_length]
                .iter()
                .tuple_combinations()
                .all(|(a, b)| a + b != window[preamble_length])
        })
        .map(|window| window[preamble_length])
        .unwrap() as i64
}

fn sum_min_max_operands(numbers: &[i64], target_sum: i64) -> i64 {
    let (mut min_index, mut max_index) = (0, 0);

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
    let numbers: Vec<_> = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("input.txt"),
    )
    .unwrap()
    .lines()
    .map(|l| l.parse().unwrap())
    .collect();

    let part_one_solution = find_inconsistent_number_naive(&numbers, 25);
    let part_two_solution = sum_min_max_operands(&numbers, part_one_solution);

    println!("Part One: {}", &part_one_solution);
    println!("Part Two: {}", &part_two_solution);

    assert_eq!(part_one_solution, 15353384);
    assert_eq!(
        part_one_solution,
        find_inconsistent_number_windows(&numbers, 25)
    );
    assert_eq!(part_two_solution, 2466556);
}
