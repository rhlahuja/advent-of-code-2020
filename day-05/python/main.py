import pathlib


def binary_search(letters: str, first_half_letter: str, range_max: int) -> int:
    possible_values = list(range(range_max))
    for letter in letters:
        middle_idx = int(len(possible_values) / 2)
        possible_values = (
            possible_values[:middle_idx]
            if letter == first_half_letter
            else possible_values[middle_idx:]
        )
    return possible_values[0]


def get_seat_id(letters: str) -> int:
    row_number = binary_search(letters[:7], 'F', 128)
    column_number = binary_search(letters[7:], 'L', 8)
    return row_number * 8 + column_number


with open(pathlib.Path(__file__).parent.parent / 'input.txt') as f:
    seat_ids = [get_seat_id(letters) for letters in f.read().splitlines()]


part_one_solution = max(seat_ids)
part_two_solution = [
    s + 1 for s in seat_ids if s + 1 not in seat_ids and s + 2 in seat_ids
][0]

print('Part One:', part_one_solution)
print('Part Two:', part_two_solution)

assert part_one_solution == 883
assert part_two_solution == 532
