import pathlib
import itertools
from typing import Callable
from copy import deepcopy


OCCUPIED_SEAT = '#'
EMPTY_SEAT = 'L'
FLOOR = '.'


def get_occuppied_seat_total(seat_layout: list[list[str]]) -> int:
    return sum(row.count(OCCUPIED_SEAT) for row in seat_layout)


def occupied_adjacent_seats(x: int, y: int, seat_layout: list[list[str]]) -> int:
    adjacent_indices = [(x + i, y + j) for i in range(-1, 2) for j in range(-1, 2)]
    adjacent_indices.remove((x, y))
    return sum(
        seat_layout[y][x] == OCCUPIED_SEAT
        for x, y in adjacent_indices
        if x in range(len(seat_layout[0])) and y in range(len(seat_layout))
    )


def occupied_visible_seats(x: int, y: int, seat_layout: list[list[str]]) -> int:
    rows = range(len(seat_layout))
    columns = range(len(seat_layout[0]))
    visibile_direction_deltas = set(itertools.product(range(-1, 2), range(-1, 2)))
    visibile_direction_deltas.remove((0, 0))

    occupied_visible_seat_count = 0
    for x_delta, y_delta in visibile_direction_deltas:
        visible_x, visible_y = x + x_delta, y + y_delta
        while visible_x in columns and visible_y in rows:
            visible_seat = seat_layout[visible_y][visible_x]
            if visible_seat == FLOOR:
                visible_x += x_delta
                visible_y += y_delta
                continue
            if visible_seat == OCCUPIED_SEAT:
                occupied_visible_seat_count += 1
            break

    return occupied_visible_seat_count


def execute_step(
    seat_layout: list[list[str]], occupancy_criteria: Callable, occupancy_threshold: int
) -> list[list[str]]:
    pending_operations = []
    for y in range(len(seat_layout)):
        for x in range(len(seat_layout[0])):
            this_seat = seat_layout[y][x]
            occupied_seat_count = occupancy_criteria(x, y, seat_layout)
            if this_seat == EMPTY_SEAT and occupied_seat_count == 0:
                pending_operations.append(((x, y), OCCUPIED_SEAT))
            elif (
                this_seat == OCCUPIED_SEAT
                and occupied_seat_count >= occupancy_threshold
            ):
                pending_operations.append(((x, y), EMPTY_SEAT))
    for indices, value in pending_operations:
        seat_layout[indices[1]][indices[0]] = value
    return seat_layout


def execute(
    seat_layout: list[list[str]], occupancy_criteria: Callable, occupancy_threshold: int
) -> int:
    while seat_layout != (
        new_seat_layout := execute_step(
            deepcopy(seat_layout), occupancy_criteria, occupancy_threshold
        )
    ):
        seat_layout = new_seat_layout
    return get_occuppied_seat_total(seat_layout)


with open(pathlib.Path(__file__).parent.parent / 'input.txt') as f:
    seat_layout = [list(line.strip()) for line in f.read().splitlines()]

part_one_solution = execute(seat_layout, occupied_adjacent_seats, 4)
part_two_solution = execute(seat_layout, occupied_visible_seats, 5)

print('Part One:', part_one_solution)
print('Part Two:', part_two_solution)

assert part_one_solution == 2453
assert part_two_solution == 2159
