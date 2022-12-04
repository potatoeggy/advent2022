from string import ascii_letters

total_prio = 0
while True:
    # code golfing
    first, second, third = map(set, (input() for _ in range(3)))
    letter, = first.intersection(second.intersection(third))
    total_prio += ascii_letters.find(letter) + 1

    print(f"prio: {total_prio}")

