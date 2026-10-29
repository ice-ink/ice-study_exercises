# N-Queens (deepseek generated)

## Problem

Given an integer `n`, return all distinct ways to place `n` queens on an `n × n` chessboard so that no two queens attack each other. A queen attacks any piece in the same row, column, or diagonal. Each solution is represented as a list of strings where `'.'` is an empty square and `'Q'` is a queen.

## Examples

| Input | Output | Why |
|---|---|---|
| `n = 4` | `[[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]` | The only two valid 4-queen boards |
| `n = 1` | `[["Q"]]` | One queen on a 1×1 board |
| `n = 2` | `[]` | No two queens can avoid attacking |
| `n = 3` | `[]` | No solution exists |

## Constraints

- `1 <= n <= 9`

## Things to think about before coding

- Placing one queen per *row* (or column) is forced. How does that turn the problem into choosing one column per row?
- A column conflict is easy to track. What arithmetic identity detects two cells on the same diagonal in O(1)?
- Backtracking = try a choice, recurse, undo the choice. Which three data structures must you undo?
- Target O(n!) time and O(n) extra space (excluding the output).

Give it a try before scrolling further.

---

## Solution

### Approach

Backtrack **row by row**, placing exactly one queen per row. Track used columns and both diagonal families with sets (or boolean arrays). Two cells share a *downward* diagonal when `row - col` is equal, and an *upward* diagonal when `row + col` is equal.

1. For each row, try every column.
2. Skip a column if its column, `row - col`, or `row + col` is already used.
3. Place the queen, mark the three sets, recurse to the next row, then unmark and remove the queen.
4. When `row === n`, convert the board to strings and record it.

**Time:** O(n!), roughly n choices × (n−1) × … with pruning. **Space:** O(n) for the recursion and sets, plus the output.

### TypeScript

```typescript
function solveNQueens(n: number): string[][] {
  const results: string[][] = [];
  const board: string[][] = Array.from({ length: n }, () => new Array<string>(n).fill("."));

  const cols = new Set<number>(); // used columns
  const diag1 = new Set<number>(); // row - col (anti-diagonal)
  const diag2 = new Set<number>(); // row + col (main diagonal)

  function backtrack(row: number): void {
    if (row === n) {
      results.push(board.map((r) => r.join("")));
      return;
    }

    for (let col = 0; col < n; col++) {
      const d1 = row - col;
      const d2 = row + col;
      if (cols.has(col) || diag1.has(d1) || diag2.has(d2)) continue;

      board[row]![col] = "Q";
      cols.add(col);
      diag1.add(d1);
      diag2.add(d2);

      backtrack(row + 1);

      board[row]![col] = ".";
      cols.delete(col);
      diag1.delete(d1);
      diag2.delete(d2);
    }
  }

  backtrack(0);
  return results;
}

// Quick checks
console.log(solveNQueens(4)); // [[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]
console.log(solveNQueens(1)); // [["Q"]]
console.log(solveNQueens(2)); // []
console.log(solveNQueens(3)); // []
```

### Rust

```rust
pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    let n = n as usize;
    let mut results: Vec<Vec<String>> = Vec::new();
    let mut board: Vec<Vec<char>> = vec![vec!['.'; n]; n];
    let mut cols = vec![false; n];
    let mut diag1 = vec![false; 2 * n]; // row - col + (n - 1), always in 0..2n
    let mut diag2 = vec![false; 2 * n]; // row + col, always in 0..2n

    fn backtrack(
        row: usize,
        n: usize,
        board: &mut Vec<Vec<char>>,
        cols: &mut Vec<bool>,
        diag1: &mut Vec<bool>,
        diag2: &mut Vec<bool>,
        results: &mut Vec<Vec<String>>,
    ) {
        if row == n {
            let solution: Vec<String> = board
                .iter()
                .map(|r| r.iter().collect::<String>())
                .collect();
            results.push(solution);
            return;
        }

        for col in 0..n {
            let d1 = row + n - 1 - col;
            let d2 = row + col;
            if cols[col] || diag1[d1] || diag2[d2] {
                continue;
            }

            board[row][col] = 'Q';
            cols[col] = true;
            diag1[d1] = true;
            diag2[d2] = true;

            backtrack(row + 1, n, board, cols, diag1, diag2, results);

            board[row][col] = '.';
            cols[col] = false;
            diag1[d1] = false;
            diag2[d2] = false;
        }
    }

    backtrack(0, n, &mut board, &mut cols, &mut diag1, &mut diag2, &mut results);
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let solutions = solve_n_queens(4);
        assert_eq!(solutions.len(), 2);
        assert_eq!(
            solutions,
            vec![
                vec![
                    ".Q..".to_string(),
                    "...Q".to_string(),
                    "Q...".to_string(),
                    "..Q.".to_string()
                ],
                vec![
                    "..Q.".to_string(),
                    "Q...".to_string(),
                    "...Q".to_string(),
                    ".Q..".to_string()
                ],
            ]
        );
    }

    #[test]
    fn no_solution_for_small_boards() {
        assert!(solve_n_queens(2).is_empty());
        assert!(solve_n_queens(3).is_empty());
    }

    #[test]
    fn single_cell() {
        assert_eq!(solve_n_queens(1), vec![vec!["Q".to_string()]]);
    }
}
```

### Notes on the two implementations

- TypeScript uses three `Set`s; Rust uses three `Vec<bool>`. Both give O(1) membership checks — `Set.has` / `Set.add` and `vec[index]` / `vec[index] = true`.
- Rust must offset the `row - col` diagonal by `+ (n - 1)` so it lands in `0..2n` instead of going negative; the sign is irrelevant to the conflict test.
- The Rust helper is a nested `fn` that takes every mutable piece as an explicit parameter, because Rust functions cannot capture outer variables the way TypeScript's nested `backtrack` can.

