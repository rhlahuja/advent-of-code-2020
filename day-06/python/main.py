import pathlib
from collections import Counter


with open(pathlib.Path(__file__).parent.parent / 'input.txt') as f:
    group_answers = f.read().split('\n\n')


print(
    'Part One:', sum(len(set(answers.replace('\n', ''))) for answers in group_answers)
)

sum_all_yes_questions = 0
for answers in group_answers:
    individual_answers = answers.splitlines()
    counts = sum((Counter(answer) for answer in individual_answers), Counter())
    sum_all_yes_questions += sum(
        bool(k) for k, v in counts.items() if v == len(individual_answers)
    )
print('Part Two:', sum_all_yes_questions)
