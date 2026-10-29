<?php

function threeSum(array $nums): array
{
    sort($nums);
    $result = [];
    $n = count($nums);

    for ($i = 0; $i < $n - 2; $i++) {
        if ($i > 0 && $nums[$i] === $nums[$i - 1]) {
            continue;
        }
        $left = $i + 1;
        $right = $n - 1;

        while ($left < $right) {
            $sum = $nums[$i] + $nums[$left] + $nums[$right];
            if ($sum < 0) {
                $left++;
            } elseif ($sum > 0) {
                $right--;
            } else {
                $result[] = [$nums[$i], $nums[$left], $nums[$right]];
                $left++;
                $right--;
                while ($left < $right && $nums[$left] === $nums[$left - 1]) {
                    $left++;
                }
                while ($left < $right && $nums[$right] === $nums[$right + 1]) {
                    $right--;
                }
            }
        }
    }

    return $result;
}

var_dump(threeSum([-1, 0, 1, 2, -1, -4])); // [[-1,-1,2],[-1,0,1]]
var_dump(threeSum([0, 1, 1]));             // []
var_dump(threeSum([0, 0, 0]));             // [[0,0,0]]
var_dump(threeSum([-2, 0, 0, 2, 2]));      // [[-2,0,2]]
