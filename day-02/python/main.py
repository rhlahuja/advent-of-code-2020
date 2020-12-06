import pathlib
from collections.abc import Callable


def part_one_validator(policy: str, password: str) -> bool:
    counts, character = policy.split(' ')
    min_count, max_count = [int(c) for c in counts.split('-')]
    return min_count <= password.count(character) <= max_count


def part_two_validator(policy: str, password: str) -> bool:
    positions, character = policy.split(' ')
    index1, index2 = [int(c) - 1 for c in positions.split('-')]
    return (password[index1] == character) ^ (password[index2] == character)


def sum_valid_passwords(
    password_validator: Callable[[str, str], bool],
    policies_and_passwords: list[list[str, str]],
):
    return sum(
        password_validator(policy, password)
        for policy, password in policies_and_passwords
    )


with open(pathlib.Path(__file__).parent.parent / 'input.txt') as f:
    policies_and_passwords = [line.split(': ') for line in f.read().splitlines()]

part_one_solution = sum_valid_passwords(part_one_validator, policies_and_passwords)
part_two_solution = sum_valid_passwords(part_two_validator, policies_and_passwords)

print('Part One:', part_one_solution)
print('Part Two:', part_two_solution)

assert part_one_solution == 410
assert part_two_solution == 694
