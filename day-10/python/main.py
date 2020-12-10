import pathlib
from collections import defaultdict


def find_joltage_differences_product(adapters: list[int]) -> int:
    jolt_differences = defaultdict(int)
    for index, adapter_jolts in enumerate(adapters[:-1]):
        jolt_differences[adapters[index + 1] - adapter_jolts] += 1
    return jolt_differences[1] * jolt_differences[3]


def find_num_valid_permutations(adapters: list[int]) -> int:
    num_valid_permutations = {0: 1}
    for adapter_jolts in adapters[1:]:
        num_valid_permutations[adapter_jolts] = sum(
            num_valid_permutations.get(adapter_jolts - delta, 0)
            for delta in range(1, 4)
        )
    return num_valid_permutations[adapters[-1]]


with open(pathlib.Path(__file__).parent.parent / 'input.txt') as f:
    adapters = sorted(int(line) for line in f.read().splitlines())

adapters.insert(0, 0)
adapters.append(adapters[-1] + 3)

part_one_solution = find_joltage_differences_product(adapters)
part_two_solution = find_num_valid_permutations(adapters)

print('Part One:', part_one_solution)
print('Part Two:', part_two_solution)

assert part_one_solution == 2112
assert part_two_solution == 3022415986688
