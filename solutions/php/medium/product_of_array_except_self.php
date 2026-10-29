<?php
// Product of Array Except Self — medium
// Return output[i] = product of all elements except nums[i], without division.

function productExceptSelf(array $nums): array
{
    $n = count($nums);
    $result = array_fill(0, $n, 1);

    $prefix = 1;
    for ($i = 0; $i < $n; $i++) {
        $result[$i] = $prefix;
        $prefix *= $nums[$i];
    }

    $suffix = 1;
    for ($i = $n - 1; $i >= 0; $i--) {
        $result[$i] *= $suffix;
        $suffix *= $nums[$i];
    }

    return $result;
}

// Quick checks
var_dump(productExceptSelf([1, 2, 3, 4]));      // [24, 12, 8, 6]
var_dump(productExceptSelf([-1, 1, 0, -3, 3])); // [0, 0, 9, 0, 0]
var_dump(productExceptSelf([0, 0]));            // [0, 0]
var_dump(productExceptSelf([2, 3]));            // [3, 2]
