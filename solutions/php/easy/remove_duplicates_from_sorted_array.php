<?php

function removeDuplicates(array &$nums): int
{
    $count = count($nums);
    if ($count === 0) {
        return 0;
    }

    $slow = 0;
    for ($fast = 1; $fast < $count; $fast++) {
        if ($nums[$fast] !== $nums[$slow]) {
            $slow++;
            $nums[$slow] = $nums[$fast];
        }
    }

    return $slow + 1;
}

$nums = [1, 1, 2];
var_dump(removeDuplicates($nums)); // 2
var_dump($nums); // [1, 2, 2]

$nums = [0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
var_dump(removeDuplicates($nums)); // 5
var_dump($nums); // [0, 1, 2, 3, 4, 2, 2, 3, 3, 4]

$nums = [1];
var_dump(removeDuplicates($nums)); // 1
var_dump($nums); // [1]

$nums = [];
var_dump(removeDuplicates($nums)); // 0
var_dump($nums); // []
