<?php

// N-Queens — hard
// Return all distinct n-queens boards: n queens on an n×n board, none attacking each other.

function solveNQueens(int $n): array
{
    $results = [];
    $board = array_fill(0, $n, array_fill(0, $n, "."));

    $cols = [];  // used columns
    $diag1 = []; // row - col (anti-diagonal)
    $diag2 = []; // row + col (main diagonal)

    $backtrack = function (int $row) use (&$backtrack, &$results, &$board, &$cols, &$diag1, &$diag2, $n): void {
        if ($row === $n) {
            $solution = [];
            foreach ($board as $r) {
                $solution[] = implode("", $r);
            }
            $results[] = $solution;
            return;
        }

        for ($col = 0; $col < $n; $col++) {
            $d1 = $row - $col;
            $d2 = $row + $col;
            if (isset($cols[$col]) || isset($diag1[$d1]) || isset($diag2[$d2])) {
                continue;
            }

            $board[$row][$col] = "Q";
            $cols[$col] = true;
            $diag1[$d1] = true;
            $diag2[$d2] = true;

            $backtrack($row + 1);

            $board[$row][$col] = ".";
            unset($cols[$col]);
            unset($diag1[$d1]);
            unset($diag2[$d2]);
        }
    };

    $backtrack(0);
    return $results;
}

// Quick checks
var_dump(solveNQueens(4)); // [[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]
var_dump(solveNQueens(1)); // [["Q"]]
var_dump(solveNQueens(2)); // []
var_dump(solveNQueens(3)); // []
