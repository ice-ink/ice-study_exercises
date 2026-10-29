# Number of Islands (deepseek generated)

## Problem

Given an `m x n` grid of characters where each cell is either `'1'` (land) or `'0'` (water), return the number of **islands**. An island is a group of `'1'` cells connected **4-directionally** (up, down, left, right — *not* diagonally), and is completely surrounded by water or the grid boundary.

You may assume all four edges of the grid are surrounded by water (you do not need to handle "lakes" as islands — a body of land is one island no matter its shape).

## Examples

| Input | Output | Why |
|---|---|---|
| `[["1","1","1","1","0"],["1","1","0","1","0"],["1","1","0","0","0"],["0","0","0","0","0"]]` | `1` | All land cells are connected into one island |
| `[["1","1","0","0","0"],["1","1","0","0","0"],["0","0","1","0","0"],["0","0","0","1","1"]]` | `3` | Three separate clusters |
| `[["1"]]` | `1` | Single land cell |
| `[["0"]]` | `0` | No land at all |
| `[["1","0","1"],["0","1","0"],["1","0","1"]]` | `5` | Diagonal touches do **not** connect cells |

## Constraints

- `m == grid.length`
- `n == grid[i].length`
- `1 <= m, n <= 300`
- `grid[i][j]` is `'0'` or `'1'`

## Things to think about before coding

- When you find a `'1'`, how can you "consume" its entire island in one shot so you never count it twice?
- Which traversal (DFS or BFS) naturally explores all 4 neighbors? How do you avoid revisiting cells you already saw?
- Can you mark a visited cell without allocating a second grid?
- What is the worst-case recursion depth, and what does that mean for space complexity?

Give it a try before scrolling further.

---

## Solution

### Approach

Treat the grid as an undirected graph where each `'1'` cell is a node and 4-directional adjacency is an edge. Every time the scan finds an unvisited `'1'`, it has discovered a new island, so we increment the counter and **flood fill** that island, turning every reachable `'1'` into `'0'` (a "sink") so it will be skipped by the rest of the scan.

1. Iterate over every cell `(r, c)`.
2. If `grid[r][c] === '1'`, increment `islands` and start a DFS from `(r, c)`.
3. The DFS marks the current cell `'0'`, then recurses into its 4 in-bounds neighbors (any that are still `'1'`).
4. Continue scanning; cells already flooded are `'0'` and skipped.

**Time:** O(m·n), each cell is visited at most once. **Space:** O(m·n) worst case (recursion stack when all cells form one island).

### TypeScript

```typescript
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
```

### Rust

```rust
pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut grid = grid;
    let mut islands = 0;

    fn dfs(grid: &mut Vec<Vec<char>>, r: usize, c: usize, rows: usize, cols: usize) {
        if r >= rows || c >= cols || grid[r][c] != '1' {
            return;
        }
        grid[r][c] = '0';
        dfs(grid, r + 1, c, rows, cols);
        if r > 0 {
            dfs(grid, r - 1, c, rows, cols);
        }
        dfs(grid, r, c + 1, rows, cols);
        if c > 0 {
            dfs(grid, r, c - 1, rows, cols);
        }
    }

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] == '1' {
                islands += 1;
                dfs(&mut grid, r, c, rows, cols);
            }
        }
    }

    islands
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(
            num_islands(vec![
                vec!['1', '1', '1', '1', '0'],
                vec!['1', '1', '0', '1', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '0', '0', '0'],
            ]),
            1
        );
        assert_eq!(
            num_islands(vec![
                vec!['1', '1', '0', '0', '0'],
                vec!['1', '1', '0', '0', '0'],
                vec!['0', '0', '1', '0', '0'],
                vec!['0', '0', '0', '1', '1'],
            ]),
            3
        );
        assert_eq!(num_islands(vec![vec!['1']]), 1);
        assert_eq!(num_islands(vec![vec!['0']]), 0);
        assert_eq!(
            num_islands(vec![
                vec!['1', '0', '1'],
                vec!['0', '1', '0'],
                vec!['1', '0', '1'],
            ]),
            5
        );
    }
}
```

### Notes on the two implementations

- Both mutate the input in place, using `'0'` as the "already visited" marker, so no second visited array is needed.
- TypeScript's recursive arrow function closes over `grid`, `rows`, and `cols`; Rust's nested `fn` can't capture, so those values are passed as explicit parameters.
- In Rust, `r - 1`/`c - 1` need an `if r > 0` / `if c > 0` guard because `usize` cannot be negative; TypeScript's `r - 1` just becomes `-1` and is caught by the boundary check.

## Likely interviewer follow-ups

1. **Iterative version:** how would you rewrite the DFS with an explicit stack (or BFS with a queue), and why might you prefer that for very large grids?
2. **No mutation:** if you were forbidden from modifying `grid`, what data structure would you use to track visited cells, and what extra space would it cost?
3. **Diagonal islands:** how would the answer change if diagonal adjacency also counted as connected?

When you're ready, send me your own solution for review, ask questions, or tell me what difficulty you'd like next.

## Syntax Explanation

## The core idea in plain steps

1. Walk the grid row by row, column by column.
2. When you hit a `'1'`, you've found a new island: add 1 to your count.
3. "Drown" that whole island by changing every connected `'1'` to `'0'` (a flood fill).
4. Because the island is now all `'0'`, your scan will skip it, so each island is counted exactly once.

