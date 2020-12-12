#[macro_use]
extern crate lazy_static;

use std::fs;
use std::path::Path;

use itertools::Itertools;

const OCCUPIED_SEAT: char = '#';
const EMPTY_SEAT: char = 'L';
const FLOOR: char = '.';

lazy_static! {
    static ref DIRECTION_UNIT_DELTAS: Vec<(i32, i32)> = (-1..2)
        .cartesian_product(-1..2)
        .filter(|(x, y)| !(*x == 0 && *y == 0))
        .collect();
}

fn get_occupied_seat_total(seat_layout: Vec<Vec<char>>) -> usize {
    seat_layout
        .iter()
        .map(|row| row.iter().filter(|seat| **seat == OCCUPIED_SEAT).count())
        .sum()
}

fn occupied_adjacent_seats(x: usize, y: usize, seat_layout: &[Vec<char>]) -> usize {
    DIRECTION_UNIT_DELTAS
        .iter()
        .map(|(x_delta, y_delta)| (x as i32 + x_delta, y as i32 + y_delta))
        .filter(|(adjacent_x, adjacent_y)| {
            let (adjacent_x, adjacent_y) = (*adjacent_x as usize, *adjacent_y as usize);
            (0..seat_layout.len()).contains(&(adjacent_y))
                && (0..seat_layout[0].len()).contains(&(adjacent_x))
                && seat_layout[adjacent_y][adjacent_x] == OCCUPIED_SEAT
        })
        .count()
}

fn occupied_visible_seats(x: usize, y: usize, seat_layout: &[Vec<char>]) -> usize {
    let rows = seat_layout.len();
    let columns = seat_layout[0].len();

    let mut occupied_visible_seat_count = 0;

    for (x_delta, y_delta) in DIRECTION_UNIT_DELTAS.iter() {
        let (mut visible_x, mut visible_y) = (x as i32 + x_delta, y as i32 + y_delta);
        while (0..columns).contains(&(visible_x as usize))
            && (0..rows).contains(&(visible_y as usize))
        {
            match seat_layout[visible_y as usize][visible_x as usize] {
                FLOOR => {
                    visible_x += x_delta;
                    visible_y += y_delta;
                    continue;
                }
                OCCUPIED_SEAT => occupied_visible_seat_count += 1,
                _ => (),
            }
            break;
        }
    }
    occupied_visible_seat_count
}

fn execute_step(
    mut seat_layout: Vec<Vec<char>>,
    occupancy_criteria: fn(usize, usize, &[Vec<char>]) -> usize,
    occupancy_threshold: usize,
) -> Vec<Vec<char>> {
    let mut pending_operations: Vec<((usize, usize), char)> = vec![];
    for y in 0..seat_layout.len() {
        for x in 0..seat_layout[0].len() {
            let this_seat = seat_layout[y][x];
            let occupied_seat_count = occupancy_criteria(x, y, &seat_layout);
            match this_seat {
                EMPTY_SEAT => {
                    if occupied_seat_count == 0 {
                        pending_operations.push(((x, y), OCCUPIED_SEAT));
                    }
                }
                OCCUPIED_SEAT => {
                    if occupied_seat_count >= occupancy_threshold {
                        pending_operations.push(((x, y), EMPTY_SEAT));
                    }
                }
                _ => (),
            }
        }
    }

    for (indices, value) in pending_operations.iter() {
        seat_layout[indices.1][indices.0] = *value;
    }

    seat_layout
}

fn execute(
    seat_layout: &[Vec<char>],
    occupancy_criteria: fn(usize, usize, &[Vec<char>]) -> usize,
    occupancy_threshold: usize,
) -> usize {
    let (mut old_seat_layout, mut new_seat_layout) = (vec![], seat_layout.to_vec());
    while !old_seat_layout.iter().eq(new_seat_layout.iter()) {
        old_seat_layout = new_seat_layout;
        new_seat_layout = execute_step(
            old_seat_layout.clone(),
            occupancy_criteria,
            occupancy_threshold,
        );
    }
    get_occupied_seat_total(new_seat_layout)
}

fn main() {
    let seat_layout: Vec<Vec<_>> = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("input.txt"),
    )
    .unwrap()
    .lines()
    .map(|l| l.parse::<String>().unwrap().chars().collect())
    .collect();

    let part_one_solution = execute(seat_layout.as_slice(), occupied_adjacent_seats, 4);
    let part_two_solution = execute(seat_layout.as_slice(), occupied_visible_seats, 5);

    println!("Part One: {}", &part_one_solution);
    println!("Part Two: {}", &part_two_solution);

    assert_eq!(part_one_solution, 2453);
    assert_eq!(part_two_solution, 2159);
}
