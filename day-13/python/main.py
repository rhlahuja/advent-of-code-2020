import pathlib
import itertools


def find_earliest_bus(
    earliest_timestamp: int, bus_id_indices: list[tuple[int, int]]
) -> int:
    for timestamp in itertools.count(start=earliest_timestamp):
        for _, bus_id in bus_id_indices:
            if timestamp % bus_id == 0:
                return bus_id * (timestamp - earliest_timestamp)


def find_earliest_valid_timestamp(bus_id_indices: list[tuple[int, int]]) -> int:
    earliest_valid_timestamp = step = 1
    for index, bus_id in bus_id_indices:
        earliest_valid_timestamp = next(
            timestamp
            for timestamp in itertools.count(earliest_valid_timestamp, step)
            if (timestamp + index) % bus_id == 0
        )
        step *= bus_id
    return earliest_valid_timestamp


with open(pathlib.Path(__file__).parent.parent / 'input.txt') as f:
    lines = f.read().splitlines()
    bus_id_indices = [
        (index, int(bus_id))
        for index, bus_id in enumerate(lines[1].split(','))
        if bus_id != 'x'
    ]

part_one_solution = find_earliest_bus(int(lines[0]), bus_id_indices)
part_two_solution = find_earliest_valid_timestamp(bus_id_indices)

print('Part One:', part_one_solution)
print('Part Two:', part_two_solution)

assert part_one_solution == 1835
assert part_two_solution == 247086664214628
