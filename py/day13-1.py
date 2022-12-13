def cmp(left: list | int, right: list | int) -> bool | None:
    if isinstance(left, list) and isinstance(right, list):
        for l, r in zip(left, right):
            if (res := cmp(l, r)) is not None:
                return res
        return None if len(left) == len(right) else len(left) < len(right)
    elif isinstance(left, int) and isinstance(right, int):
        return None if left == right else left < right
    else:
        return cmp([left], right) if isinstance(left, int) else cmp(left, [right])
   

i = 1
total = 0

while ((first := input()), (second := input())) != ("", ""):
    first: list = eval(first)
    second: list = eval(second)
    if cmp(first, second):
        total += i
        print("Pass!")
    i += 1
    input()
print(total)