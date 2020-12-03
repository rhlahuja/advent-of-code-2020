use std::path::Path;

use std::env;
use std::fs;

fn part_one(numbers: &[i32]) -> i32 {
    for num1 in numbers {
        let num2 = 2020 - num1;
        if numbers.contains(&num2) {
            return num1 * num2;
        };
    }
    0
}

fn part_two(numbers: &[i32]) -> i32 {
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

fn main() {
    let input_filepath = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("input.txt");
    let input = fs::read_to_string(input_filepath).unwrap();

    let mut vec: Vec<i32> = Vec::new();
    for line in input.lines() {
        vec.push(line.parse::<i32>().unwrap());
    }

    println!("Part One: {}", part_one(vec.as_slice()));
    println!("Part Two: {}", part_two(vec.as_slice()));
}
