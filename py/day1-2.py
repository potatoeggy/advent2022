top_three: list[int] = []

while True:
    total = 0
    while (line := input()) != "":
        total += int(line)

    # this way is super
    # super
    # super inefficient
    # but it's real short yknow
    top_three.append(total)
    top_three.sort(reverse=True)
    if len(top_three) > 3:
        top_three.pop()
    print("Top three:", top_three)
    print("Top three total:", sum(top_three))