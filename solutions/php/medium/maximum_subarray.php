<?php
// Maximum Subarray — medium
// Return the largest sum of any contiguous subarray.

function maxSubArray(array $nums): int
{
    $maxSoFar = $nums[0];
    $current = 0;

    foreach ($nums as $n) {
        if ($current < 0) {
            $current = 0;
        }
        $current += $n;
        if ($current > $maxSoFar) {
            $maxSoFar = $current;
        }
    }

    return $maxSoFar;
}

// Quick checks
var_dump(maxSubArray([-2, 1, -3, 4, -1, 2, 1, -5, 4])); // 6
var_dump(maxSubArray([1]));                              // 1
var_dump(maxSubArray([5, 4, -1, 7, 8]));                 // 23
var_dump(maxSubArray([-2, -1]));                         // -1
