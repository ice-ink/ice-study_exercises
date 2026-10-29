<?php

function rotate(array &$nums, int $k): void
{
    $n = count($nums);
    if ($n === 0) {
        return;
    }
    $k %= $n;

    reverseRange($nums, 0, $n - 1);
    reverseRange($nums, 0, $k - 1);
    reverseRange($nums, $k, $n - 1);
}

function reverseRange(array &$nums, int $start, int $end): void
{
    while ($start < $end) {
        [$nums[$start], $nums[$end]] = [$nums[$end], $nums[$start]];
        $start++;
        $end--;
    }
}

$a1 = [1, 2, 3, 4, 5, 6, 7];
rotate($a1, 3);
var_dump($a1); // [5, 6, 7, 1, 2, 3, 4]

$a2 = [-1, -100, 3, 99];
rotate($a2, 2);
var_dump($a2); // [3, 99, -1, -100]

$a3 = [1, 2, 3];
rotate($a3, 4);
var_dump($a3); // [3, 1, 2]

$a4 = [1, 2, 3, 4];
rotate($a4, 0);
var_dump($a4); // [1, 2, 3, 4]
