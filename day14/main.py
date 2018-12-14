def main():
    LIMIT = 681901
    recipes = "37"
    i1, i2 = 0, 1
    while len(recipes) < LIMIT+10:
        a, b = int(recipes[i1]), int(recipes[i2])
        recipes += str(a + b)
        i1 = (i1 + a + 1) % len(recipes)
        i2 = (i2 + b + 1) % len(recipes)

    print(recipes[LIMIT:LIMIT+10])

    # part 2
    LIMIT = str(LIMIT)
    recipes = "37"
    i1, i2 = 0, 1
    while True:
        a, b = int(recipes[i1]), int(recipes[i2])
        recipes += str(a + b)
        i1 = (i1 + a + 1) % len(recipes)
        i2 = (i2 + b + 1) % len(recipes)

        if LIMIT in recipes[-8:]:
            break
    print(recipes.index(LIMIT))

main()
