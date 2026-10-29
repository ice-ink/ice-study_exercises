<?php

function numIslands(array $grid): int
{
    $rows = count($grid);
    $cols = count($grid[0]);
    $islands = 0;

    $dfs = function (int $r, int $c) use (&$dfs, &$grid, $rows, $cols): void {
        if ($r < 0 || $c < 0 || $r >= $rows || $c >= $cols || $grid[$r][$c] !== '1') {
            return;
        }
        $grid[$r][$c] = '0';
        $dfs($r + 1, $c);
        $dfs($r - 1, $c);
        $dfs($r, $c + 1);
        $dfs($r, $c - 1);
    };

    for ($r = 0; $r < $rows; $r++) {
        for ($c = 0; $c < $cols; $c++) {
            if ($grid[$r][$c] === '1') {
                $islands++;
                $dfs($r, $c);
            }
        }
    }

    return $islands;
}

var_dump(numIslands([
    ['1', '1', '1', '1', '0'],
    ['1', '1', '0', '1', '0'],
    ['1', '1', '0', '0', '0'],
    ['0', '0', '0', '0', '0'],
])); // 1

var_dump(numIslands([
    ['1', '1', '0', '0', '0'],
    ['1', '1', '0', '0', '0'],
    ['0', '0', '1', '0', '0'],
    ['0', '0', '0', '1', '1'],
])); // 3

var_dump(numIslands([['1']])); // 1
var_dump(numIslands([['0']])); // 0

var_dump(numIslands([
    ['1', '0', '1'],
    ['0', '1', '0'],
    ['1', '0', '1'],
])); // 5 (diagonals don't connect)
