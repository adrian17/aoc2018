from tqdm import tqdm

lines = open("input.txt").read().splitlines()

H = len(lines)
W = len(lines[0])

grid = {}

for y, line in enumerate(lines):
	for x, c in enumerate(line):
		grid[(x, y)] = c

def step(grid):
	new_grid = {}
	for y in range(H):
		for x in range(W):
			neighbors = [
				grid.get((x, y+1), '.'),
				grid.get((x, y-1), '.'),
				grid.get((x+1, y), '.'),
				grid.get((x-1, y), '.'),
				grid.get((x+1, y+1), '.'),
				grid.get((x+1, y-1), '.'),
				grid.get((x-1, y+1), '.'),
				grid.get((x-1, y-1), '.'),
			]
			spot = grid[(x, y)]
			new_spot = spot
			if spot == '.' and neighbors.count('|') >= 3:
				new_spot = '|'
			elif spot == '|' and neighbors.count('#') >= 3:
				new_spot = '#'
			elif spot == '#' and (neighbors.count('#') == 0 or neighbors.count('|') == 0):
				new_spot = '.'
			new_grid[(x, y)] = new_spot
	return new_grid

def score(grid):
	trees = len([v for v in grid.values() if v == '|'])
	lumberyards = len([v for v in grid.values() if v == '#'])
	print(trees * lumberyards)

def grid_hashable(grid):
	return ''.join(grid[(x, y)] for y in range(H) for x in range(W))

seen = {}

for i in range(1000000000):
	hashable = grid_hashable(grid)
	if hashable in seen:
		difference = i - seen[hashable]
		break
	seen[hashable] = i
	grid = step(grid)
	if i == 9:
		score(grid)

to_add = (1000000000 - i) % difference
for i in range(to_add):
	grid = step(grid)

score(grid)