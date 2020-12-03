use std::path::Path;

use std::env;
use std::fs;

use itertools::Itertools;

fn part_one_validator(policy: &str, password: &str) -> bool {
    let (counts, character) = policy.split_whitespace().collect_tuple().unwrap();
    let (min_count, max_count) = counts
        .splitn(2, "-")
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
        .splitn(2, "-")
        .map(|p| (p.parse::<usize>().unwrap() - 1))
        .collect_tuple()
        .unwrap();
    let characters = password.chars();

    (&characters.clone().nth(index1).unwrap() == &character)
        ^ (&characters.clone().nth(index2).unwrap() == &character)
}

fn sum_valid_passwords(
    password_validator: &dyn Fn(&str, &str) -> bool,
    passwords_and_policies: &Vec<(&str, &str)>,
) -> usize {
    let valid_passwords: Vec<(&str, &str)> = passwords_and_policies
        .into_iter()
        .filter(|p| password_validator(p.0, p.1))
        .cloned()
        .collect();
    valid_passwords.len()
}

fn main() {
    let input_filepath = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("input.txt");
    let input = fs::read_to_string(input_filepath).unwrap();

    let mut passwords_and_policies: Vec<(&str, &str)> = Vec::new();
    for line in input.lines() {
        passwords_and_policies.push(line.split(": ").collect_tuple().unwrap());
    }

    println!(
        "Part One: {}",
        sum_valid_passwords(&part_one_validator, &passwords_and_policies)
    );
    println!(
        "Part Two: {}",
        sum_valid_passwords(&part_two_validator, &passwords_and_policies)
    );
}
