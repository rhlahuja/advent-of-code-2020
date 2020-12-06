import re
import pathlib
from collections.abc import Callable


FIELD_RULES = {
    'required': {
        'byr': ({'pattern': r'^(\d{4})$', 'min': 1920, 'max': 2002},),
        'iyr': ({'pattern': r'^(\d{4})$', 'min': 2010, 'max': 2020},),
        'eyr': ({'pattern': r'^(\d{4})$', 'min': 2020, 'max': 2030},),
        'hgt': (
            {
                'pattern': r'^(\d*)(cm)$',
                'min': 150,
                'max': 193,
            },
            {
                'pattern': r'^(\d*)(in)$',
                'min': 59,
                'max': 76,
            },
        ),
        'ecl': ({'pattern': r'^(amb|blu|brn|gry|grn|hzl|oth)$'},),
        'hcl': ({'pattern': r'^#[0-9a-f]{6}$'},),
        'pid': ({'pattern': r'^(\d{9})$'},),
    },
    'optional': {'cid': ({'pattern': r'(.*)'},)},
}


def part_one_validator(passport: dict[str, str]) -> bool:
    return all(f in passport.keys() for f in FIELD_RULES['required'].keys())


def part_two_validator(passport: dict[str, str]) -> bool:
    def is_valid_field(field_value: str, rule: dict) -> bool:
        match = re.match(rule['pattern'], field_value)
        return (
            match
            and ('min' not in rule or int(match.group(1)) >= rule['min'])
            and ('max' not in rule or int(match.group(1)) <= rule['max'])
        )

    return part_one_validator(passport) and all(
        any(
            is_valid_field(field_value, rule)
            for rule in {**FIELD_RULES['required'], **FIELD_RULES['optional']}[
                field_key
            ]
        )
        for field_key, field_value in passport.items()
    )


def sum_valid_passports(
    passport_validator: Callable[[dict[str, str]], bool],
    passports: list[dict[str, str]],
) -> int:
    return sum(passport_validator(passport) for passport in passports)


with open(pathlib.Path(__file__).parent.parent / 'input.txt') as f:
    passports = [
        dict(field.split(':') for field in passport_fields)
        for passport_fields in [
            passport.replace('\n', ' ').split(' ')
            for passport in f.read().split('\n\n')
        ]
    ]

part_one_solution = sum_valid_passports(part_one_validator, passports)
part_two_solution = sum_valid_passports(part_two_validator, passports)

print('Part One:', part_one_solution)
print('Part Two:', part_two_solution)

assert part_one_solution == 233
assert part_two_solution == 111
