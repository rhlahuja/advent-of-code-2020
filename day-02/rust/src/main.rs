use std::fs;
use std::path::Path;

use itertools::Itertools;

fn part_one_validator(policy: &str, password: &str) -> bool {
    let (counts, character) = policy.split_whitespace().collect_tuple().unwrap();
    let (min_count, max_count) = counts
        .splitn(2, '-')
        .map(|c| c.parse::<usize>().ok())
        .collect_tuple()
        .unwrap();

    min_count <= Some(password.matches(character).count())
        && Some(password.matches(character).count()) <= max_count
}

fn part_two_validator(policy: &str, password: &str) -> bool {
    let (positions, character) = policy.split_whitespace().collect_tuple().unwrap();
    let character = character.chars().next().unwrap();
    let (index1, index2) = positions
        .splitn(2, '-')
        .map(|p| (p.parse::<usize>().unwrap() - 1))
        .collect_tuple()
        .unwrap();

    (password.chars().nth(index1).unwrap() == character)
        ^ (password.chars().nth(index2).unwrap() == character)
}

fn sum_valid_passwords(
    password_validator: &dyn Fn(&str, &str) -> bool,
    policies_and_passwords: &[(&str, &str)],
) -> usize {
    let valid_passwords: Vec<(&str, &str)> = policies_and_passwords
        .iter()
        .filter(|p| password_validator(p.0, p.1))
        .cloned()
        .collect();
    valid_passwords.len()
}

fn main() {
    let input = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("input.txt"),
    )
    .unwrap();
    let policies_and_passwords: Vec<_> = input
        .lines()
        .map(|l| l.split(": ").collect_tuple().unwrap())
        .collect();

    let part_one_solution = sum_valid_passwords(&part_one_validator, &policies_and_passwords);
    let part_two_solution = sum_valid_passwords(&part_two_validator, &policies_and_passwords);

    println!("Part One: {}", part_one_solution);
    println!("Part Two: {}", part_two_solution);

    assert_eq!(part_one_solution, 410);
    assert_eq!(part_two_solution, 694);
}
