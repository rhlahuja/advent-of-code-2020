import pathlib
from collections import Counter


with open(pathlib.Path(__file__).parent.parent / 'input.txt') as f:
    group_answers = f.read().split('\n\n')


part_one_solution = sum(
    len(set(answers.replace('\n', ''))) for answers in group_answers
)
part_two_solution = sum(
    sum(
        bool(answer)
        for answer, answer_count in Counter(answers.replace('\n', '')).items()
        if answer_count == answers.count('\n') + 1
    )
    for answers in group_answers
)

print('Part One:', part_one_solution)
print('Part Two:', part_two_solution)

assert part_one_solution == 6799
assert part_two_solution == 3354
