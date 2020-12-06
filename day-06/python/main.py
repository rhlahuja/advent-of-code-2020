import pathlib
from collections import Counter


with open(pathlib.Path(__file__).parent.parent / 'input.txt') as f:
    group_answers = f.read().split('\n\n')

print(
    'Part One:', sum(len(set(answers.replace('\n', ''))) for answers in group_answers)
)
print(
    'Part Two:',
    sum(
        sum(
            bool(answer)
            for answer, answer_count in Counter(answers.replace('\n', '')).items()
            if answer_count == answers.count('\n') + 1
        )
        for answers in group_answers
    ),
)
