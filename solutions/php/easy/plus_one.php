<?php

function plusOne(array $digits): array
{
    $count = count($digits);

    for ($i = $count - 1; $i >= 0; $i--) {
        if ($digits[$i] < 9) {
            $digits[$i]++;
            return $digits;
        }
        $digits[$i] = 0;
    }

    array_unshift($digits, 1);
    return $digits;
}

var_dump(plusOne([1, 2, 3])); // [1, 2, 4]
var_dump(plusOne([4, 3, 2, 1])); // [4, 3, 2, 2]
var_dump(plusOne([9])); // [1, 0]
var_dump(plusOne([9, 9, 9])); // [1, 0, 0, 0]
