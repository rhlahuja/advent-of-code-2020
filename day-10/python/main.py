import pathlib
from collections import defaultdict


def find_joltage_differences(adapter_jolts: list[int]) -> int:
    jolt_differnces = defaultdict(int)
    for index, jolts in enumerate(adapter_jolts[:-1]):
        jolt_differnces[adapter_jolts[index + 1] - jolts] += 1
    return jolt_differnces[1] * jolt_differnces[3]


with open(pathlib.Path(__file__).parent.parent / 'input.txt') as f:
    adapter_jolts = sorted(int(line) for line in f.read().splitlines())

adapter_jolts.insert(0, 0)
adapter_jolts.append(adapter_jolts[-1] + 3)

part_one_solution = find_joltage_differences(adapter_jolts)

print('Part One:', part_one_solution)

assert part_one_solution == 2112
