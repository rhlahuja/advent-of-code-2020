import pathlib


def part_one(numbers: list[int]) -> int:
    for num1 in numbers:
        num2 = 2020 - num1
        if num2 in numbers:
            return num1 * num2


def part_two(numbers: list[int]) -> int:
    for num1 in numbers:
        for num2 in numbers:
            num3 = 2020 - num1 - num2
            if num3 in numbers:
                return num1 * num2 * num3


with open(pathlib.Path(__file__).parent.parent / 'input.txt') as f:
    numbers = [int(line) for line in f]

print('Part One: ', part_one(numbers))
print('Part Two: ', part_two(numbers))
