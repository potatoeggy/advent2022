line = input()
for i, c in enumerate(line[14:]):
    if len(set(line[i:i+14])) == 14:
        print(i+14)
        break