## TypeScript, piece by piece

```typescript
function numIslands(grid: string[][]): number {
```
The parameter `grid: string[][]` is "an array of arrays of strings" — a 2D grid. `: number` is the return type. `string` is used (not `number`) because the problem gives us the characters `'1'` and `'0'`.

```typescript
const cols = grid[0]!.length;
```
With the project's `noUncheckedIndexedAccess` option, `grid[0]` would normally have type `string[] | undefined`. The `!` is a **non-null assertion**: it tells TypeScript "I promise this is not `undefined`." It is safe here because the constraints guarantee `m >= 1`, so row 0 always exists.

```typescript
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
```
This is the flood fill. The arrow function `(r, c): void => …` is a function with no return value (`void`). The guard `if (…)` rejects three kinds of cells: out-of-bounds coordinates (`r < 0`, `r >= rows`, etc.) and water or already-visited cells (`!== "1"`). Once a cell passes, we mark it `'0'` and recurse into all four directions. Because the function references itself by name (`dfs`) inside its own body, it is **recursive** — that works because the body only runs after `dfs` has been assigned.

```typescript
for (let r = 0; r < rows; r++) {
  for (let c = 0; c < cols; c++) {
    if (grid[r]![c] === "1") {
      islands++;
      dfs(r, c);
    }
  }
}
```
The outer scan visits every cell exactly once. The `if` fires only for cells that are still `'1'` — i.e. land nobody has flooded yet. We count it, then `dfs` erases the whole island. `grid[r]![c]` again uses `!` because `grid[r]` is `string[] | undefined` and `[c]` is `string | undefined` under the strict index-access rules.

## Rust, piece by piece

```rust
pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let mut grid = grid;
```
The parameter `Vec<Vec<char>>` is a 2D grid of characters (`char`, not `String`). The second line is subtle: `grid` arrives as an immutable binding, but we need to mutate it, so we rebind it to a new **mutable** binding with `let mut grid = grid`. This **moves** ownership from the old `grid` to the new one (the name is shadowed), which is fine because we own the value.

```rust
fn dfs(grid: &mut Vec<Vec<char>>, r: usize, c: usize, rows: usize, cols: usize) {
    if r >= rows || c >= cols || grid[r][c] != '1' {
        return;
    }
    grid[r][c] = '0';
```
This is a nested `fn`, not a closure, so it captures nothing — everything it needs is passed in. `&mut Vec<Vec<char>>` is a **mutable reference** to the grid, which lets `dfs` write `'0'` into it. Indexing `grid[r][c]` returns a plain `char` (a value copy), never an `Option`, so there is no `unwrap` here.

```rust
if r > 0 {
    dfs(grid, r - 1, c, rows, cols);
}
if c > 0 {
    dfs(grid, r, c - 1, rows, cols);
}
```
`r` and `c` are `usize` (unsigned), so `r - 1` would **panic** if `r` were `0`. The `if r > 0` guard prevents that underflow. The `+1` directions (`r + 1`, `c + 1`) need no guard: if they overshoot, the `r >= rows` / `c >= cols` check at the top of `dfs` catches them.

```rust
if grid[r][c] == '1' {
    islands += 1;
    dfs(&mut grid, r, c, rows, cols);
}
```
To call `dfs`, which borrows the grid mutably, we pass `&mut grid`. This is a **reborrow** — Rust temporarily lends the grid to `dfs`, gets it back when the call returns, and lets the outer loop keep using `grid`. The borrow checker enforces that only one mutable borrow is live at a time.

```rust
islands
```
The final expression with no semicolon is the function's return value (Rust's **implicit return**). Adding a `;` would turn it into a statement and make the function return `()`, a compile error here.

## Trace: `[["1","1","0"],["0","1","0"],["0","0","0"]]` (should be `1`)

| Step | Cell | Land? | Action | Count |
|---|---|---|---|---|
| 1 | (0,0) | yes | `islands` → 1; DFS floods (0,0),(0,1),(1,1) to `'0'` | 1 |
| 2 | (0,1) | now `'0'` | skip | 1 |
| 3 | (0,2) | no | skip | 1 |
| 4 | (1,0) | no | skip | 1 |
| 5 | (1,1) | now `'0'` | skip | 1 |
| 6 | (1,2) | no | skip | 1 |
| 7 | (2,0) | no | skip | 1 |
| 8 | (2,1) | no | skip | 1 |
| 9 | (2,2) | no | skip | 1 |

The flood fill erases the whole L-shaped island in one step, so the scan never counts a cell twice.

## Quick syntax comparison

| Concept | TypeScript | Rust |
|---|---|---|
| 2D grid type | `string[][]` | `Vec<Vec<char>>` |
| Cell character | `"1"` / `"0"` | `'1'` / `'0'` |
| Read a cell | `grid[r]![c]` (with `!`) | `grid[r][c]` |
| Mark visited | `grid[r]![c] = "0"` | `grid[r][c] = '0'` |
| Recursive helper | arrow function (captures) | nested `fn` (explicit params) |
| Boundary for `r - 1` | `r - 1` then `< 0` check | `if r > 0` guard (no negatives) |
| Final return | `return islands;` | `islands` (no `;`) |
