import pathlib
import itertools


def find_inconsistent_number(numbers: list[int], preamble_length: int = 25) -> int:
    current_index = preamble_length
    for number in numbers[preamble_length:]:
        for pair in itertools.combinations(
            numbers[current_index - preamble_length : current_index], 2
        ):
            if number == sum(pair):
                break
        else:
            return number
        current_index += 1


def sum_min_max_operands(numbers: list[int], target_sum: int) -> int:
    min_index = max_index = 0
    while min_index < len(numbers):
        operands = numbers[min_index:max_index]
        continguous_sum = sum(operands)
        if continguous_sum < target_sum:
            max_index += 1
        elif continguous_sum > target_sum:
            min_index += 1
        else:
            return min(operands) + max(operands)


with open(pathlib.Path(__file__).parent.parent / 'input.txt') as f:
    numbers = [int(line) for line in f.read().splitlines()]

part_one_solution = find_inconsistent_number(numbers)
part_two_solution = sum_min_max_operands(numbers, part_one_solution)

print('Part One:', part_one_solution)
print('Part Two:', part_two_solution)

assert part_one_solution == 15353384
assert part_two_solution == 2466556
