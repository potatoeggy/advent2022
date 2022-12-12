from dataclasses import dataclass
from typing import Callable
from collections import deque
from math import prod

@dataclass
class Monkey:
    items: deque[int]
    operation: Callable[[int], int]
    div_test: int
    test_dest: tuple[int, int]
    count = 0


monkeys: list[Monkey] = []

for _ in range(8):
    data = [input() for _ in range(7)]
    items = deque(map(int, data[1][data[1].find(":")+1:].split(",")))
    def op(old: int, command=data[2][data[2].find("=")+1:]) -> int:
        return eval(command)
    test = int(data[3].split()[-1])
    dest = (int(data[4].split()[-1]), int(data[5].split()[-1]))

    monkeys.append(Monkey(items, op, test, dest))

print("Processing")
total = prod([m.div_test for m in monkeys])
for _ in range(10000):
    for monkey in monkeys:
        monkey.count += len(monkey.items)
        while (item := monkey.items.popleft() if monkey.items else None) is not None:
            val = monkey.operation(item)
            val %= total
            if val % monkey.div_test == 0:
                monkeys[monkey.test_dest[0]].items.append(val)
            else:
                monkeys[monkey.test_dest[1]].items.append(val)

*_, one, two = sorted([m.count for m in monkeys])
print(one * two)
print([m.count for m in monkeys])
