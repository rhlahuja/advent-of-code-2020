import re
import pathlib


def contains_bag(
    enclosing_bag_color: str, contained_bag_color: str, bag_rules: dict[str, dict]
) -> bool:
    return any(
        contained_bag_color in bag_rules[enclosing_bag_color]
        or contains_bag(color, contained_bag_color, bag_rules)
        for color in bag_rules[enclosing_bag_color]
    )


def get_bag_count(bag_color: str, bag_rules: dict[str, dict]) -> int:
    return sum(
        bag_count + bag_count * get_bag_count(color, bag_rules)
        for color, bag_count in bag_rules[bag_color].items()
    )


with open(pathlib.Path(__file__).parent.parent / 'input.txt') as f:
    bag_rules_data = [line.split(' bags contain ') for line in f.read().splitlines()]

bag_rules = {}
for bag_color, rules in bag_rules_data:
    bag_color_rules = {}
    for rule in rules.split(', '):
        rule = rule.rstrip('.')
        if rule == 'no other bags':
            break
        match = re.match(r'(\d+) (.*) bags?', rule)
        bag_color_rules[match.group(2)] = int(match.group(1))
    bag_rules[bag_color] = bag_color_rules


part_one_solution = sum(
    (contains_bag(bag_color, 'shiny gold', bag_rules) for bag_color in bag_rules.keys())
)
part_two_solution = get_bag_count('shiny gold', bag_rules)

print('Part One:', part_one_solution)
print('Part Two:', part_two_solution)

assert part_one_solution == 289
assert part_two_solution == 30055
