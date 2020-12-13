use std::fs;
use std::path::Path;

fn find_earliest_bus(earliest_timestamp: usize, bus_id_indices: &[(usize, usize)]) -> usize {
    for timestamp in earliest_timestamp.. {
        for (_, bus_id) in bus_id_indices {
            if timestamp % bus_id == 0 {
                return bus_id * (timestamp - earliest_timestamp);
            }
        }
    }
    0
}

fn find_earliest_valid_timestamp(bus_id_indices: &[(usize, usize)]) -> usize {
    let (mut earliest_valid_timestamp, mut step) = (1, 1);
    for (index, bus_id) in bus_id_indices {
        earliest_valid_timestamp = (earliest_valid_timestamp..)
            .step_by(step)
            .find(|timestamp| (timestamp + index) % bus_id == 0)
            .unwrap();
        step *= bus_id
    }
    earliest_valid_timestamp
}

fn main() {
    let input: Vec<String> = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("input.txt"),
    )
    .unwrap()
    .lines()
    .map(|l| l.parse().unwrap())
    .collect();

    let earliest_timestamp = input[0].parse().unwrap();
    let bus_indices: Vec<_> = input[1]
        .split(',')
        .enumerate()
        .filter(|(_, bus_id)| *bus_id != "x")
        .map(|(index, bus_id)| (index, bus_id.parse().unwrap()))
        .collect();

    let part_one_solution = find_earliest_bus(earliest_timestamp, &bus_indices);
    let part_two_solution = find_earliest_valid_timestamp(&bus_indices);

    println!("Part One: {}", part_one_solution);
    println!("Part Two: {}", part_two_solution);

    assert_eq!(part_one_solution, 1835);
    assert_eq!(part_two_solution, 247086664214628);
}
