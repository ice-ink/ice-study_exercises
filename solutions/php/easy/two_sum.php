<?php

function twoSum(array $nums, int $target): array
{
    $seen = []; // value -> index

    foreach ($nums as $i => $num) {
        $complement = $target - $num;
        if (isset($seen[$complement])) {
            return [$seen[$complement], $i];
        }
        $seen[$num] = $i;
    }

    return []; // unreachable: exactly one solution is guaranteed
}

var_dump(twoSum([2, 7, 11, 15], 9)); // [0, 1]
var_dump(twoSum([3, 2, 4], 6));      // [1, 2]
var_dump(twoSum([3, 3], 6));         // [0, 1]
