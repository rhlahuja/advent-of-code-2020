use std::collections::HashMap;
use std::fs;
use std::path::Path;

fn main() {
    let group_answers: Vec<String> = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("input.txt"),
    )
    .unwrap()
    .split("\n\n")
    .map(|l| l.parse().unwrap())
    .collect();

    let part_one_solution = group_answers
        .iter()
        .fold(0, |sum_one_yes_answers, group_answer| {
            let mut group_answer: Vec<_> = group_answer.replace("\n", "").chars().collect();
            group_answer.sort_unstable();
            group_answer.dedup();
            sum_one_yes_answers + group_answer.len()
        });
    let part_two_solution = group_answers
        .iter()
        .fold(0, |sum_all_yes_answers, group_answer| {
            sum_all_yes_answers
                + group_answer
                    .replace("\n", "")
                    .chars()
                    .fold(HashMap::new(), |mut character_counts, character| {
                        *character_counts.entry(character).or_insert(0) += 1;
                        character_counts
                    })
                    .into_iter()
                    .filter(|&(_character, count)| count == group_answer.matches('\n').count() + 1)
                    .collect::<HashMap<_, _>>()
                    .keys()
                    .len()
        });

    println!("Part One: {}", part_one_solution);
    println!("Part Two: {}", part_two_solution);

    assert_eq!(part_one_solution, 6799);
    assert_eq!(part_two_solution, 3354);
}
