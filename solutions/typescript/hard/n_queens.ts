// N-Queens — hard
// Return all distinct n-queens boards: n queens on an n×n board, none attacking each other.

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
