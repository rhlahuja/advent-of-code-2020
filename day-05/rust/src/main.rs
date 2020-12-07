use std::env;
use std::fs;
use std::path::Path;

fn binary_search(letters: &str, first_half_letter: char, range_max: u32) -> u32 {
    let mut possible_values: Vec<u32> = (0..range_max).collect();

    for character in letters.chars() {
        let (first, last) = possible_values.split_at(possible_values.len() / 2);

        if character == first_half_letter {
            possible_values = first.to_vec();
        } else {
            possible_values = last.to_vec();
        }
    }
    possible_values[0]
}

fn get_seat_id(letters: &str) -> u32 {
    let row_number = binary_search(&letters[0..7], 'F', 128);
    let column_number = binary_search(&letters[7..], 'L', 8);
    row_number * 8 + column_number
}

fn main() {
    let input = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("input.txt"),
    )
    .unwrap();

    let mut seat_ids: Vec<_> = Vec::new();
    for line in input.lines() {
        seat_ids.push(get_seat_id(&line));
    }

    let part_one_solution = *seat_ids.iter().max().unwrap();
    let part_two_solution = seat_ids
        .iter()
        .find(|seat_id| !&seat_ids.contains(&(*seat_id + 1)) && seat_ids.contains(&(*seat_id + 2)))
        .unwrap()
        + 1;

    println!("Part One: {}", part_one_solution);
    println!("Part Two: {:?}", part_two_solution);

    assert_eq!(part_one_solution, 883);
    assert_eq!(part_two_solution, 532);
}
