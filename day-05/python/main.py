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

print('Part One: ', max(seat_ids))
print(
    'Part Two: ',
    [s + 1 for s in seat_ids if s + 1 not in seat_ids and s + 2 in seat_ids][0],
)