## Likely interviewer follow-ups

1. **Count only:** how would you return just the number of solutions (the "N-Queens II" variant) without materializing boards?
2. **Bitmask optimization:** how can you pack the column and diagonal sets into integers for faster membership checks?
3. **Symmetric dedup:** many solutions are rotations/reflections of each other; how would you return only the distinct ones up to symmetry?

When you're ready, send me your own solution for review, ask questions, or tell me what difficulty you'd like next.

## Syntax Explanation

## The core idea in plain steps

1. Build an `n × n` board of `'.'` and three "used" trackers: columns, `row - col` diagonals, `row + col` diagonals.
2. For row 0 to n−1, try each column.
3. If the column and both diagonals are free, place a queen, mark them, and recurse to the next row.
4. Undo the placement (unmark all three) when backtracking, and record the board as strings when all rows are filled.

## TypeScript, piece by piece

```typescript
const board: string[][] = Array.from({ length: n }, () => new Array<string>(n).fill("."));
```
`Array.from({ length: n }, fn)` builds an array of length `n` by calling `fn` once per slot; each call returns a fresh row of `n` dots. The `string[][]` annotation says "array of arrays of strings".

```typescript
const diag1 = new Set<number>(); // row - col
const diag2 = new Set<number>(); // row + col
```
Two cells `(r1,c1)` and `(r2,c2)` are on the same `\`-diagonal exactly when `r1 - c1 === r2 - c2`, and on the same `/`-diagonal when `r1 + c1 === r2 + c2`. Storing these two numbers per queen lets us test a whole diagonal with one `Set.has` call.

```typescript
if (cols.has(col) || diag1.has(d1) || diag2.has(d2)) continue;
```
This is the prune step: if the column or either diagonal is already occupied, this column is attacked, so we skip it entirely and try the next one. This is what keeps the search from exploring doomed boards.

```typescript
board[row]![col] = "Q";
// ... backtrack(row + 1) ...
board[row]![col] = ".";
```
`board[row]!` asserts the row exists (it always does, since `row < n`). After recursing we restore the cell to `'.'` — this "undo" is the essence of backtracking and lets the same board be reused for sibling branches.

## Rust, piece by piece

```rust
let mut board: Vec<Vec<char>> = vec![vec!['.'; n]; n];
```
`vec!['.'; n]` makes a row of `n` dots, and `vec![row; n]` makes `n` copies of that row. The type is `Vec<Vec<char>>`: a vector of vectors of single characters (single quotes in Rust are `char`; double quotes are strings).

```rust
let mut diag1 = vec![false; 2 * n]; // row - col + (n - 1)
let d1 = row + n - 1 - col;
```
`row - col` ranges from `-(n-1)` to `n-1`, which would be negative and unusable as an array index. Adding `n - 1` shifts it into `0..2n`. The exact offset does not matter for correctness — only that two cells on the same diagonal produce the same value.

```rust
fn backtrack(
    row: usize,
    n: usize,
    board: &mut Vec<Vec<char>>,
    cols: &mut Vec<bool>,
    diag1: &mut Vec<bool>,
    diag2: &mut Vec<bool>,
    results: &mut Vec<Vec<String>>,
) { ... }
```
A nested `fn` in Rust cannot "see" the outer function's variables, so every piece of shared state is passed as a `&mut` (mutable borrow). The recursion passes the same borrows down, which is fine because each call returns before the borrow is used again.

```rust
let solution: Vec<String> = board
    .iter()
    .map(|r| r.iter().collect::<String>())
    .collect();
```
When all rows are placed, each `Vec<char>` row is collected into a `String` (via `collect::<String>()` on an iterator of `char`s), then the rows are collected into a `Vec<String>`. The `::<String>` turbofish tells `collect` exactly which type to build.

## Trace: `n = 4` (first solution `[".Q..","...Q","Q...","..Q."]`)

| Step | row | Action |
|---|---|---|
| 1 | 0 | place queen at col 0 |
| 2 | 1 | place queen at col 2 |
| 3 | 2 | every column attacked → backtrack to row 1 |
| 4 | 1 | place queen at col 3 |
| 5 | 2 | place queen at col 1 |
| 6 | 3 | every column attacked → backtrack … up to row 0 |
| 7 | 0 | place queen at col 1 |
| 8 | 1 | place queen at col 3 |
| 9 | 2 | place queen at col 0 |
| 10 | 3 | place queen at col 2 → `row == n`, record solution |

The first recorded board is queens at `(0,1),(1,3),(2,0),(3,2)`:

```
.Q..
...Q
Q...
..Q.
```

Checking diagonals: `row - col` values are `-1,-2,2,1` and `row + col` values are `1,4,2,5` — all distinct, so no two queens attack.

## Quick syntax comparison

| Concept | TypeScript | Rust |
|---|---|---|
| 2D board | `Array.from({length:n}, () => new Array(n).fill("."))` | `vec![vec!['.'; n]; n]` |
| Used-set | `Set<number>` + `has`/`add`/`delete` | `Vec<bool>` + index read/write |
| Diagonal id | `row - col`, `row + col` | `row + n - 1 - col`, `row + col` |
| Nested helper | closure `function backtrack(row)` | `fn` with explicit `&mut` params |
| Row → string | `r.join("")` | `r.iter().collect::<String>()` |
| Add a result | `results.push(...)` | `results.push(...)` |
