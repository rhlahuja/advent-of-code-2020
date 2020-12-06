use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let input = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("input.txt"),
    )
    .unwrap();
    let group_answers: Vec<&str> = input.split("\n\n").collect();

    println!(
        "Part One: {}",
        group_answers
            .iter()
            .fold(0, |sum_one_yes_answers, group_answer| {
                let mut group_answer: Vec<char> = group_answer.replace("\n", "").chars().collect();
                group_answer.sort_unstable();
                group_answer.dedup();
                sum_one_yes_answers + group_answer.len()
            })
    );
    println!(
        "Part Two: {}",
        group_answers
            .iter()
            .fold(0, |sum_all_yes_answers, answers| {
                sum_all_yes_answers
                    + answers
                        .replace("\n", "")
                        .chars()
                        .fold(HashMap::new(), |mut character_counts, character| {
                            *character_counts.entry(character).or_insert(0) += 1;
                            character_counts
                        })
                        .into_iter()
                        .filter(|&(_character, count)| count == answers.matches('\n').count() + 1)
                        .collect::<HashMap<char, usize>>()
                        .keys()
                        .len()
            })
    );
}
