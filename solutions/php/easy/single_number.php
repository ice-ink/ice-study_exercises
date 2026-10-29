<?php

function singleNumber(array $nums): int
{
    $result = 0;

    foreach ($nums as $num) {
        $result ^= $num;
    }

    return $result;
}

var_dump(singleNumber([2, 2, 1])); // 1
var_dump(singleNumber([4, 1, 2, 1, 2])); // 4
var_dump(singleNumber([1])); // 1
