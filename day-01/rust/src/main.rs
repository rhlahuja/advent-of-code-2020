use std::env;
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
    for expenses in numbers.iter().combinations(num_expenses) {
        if expenses.clone().into_iter().sum::<i32>() == 2020 {
            return expenses.into_iter().product::<i32>();
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

    let mut expenses: Vec<i32> = Vec::new();
    for line in input.lines() {
        expenses.push(line.parse::<i32>().unwrap());
    }

    let part_one_naive_solution = part_one_naive(expenses.as_slice());
    let part_two_naive_solution = part_two_naive(expenses.as_slice());

    assert_eq!(
        part_one_naive_solution,
        itertools_combination_product(&expenses, 2)
    );
    assert_eq!(
        part_two_naive_solution,
        itertools_combination_product(&expenses, 3)
    );

    println!("Part One: {}", part_one_naive_solution);
    println!("Part Two: {}", part_two_naive_solution);
}
