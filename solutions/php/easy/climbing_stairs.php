<?php

function climbStairs(int $n): int
{
    if ($n <= 2) {
        return $n;
    }

    $first = 1;  // ways to climb 1 step
    $second = 2; // ways to climb 2 steps

    for ($i = 3; $i <= $n; $i++) {
        $third = $first + $second;
        $first = $second;
        $second = $third;
    }

    return $second;
}

var_dump(climbStairs(2)); // 2
var_dump(climbStairs(3)); // 3
var_dump(climbStairs(4)); // 5
var_dump(climbStairs(5)); // 8
var_dump(climbStairs(1)); // 1
