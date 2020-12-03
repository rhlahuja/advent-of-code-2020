import math
import pathlib
import itertools


def part_one_naive(numbers: list[int]) -> int:
    for num1 in numbers:
        num2 = 2020 - num1
        if num2 in numbers:
            return num1 * num2


def part_two_naive(numbers: list[int]) -> int:
    for num1 in numbers:
        for num2 in numbers:
            num3 = 2020 - num1 - num2
            if num3 in numbers:
                return num1 * num2 * num3


def itertools_combination_product(numbers: list[int], num_elements: int) -> int:
    expenses = [
        e for e in itertools.combinations(numbers, num_elements) if sum(e) == 2020
    ][0]
    return math.prod(expenses)


with open(pathlib.Path(__file__).parent.parent / 'input.txt') as f:
    numbers = [int(line) for line in f]

part_one_naive_solution = part_one_naive(numbers)
part_two_naive_solution = part_two_naive(numbers)

assert part_one_naive_solution == itertools_combination_product(numbers, 2)
assert part_two_naive_solution == itertools_combination_product(numbers, 3)

print('Part One: ', part_one_naive_solution)
print('Part Two: ', part_two_naive_solution)
