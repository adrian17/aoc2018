from collections import defaultdict
import ast

hints = open('hints.txt').read().splitlines()
program = open('program.txt').read().splitlines()

instructions = {
    'addr': lambda regs, a, b: regs[a] + regs[b],
    'addi': lambda regs, a, b: regs[a] + b,
    'mulr': lambda regs, a, b: regs[a] * regs[b],
    'muli': lambda regs, a, b: regs[a] * b,
    'banr': lambda regs, a, b: regs[a] & regs[b],
    'bani': lambda regs, a, b: regs[a] & b,
    'borr': lambda regs, a, b: regs[a] | regs[b],
    'bori': lambda regs, a, b: regs[a] | b,
    'setr': lambda regs, a, b: regs[a],
    'seti': lambda regs, a, b: a,
    'gtir': lambda regs, a, b: int(a > regs[b]),
    'gtri': lambda regs, a, b: int(regs[a] > b),
    'gtrr': lambda regs, a, b: int(regs[a] > regs[b]),
    'eqir': lambda regs, a, b: int(a == regs[b]),
    'eqri': lambda regs, a, b: int(regs[a] == b),
    'eqrr': lambda regs, a, b: int(regs[a] == regs[b]),
}

possibilities = defaultdict(lambda: set(instructions.keys()))
over_three_matches = 0

chunks = [hints[i:i + 4] for i in range(0, len(hints), 4)]
for before, instruction, after, *_ in chunks:
    before = ast.literal_eval(before.split(':')[1].strip())
    after = ast.literal_eval(after.split(':')[1].strip())
    opcode, a, b, c = map(int, instruction.split())

    matches = []
    for name, implementation in instructions.items():
        regs = before.copy()
        regs[c] = implementation(regs, a, b)
        if regs == after:
            matches.append(name)
    possibilities[opcode] &= set(matches)
    if len(matches) >= 3:
        over_three_matches += 1

print(over_three_matches)

mapping = {}
while possibilities:
    opcode, name = [(k, list(v)[0]) for k, v in possibilities.items() if len(v) == 1][0]
    possibilities.pop(opcode)
    mapping[opcode] = name
    for guesses in possibilities.values():
        if name in guesses:
            guesses.remove(name)

regs = [0, 0, 0, 0]
for line in program:
    opcode, a, b, c = map(int, line.split())
    regs[c] = instructions[mapping[opcode]](regs, a, b)

print(regs)
