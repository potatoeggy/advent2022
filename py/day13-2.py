import functools

def cmp(left: list | int, right: list | int) -> int:
    if isinstance(left, list) and isinstance(right, list):
        for l, r in zip(left, right):
            if (res := cmp(l, r)) != 0:
                return res
        return (len(left) > len(right)) - (len(left) < len(right))
    elif isinstance(left, int) and isinstance(right, int):
        return (left > right) - (left < right)
    else:
        return cmp([left], right) if isinstance(left, int) else cmp(left, [right])


i = 1
total = 0

packets = []
while ((first := input()), (second := input())) != ("", ""):
    packets.extend([eval(first), eval(second)])
    input()
packets.extend([[[2]], [[6]]])

packets.sort(key=functools.cmp_to_key(cmp))
print((packets.index([[2]])+1) * (packets.index([[6]])+1))