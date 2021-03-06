import pathlib
from collections import deque
from dataclasses import dataclass


@dataclass
class Waypoint:
    relative_east_position: int = 10
    relative_north_position: int = 1

    def move_east(self, units: int):
        self.relative_east_position += units

    def move_west(self, units: int):
        self.relative_east_position -= units

    def move_north(self, units: int):
        self.relative_north_position += units

    def move_south(self, units: int):
        self.relative_north_position -= units

    def rotate_right_around_ship(self, degrees: int):
        for _ in range(degrees // 90):
            self.relative_east_position, self.relative_north_position = (
                self.relative_north_position,
                -self.relative_east_position,
            )

    def rotate_left_around_ship(self, degrees: int):
        for _ in range(degrees // 90):
            self.relative_east_position, self.relative_north_position = (
                -self.relative_north_position,
                self.relative_east_position,
            )


class Ship:
    def __init__(self, action_map_part: int):
        self.waypoint = Waypoint()
        self.cardinal_directions = deque(('E', 'N', 'W', 'S'))
        self.east_position = 0
        self.north_position = 0
        self.action_map_part = action_map_part
        self.action_maps = {
            1: {
                'N': self.move_north,
                'E': self.move_east,
                'S': self.move_south,
                'W': self.move_west,
                'L': self.turn_left,
                'R': self.turn_right,
                'F': self.move_forward,
            },
            2: {
                'N': self.waypoint.move_north,
                'E': self.waypoint.move_east,
                'S': self.waypoint.move_south,
                'W': self.waypoint.move_west,
                'L': self.waypoint.rotate_left_around_ship,
                'R': self.waypoint.rotate_right_around_ship,
                'F': self.move_toward_waypoint,
            },
        }

    @property
    def current_direction(self):
        return self.cardinal_directions[0]

    def move_east(self, units: int):
        self.east_position += units

    def move_west(self, units: int):
        self.east_position -= units

    def move_north(self, units: int):
        self.north_position += units

    def move_south(self, units: int):
        self.north_position -= units

    def turn_left(self, degrees: int):
        self.cardinal_directions.rotate(-degrees // 90)

    def turn_right(self, degrees: int):
        self.cardinal_directions.rotate(degrees // 90)

    def move_forward(self, units: int):
        self.action_maps[self.action_map_part][self.current_direction](units)

    def move_toward_waypoint(self, units: int):
        self.east_position += units * self.waypoint.relative_east_position
        self.north_position += units * self.waypoint.relative_north_position

    def process_instruction(self, action: str, value: int):
        self.action_maps[self.action_map_part][action](value)

    @property
    def manhattan_distance(self) -> int:
        return abs(self.east_position) + abs(self.north_position)


def compute_manhattan_distance(instructions: list[tuple[str, int]], part: int) -> int:
    ship = Ship(part)
    for action, value in instructions:
        ship.process_instruction(action, value)
    return ship.manhattan_distance


with open(pathlib.Path(__file__).parent.parent / 'input.txt') as f:
    instructions = [(line[0], int(line[1:])) for line in f.read().splitlines()]

part_one_solution = compute_manhattan_distance(instructions, 1)
part_two_solution = compute_manhattan_distance(instructions, 2)

print('Part One:', part_one_solution)
print('Part Two:', part_two_solution)

assert part_one_solution == 1106
assert part_two_solution == 107281
