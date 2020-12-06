use std::env;
use std::fs;
use std::path::Path;

fn part_one(group_answers: &[&str]) -> usize {
    group_answers
        .iter()
        .map(|group_answer| {
            let mut group_answer: Vec<char> = group_answer.replace("\n", "").chars().collect();
            group_answer.sort_unstable();
            group_answer.dedup();
            group_answer.len()
        })
        .sum::<usize>()
}

fn main() {
    let input = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("input.txt"),
    )
    .unwrap();
    let group_answers: Vec<&str> = input.split("\n\n").collect();

    println!("Part One: {}", part_one(&group_answers))
}
