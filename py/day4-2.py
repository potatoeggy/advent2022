dupes = 0

while True:
    s1, f1, s2, f2 = map(int, input().replace("-", ",").split(","))

    if s1 <= s2 <= f1 or s2 <= s1 <= f2 or s1 <= f2 <= f1 or s2 <= f1 <= f2:
        dupes += 1
    print("Dupes:", dupes)
