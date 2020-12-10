import pathlib
from collections import defaultdict


def find_joltage_differences_product(adapter_jolts: list[int]) -> int:
    jolt_differnces = defaultdict(int)
    for index, jolts in enumerate(adapter_jolts[:-1]):
        jolt_differnces[adapter_jolts[index + 1] - jolts] += 1
    return jolt_differnces[1] * jolt_differnces[3]


def find_valid_permutations(adapter_jolts: list[int]) -> int:
    valid_permutations = {0: 1}
    for jolts in adapter_jolts[1:]:
        valid_permutations[jolts] = sum(
            valid_permutations.get(jolts - delta, 0) for delta in range(1, 4)
        )
    return valid_permutations[adapter_jolts[-1]]


with open(pathlib.Path(__file__).parent.parent / 'input.txt') as f:
    adapter_jolts = sorted(int(line) for line in f.read().splitlines())

adapter_jolts.insert(0, 0)
adapter_jolts.append(adapter_jolts[-1] + 3)

part_one_solution = find_joltage_differences_product(adapter_jolts)
part_two_solution = find_valid_permutations(adapter_jolts)

print('Part One:', part_one_solution)
print('Part Two:', part_two_solution)

assert part_one_solution == 2112
assert part_one_solution == 3022415986688
