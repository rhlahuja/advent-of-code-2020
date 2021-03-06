use std::collections::VecDeque;
use std::fs;
use std::mem;
use std::path::Path;

struct Waypoint {
    pub relative_east_position: i32,
    pub relative_north_position: i32,
}

impl Default for Waypoint {
    fn default() -> Self {
        Waypoint {
            relative_east_position: 10,
            relative_north_position: 1,
        }
    }
}

struct Ship {
    cardinal_directions: VecDeque<char>,
    east_position: i32,
    north_position: i32,
    waypoint: Waypoint,
    pub action_map_part: u32,
}

impl Default for Ship {
    fn default() -> Self {
        Ship {
            cardinal_directions: vec!['E', 'N', 'W', 'S'].into_iter().collect(),
            east_position: 0,
            north_position: 0,
            waypoint: Waypoint::default(),
            action_map_part: 0,
        }
    }
}

impl Ship {
    fn current_direction(&self) -> char {
        self.cardinal_directions[0]
    }

    fn manhattan_distance(&self) -> u32 {
        (self.east_position.abs() + self.north_position.abs()) as u32
    }

    fn process_instruction(&mut self, action: char, value: i32) {
        match &self.action_map_part {
            1 => match action {
                'N' => self.north_position += value,
                'S' => self.north_position -= value,
                'E' => self.east_position += value,
                'W' => self.east_position -= value,
                'R' => self.cardinal_directions.rotate_right((value / 90) as usize),
                'L' => self.cardinal_directions.rotate_left((value / 90) as usize),
                'F' => self.process_instruction(self.current_direction(), value),
                _ => panic!("unrecognized instruction '{}{}'", action, value),
            },
            2 => match action {
                'N' => self.waypoint.relative_north_position += value,
                'S' => self.waypoint.relative_north_position -= value,
                'E' => self.waypoint.relative_east_position += value,
                'W' => self.waypoint.relative_east_position -= value,
                'R' => {
                    for _ in 0..(value / 90) as usize {
                        mem::swap(
                            &mut self.waypoint.relative_east_position,
                            &mut self.waypoint.relative_north_position,
                        );
                        self.waypoint.relative_north_position =
                            -self.waypoint.relative_north_position;
                    }
                }
                'L' => {
                    for _ in 0..(value / 90) as usize {
                        mem::swap(
                            &mut self.waypoint.relative_east_position,
                            &mut self.waypoint.relative_north_position,
                        );
                        self.waypoint.relative_east_position =
                            -self.waypoint.relative_east_position;
                    }
                }
                'F' => {
                    self.east_position += value * self.waypoint.relative_east_position;
                    self.north_position += value * self.waypoint.relative_north_position;
                }
                _ => panic!("unrecognized instruction '{}{}'", action, value),
            },
            _ => unreachable!(),
        }
    }
}

fn compute_manhattan_distance(instructions: &[(char, i32)], part: u32) -> u32 {
    let mut ship = Ship {
        action_map_part: part,
        ..Ship::default()
    };
    for (action, value) in instructions.iter() {
        ship.process_instruction(*action, *value);
    }
    ship.manhattan_distance()
}

fn main() {
    let input = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("input.txt"),
    )
    .unwrap();
    let instructions: Vec<_> = input
        .lines()
        .map(|l| {
            let (action, value) = l.split_at(1);
            (
                action.chars().next().unwrap(),
                value.parse::<i32>().unwrap(),
            )
        })
        .collect();

    let part_one_solution = compute_manhattan_distance(&instructions, 1);
    let part_two_solution = compute_manhattan_distance(&instructions, 2);

    println!("Part One: {}", part_one_solution);
    println!("Part Two: {}", part_two_solution);

    assert_eq!(part_one_solution, 1106);
    assert_eq!(part_two_solution, 107281);
}
