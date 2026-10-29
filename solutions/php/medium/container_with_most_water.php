<?php

function maxArea(array $height): int
{
    $left = 0;
    $right = count($height) - 1;
    $maxWater = 0;

    while ($left < $right) {
        $width = $right - $left;
        $water = $width * min($height[$left], $height[$right]);
        if ($water > $maxWater) {
            $maxWater = $water;
        }

        if ($height[$left] < $height[$right]) {
            $left++;
        } else {
            $right--;
        }
    }

    return $maxWater;
}

var_dump(maxArea([1, 8, 6, 2, 5, 4, 8, 3, 7])); // 49
var_dump(maxArea([1, 1]));                      // 1
var_dump(maxArea([4, 3, 2, 1, 4]));             // 16
var_dump(maxArea([1, 2, 1]));                   // 2
