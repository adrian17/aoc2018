from collections import defaultdict

lines = open("input.txt").read().splitlines()

def parse(string):
    string = string[2:]
    if '..' in string:
        low, high = map(int, string.split('..'))
        return list(range(low, high+1))
    return [int(string)]

ground = defaultdict(lambda: '.')

for line in lines:
    x_part, y_part = sorted(line.split(', '))
    x_range = parse(x_part)
    y_range = parse(y_part)
    for x in x_range:
        for y in y_range:
            ground[(x, y)] = '#'

min_y = min(y for x, y in ground)
max_y = max(y for x, y in ground)

def display(ground):
    min_x = min(x for x, y in ground)
    max_x = max(x for x, y in ground)    
    for y in range(0, max_y+1):
        for x in range(min_x, max_x+1):
            print(ground[(x, y)], end="")
        print()
    print()


streams_to_process = [(500, 0)]

def fill_side(stream, direction):
    while True:
        stream = (stream[0]+direction, stream[1])
        if ground[stream] == '#':
            return False
        under = (stream[0], stream[1]+1)
        edge = (under[0] - direction, under[1])
        if ground[under] == '.':
            if ground[edge] == '#':
                streams_to_process.append(stream)
            return True
        ground[stream] = '~'

def fill_shallow(stream, direction):
    while ground[stream] not in '.#':
        ground[stream] = '-'
        stream = (stream[0]+direction, stream[1])

def process(stream):
    while ground[stream] != '#':
        ground[stream] = '|'
        stream = (stream[0], stream[1]+1)
        if stream[1] > max_y:
            return
    found_edge = False
    while not found_edge:
        stream = (stream[0], stream[1]-1)
        ground[stream] = '~'
        if fill_side(stream, 1): found_edge = True
        if fill_side(stream, -1): found_edge = True
        if found_edge:
            fill_shallow(stream, 1)
            fill_shallow(stream, -1)

while streams_to_process:
    stream = streams_to_process.pop()
    process(stream)

#display(ground)

print(len([v for k, v in ground.items() if v in '~|-' and k[1] >= min_y]))
print(len([v for k, v in ground.items() if v in '~' and k[1] >= min_y]))
