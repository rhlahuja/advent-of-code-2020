use std::fs;
use std::path::Path;

use itertools::Itertools;

fn part_one_naive(numbers: &[i32]) -> i32 {
    for num1 in numbers {
        let num2 = 2020 - num1;
        if numbers.contains(&num2) {
            return num1 * num2;
        };
    }
    0
}

fn part_two_naive(numbers: &[i32]) -> i32 {
    for num1 in numbers {
        for num2 in numbers {
            let num3 = 2020 - num1 - num2;
            if numbers.contains(&num3) {
                return num1 * num2 * num3;
            }
        }
    }
    0
}

fn itertools_combination_product(numbers: &[i32], num_expenses: usize) -> i32 {
    numbers
        .iter()
        .combinations(num_expenses)
        .find(|expenses| expenses.clone().into_iter().sum::<i32>() == 2020)
        .map(|expenses| expenses.into_iter().product())
        .unwrap()
}

fn main() {
    let expenses: Vec<_> = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("input.txt"),
    )
    .unwrap()
    .lines()
    .map(|l| l.parse().unwrap())
    .collect();

    let part_one_naive_solution = part_one_naive(expenses.as_slice());
    let part_two_naive_solution = part_two_naive(expenses.as_slice());

    println!("Part One: {}", part_one_naive_solution);
    println!("Part Two: {}", part_two_naive_solution);

    assert_eq!(part_one_naive_solution, 482811);
    assert_eq!(part_two_naive_solution, 193171814);

    assert_eq!(
        part_one_naive_solution,
        itertools_combination_product(&expenses, 2)
    );
    assert_eq!(
        part_two_naive_solution,
        itertools_combination_product(&expenses, 3)
    );
}
