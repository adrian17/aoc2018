tracks = open('input.txt').read().splitlines()

directions = [(1, 0), (0, 1), (-1, 0), (0, -1)]
locations = {}

class CartCollsion(Exception): pass

class Cart:
    def __init__(self, x, y, dir):
        self.x = x
        self.y = y
        self.dir = dir
        self.state = -1
        self.crashed = False

        locations[(self.x, self.y)] = self

    def move(self):
        dx, dy = directions[self.dir]
        del locations[(self.x, self.y)]
        self.x += dx
        self.y += dy

        if (self.x, self.y) in locations:
            print(self.x, self.y)
            other = locations.pop((self.x, self.y))
            self.crashed = True
            other.crashed = True
            return

        locations[(self.x, self.y)] = self

        c = tracks[self.y][self.x] 
        if c == '/':
            self.dir = {0: 3, 1: 2, 2: 1, 3: 0}[self.dir]
        elif c == '\\':
            self.dir = {0: 1, 1: 0, 2: 3, 3: 2}[self.dir]
        elif c == '+':
            self.dir = (self.dir + self.state) % 4
            self.state += 1
            if self.state == 2:
                self.state = -1

carts = []

for y, line in enumerate(tracks):
    for x, c in enumerate(line):
        if c in '>v<^':
            carts.append(Cart(x, y, '>v<^'.index(c)))


while len(carts) > 1:
    carts.sort(key=lambda cart: (cart.y, cart.x))

    for cart in carts:
        if not cart.crashed:
            cart.move()

    carts = [cart for cart in carts if not cart.crashed]

print('last:', carts[0].x, carts[0].y)
