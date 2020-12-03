import pathlib


def part_one_validator(policy: str, password: str) -> bool:
    counts, character = policy.split(' ')
    min_count, max_count = [int(c) for c in counts.split('-')]
    return min_count <= password.count(character) <= max_count


def part_two_validator(policy: str, password: str) -> bool:
    positions, character = policy.split(' ')
    index1, index2 = [int(c) - 1 for c in positions.split('-')]
    return (password[index1] == character) ^ (password[index2] == character)


def sum_valid_passwords(
    password_validator, passwords_and_policies: list[list[str, str]]
):
    return sum(
        password_validator(policy, password)
        for policy, password in passwords_and_policies
    )


with open(pathlib.Path(__file__).parent.parent / 'input.txt') as f:
    passwords_and_policies = [line.split(': ') for line in f.read().splitlines()]

print('Part One: ', sum_valid_passwords(part_one_validator, passwords_and_policies))
print('Part Two: ', sum_valid_passwords(part_two_validator, passwords_and_policies))
