import pathlib
from collections import Counter


def part_one(group_answers: list[str]) -> int:
    return sum(len(set(answers.replace('\n', ''))) for answers in group_answers)


def part_two(group_answers: list[str]) -> int:
    sum_all_yes_questions = 0
    for answers in group_answers:
        individual_answers = answers.splitlines()
        counts = sum((Counter(answer) for answer in individual_answers), Counter())
        sum_all_yes_questions += sum(
            bool(k) for k, v in counts.items() if v == len(individual_answers)
        )
    return sum_all_yes_questions


with open(pathlib.Path(__file__).parent.parent / 'input.txt') as f:
    group_answers = f.read().split('\n\n')

print('Part One:', part_one(group_answers))
print('Part Two:', part_two(group_answers))
