from collections import defaultdict

line = open('input.txt').read().strip()[1:-1]

def move(loc, direction):
	if direction == 'E': return (loc[0]+1, loc[1])
	elif direction == 'W': return (loc[0]-1, loc[1])
	elif direction == 'S': return (loc[0], loc[1]+1)
	elif direction == 'N': return (loc[0], loc[1]-1)

connections = defaultdict(list)

def recurse(start_locations, text, i):
	locations = start_locations.copy()
	end_locations = []
	while True:
		if i >= len(text):
			end_locations = locations.copy()
			break
		#print('{!s:<30} {!s:<4} {!s:<4} {!s:<10}'.format(locations, i, text[i], end_locations))
		if text[i] == '(':
			locations, i = recurse(list(set(locations)), text, i+1)
		elif text[i] == ')':
			end_locations += locations
			break
		elif text[i] == '|':
			end_locations += locations
			locations = start_locations.copy()
		else:
			for j in range(len(locations)):
				new_location = move(locations[j], text[i])
				connections[new_location].append(locations[j])
				connections[locations[j]].append(new_location)
				locations[j] = new_location
		i += 1
	return end_locations, i

locations, _ = recurse([(0, 0)], line, 0)
print('locations', len(locations))
print('connections', len(connections))

to_check = [(0, 0)]
seen = set(to_check)
more_than_1000 = set()

i = 0
while True:
	new_to_check = []
	for location in to_check:
		for new in connections[location]:
			if new not in seen:
				if i >= (1000-1):
					more_than_1000.add(new)
				new_to_check.append(new)
				seen.add(new)
	to_check = new_to_check

	if not to_check:
		break
	i += 1

print(i)
print(len(more_than_1000))
