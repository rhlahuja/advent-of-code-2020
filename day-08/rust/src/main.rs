use itertools::Itertools;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

#[derive(Default)]
struct Program {
    instructions: Vec<String>,
    executed_instruction_indices: HashSet<usize>,
    accumulator: i32,
}

impl Program {
    fn run(&mut self, mut index: usize) -> (bool, &i32) {
        for instruction in &self.instructions[index..] {
            if self.executed_instruction_indices.contains(&(index)) {
                return (false, &self.accumulator);
            }
            let (operation, value) = &instruction.split(' ').collect_tuple().unwrap();
            let value = value.parse::<i32>().unwrap();
            self.executed_instruction_indices.insert(index);
            match *operation {
                "acc" => self.accumulator += value,
                "jmp" => {
                    return self.run((index as i32 + value) as usize);
                }
                "nop" => (),
                _ => panic!(
                    "unrecognized instruction '{}' at index {}",
                    &instruction, index
                ),
            }
            index += 1;
        }
        (true, &self.accumulator)
    }
}

fn flip_instruction(index: usize, instructions: &[String]) -> Vec<String> {
    let old_instruction = &instructions[index];
    let mut new_instructions = instructions.to_vec();
    match &old_instruction[..3] {
        "jmp" => {
            new_instructions[index] = old_instruction.replace("jmp", "nop");
        }
        "nop" => {
            new_instructions[index] = old_instruction.replace("nop", "jmp");
        }
        _ => (),
    };
    new_instructions
}

fn fix_program(instructions: &[String]) -> i32 {
    for (index, instruction) in instructions.iter().enumerate() {
        if instruction.starts_with("jmp") || instruction.starts_with("nop") {
            let mut new_program = Program {
                instructions: flip_instruction(index, &instructions),
                ..Default::default()
            };
            let (program_terminates, accumulator) = new_program.run(0);
            if program_terminates {
                return *accumulator;
            }
        }
    }
    0
}

fn main() {
    let input = fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("input.txt"),
    )
    .unwrap();
    let instructions: Vec<String> = input.lines().map(|l| l.to_string()).collect();

    let mut part_one_program = Program {
        instructions: instructions.clone(),
        ..Default::default()
    };
    let part_one_solution = *part_one_program.run(0).1;
    let part_two_solution = fix_program(&instructions);

    println!("Part One: {}", &part_one_solution);
    println!("Part Two: {}", &part_two_solution);

    assert_eq!(part_one_solution, 1782);
    assert_eq!(part_two_solution, 797);
}
