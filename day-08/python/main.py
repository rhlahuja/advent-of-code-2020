import pathlib
from dataclasses import dataclass, field

with open(pathlib.Path(__file__).parent.parent / 'input.txt') as f:
    instructions = f.read().splitlines()


@dataclass
class Program:
    instructions: list[str]
    executed_instruction_indices: set[int] = field(default_factory=set)
    accumulator: int = 0

    def run(self, index: int) -> tuple[bool, int]:
        for instruction in self.instructions[index:]:
            if index in self.executed_instruction_indices:
                return False, self.accumulator
            operation, value = instruction.split(' ')
            self.executed_instruction_indices.add(index)
            if operation == 'acc':
                self.accumulator += int(value)
            elif operation == 'jmp':
                return self.run(index + int(value))
            index += 1
        else:
            return True, self.accumulator


def flip_instruction(index: int, instructions: list[str]) -> list[str]:
    old_instruction = instructions[index]
    new_instructions = instructions.copy()
    if old_instruction.startswith('nop'):
        new_instructions[index] = old_instruction.replace('nop', 'jmp')
    elif old_instruction.startswith('jmp'):
        new_instructions[index] = old_instruction.replace('jmp', 'nop')
    return new_instructions


def fix_program(instructions: list[str]) -> int:
    for index, instruction in enumerate(instructions):
        if instruction.startswith(('nop', 'jmp')):
            program_terminates, accmumlator = Program(
                flip_instruction(index, instructions)
            ).run(0)
            if program_terminates:
                return accmumlator


part_one_solution = Program(instructions).run(0)[1]
part_two_solution = fix_program(instructions)

print('Part One:', part_one_solution)
print('Part Two:', part_two_solution)

assert part_one_solution == 1782
assert part_two_solution == 797
