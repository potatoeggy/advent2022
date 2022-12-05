from collections import deque
from math import ceil

grid: list[deque[str]] = []


while "1" not in (line := input()):
    if len(grid) == 0:
        # first run
        for _ in range(ceil(len(line) / 4)):
            grid.append(deque())

    parsed = (
        line.replace("    ", "0").replace("[", "").replace("]", "").replace(" ", "")
    )

    for i, char in enumerate(parsed):
        if char != "0":
            grid[i].append(char)

while True:
    command = "".join([c for c in input() if not c.isalpha()])
    if not command:
        continue

    nums_to_move, orig, dest = map(int, command.split())

    orig -= 1
    dest -= 1

    for c in reversed([grid[orig].popleft() for _ in range(nums_to_move)]):
        grid[dest].appendleft(c)
    print("".join((" " if len(d) == 0 else d[0]) for d in grid))

