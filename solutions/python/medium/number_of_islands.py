# Number of Islands — medium
# Given a grid of '1's (land) and '0's (water), return the number of islands
# (4-directionally connected land).


def num_islands(grid: list[list[str]]) -> int:
    rows, cols = len(grid), len(grid[0])
    islands = 0

    def dfs(r: int, c: int) -> None:
        if r < 0 or c < 0 or r >= rows or c >= cols or grid[r][c] != "1":
            return
        grid[r][c] = "0"
        dfs(r + 1, c)
        dfs(r - 1, c)
        dfs(r, c + 1)
        dfs(r, c - 1)

    for r in range(rows):
        for c in range(cols):
            if grid[r][c] == "1":
                islands += 1
                dfs(r, c)

    return islands


# Quick checks
print(num_islands([
    ["1", "1", "1", "1", "0"],
    ["1", "1", "0", "1", "0"],
    ["1", "1", "0", "0", "0"],
    ["0", "0", "0", "0", "0"],
]));  # 1
print(num_islands([
    ["1", "1", "0", "0", "0"],
    ["1", "1", "0", "0", "0"],
    ["0", "0", "1", "0", "0"],
    ["0", "0", "0", "1", "1"],
]));  # 3
print(num_islands([["1"]]));  # 1
print(num_islands([["0"]]));  # 0
print(num_islands([
    ["1", "0", "1"],
    ["0", "1", "0"],
    ["1", "0", "1"],
]));  # 5 (diagonals don't connect)
