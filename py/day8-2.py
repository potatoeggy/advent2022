# i'm *pretty* sure there's a dp solution
# to this, but i'm native and impatient

grid: list[str] = []

while (line := input()) != "":
    grid.append(line)

max_view = 0

for i, row in enumerate(grid):
    # all edges are zeroes so they can be skipped
    if i == 0 or i == len(grid) - 1:
        continue

    for j, tree in enumerate(row):
        if j == 0 or j == len(row) - 1:
            continue

        product = 1

        # guaranteed to be safe because of the above checks
        views = [
            list(reversed(row[:j])),
            row[j + 1 :],
            [col[j] for col in reversed(grid[:i])],
            [col[j] for col in grid[i + 1 :]],
        ]
        for view in views:
            num_trees = 0
            for k, char in enumerate(view, start=1):
                if char >= tree:
                    num_trees += k
                    break
            else:
                num_trees += len(view)
            product *= num_trees
        max_view = max(max_view, product)

print("Max:", max_view)

