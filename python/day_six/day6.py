import string

with open('./resources/day6.txt', 'r') as f:

    dirty_data = f.read().split('\n\n')
    data = [[answers for answers in group.split("\n")] for group in dirty_data]
    
valid_answers = 0

for multi_entry in data:
    
    for group, answers in enumerate(multi_entry):
        validate = set(answers) if group == 0 else validate.intersection(set(answers))
    valid_answers += len(validate)

print(f'Total valid answers: {valid_answers}')