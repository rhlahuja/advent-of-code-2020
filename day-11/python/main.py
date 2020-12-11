import pathlib
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
    row_count = len(seat_layout)
    row_deltas = range(1, row_count)
    rows = range(row_count)
    column_count = len(seat_layout[0])
    column_deltas = range(1, column_count)
    columns = range(column_count)
    max_deltas = range(1, max(row_count, column_count))

    visible_indices = [
        [(right_idx, y) for i in column_deltas if (right_idx := x + i) in columns],
        [(left_idx, y) for i in column_deltas if (left_idx := x - i) in columns],
        [(x, up_idx) for i in row_deltas if (up_idx := y + i) in rows],
        [(x, down_idx) for i in row_deltas if (down_idx := y - i) in rows],
        [
            (right_idx, up_idx)
            for i in max_deltas
            if (right_idx := x + i) in columns and (up_idx := y + i) in rows
        ],
        [
            (left_idx, up_idx)
            for i in max_deltas
            if (left_idx := x - i) in columns and (up_idx := y + i) in rows
        ],
        [
            (left_idx, down_idx)
            for i in max_deltas
            if (left_idx := x - i) in columns and (down_idx := y - i) in rows
        ],
        [
            (right_idx, down_idx)
            for i in max_deltas
            if (right_idx := x + i) in columns and (down_idx := y - i) in rows
        ],
    ]

    occupied_visible_seat_count = 0
    for direction in visible_indices:
        for x, y in direction:
            seat = seat_layout[y][x]
            if seat == FLOOR:
                continue
            if seat == OCCUPIED_SEAT:
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


def run(
    seat_layout: list[list[str]], occupancy_criteria: Callable, occupancy_threshold: int
) -> int:
    while seat_layout != (
        new_seat_layout := execute_step(
            deepcopy(seat_layout), occupancy_criteria, occupancy_threshold
        )
    ):
        seat_layout = new_seat_layout
    return get_occuppied_seat_total(new_seat_layout)


with open(pathlib.Path(__file__).parent.parent / 'input.txt') as f:
    seat_layout = [list(line) for line in f.read().splitlines()]

part_one_solution = run(seat_layout, occupied_adjacent_seats, 4)
part_two_solution = run(seat_layout, occupied_visible_seats, 5)

print('Part One:', part_one_solution)
print('Part Two:', part_two_solution)

assert part_one_solution == 2453
assert part_two_solution == 2159
