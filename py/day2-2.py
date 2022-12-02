points = {"X": 0, "Y": 3, "Z": 6}
res_points = {
    "A": {"X": 3, "Y": 1, "Z": 2},
    "B": {"X": 1, "Y": 2, "Z": 3},
    "C": {"X": 2, "Y": 3, "Z": 1},
}

total = 0

while True:
    opp, res = input().split()
    total += points[res] + res_points[opp][res]
    print(total)
