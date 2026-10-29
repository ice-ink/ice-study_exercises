// Number of Islands — medium
// Given a grid of '1's (land) and '0's (water), return the number of islands (4-directionally connected land).

function numIslands(grid: string[][]): number {
  const rows = grid.length;
  const cols = grid[0]!.length;
  let islands = 0;

  const dfs = (r: number, c: number): void => {
    if (r < 0 || c < 0 || r >= rows || c >= cols || grid[r]![c] !== "1") {
      return;
    }
    grid[r]![c] = "0";
    dfs(r + 1, c);
    dfs(r - 1, c);
    dfs(r, c + 1);
    dfs(r, c - 1);
  };

  for (let r = 0; r < rows; r++) {
    for (let c = 0; c < cols; c++) {
      if (grid[r]![c] === "1") {
        islands++;
        dfs(r, c);
      }
    }
  }

  return islands;
}

// Quick checks
console.log(numIslands([
  ["1", "1", "1", "1", "0"],
  ["1", "1", "0", "1", "0"],
  ["1", "1", "0", "0", "0"],
  ["0", "0", "0", "0", "0"],
])); // 1
console.log(numIslands([
  ["1", "1", "0", "0", "0"],
  ["1", "1", "0", "0", "0"],
  ["0", "0", "1", "0", "0"],
  ["0", "0", "0", "1", "1"],
])); // 3
console.log(numIslands([["1"]])); // 1
console.log(numIslands([["0"]])); // 0
console.log(numIslands([
  ["1", "0", "1"],
  ["0", "1", "0"],
  ["1", "0", "1"],
])); // 5 (diagonals don't connect)
