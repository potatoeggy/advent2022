ops: list[int] = []
register = 1

while (line := input()) != "":
    match line.split():
        case ["noop"]:
            ops.append(0)
        case ["addx", num]:
            ops.extend([0, int(num)])

i = 0
for row in range(6):
    for col in range(40):
        print("#" if abs(col - register) <= 1 else ".", end="")
        register += ops[i]
        i += 1
    print()