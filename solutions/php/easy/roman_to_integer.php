<?php

function romanToInt(string $s): int
{
    $values = [
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        'M' => 1000,
    ];

    $total = 0;
    $length = strlen($s);

    for ($i = 0; $i < $length; $i++) {
        $curr = $values[$s[$i]];
        $next = ($i + 1 < $length) ? $values[$s[$i + 1]] : 0;
        $total += ($curr < $next) ? -$curr : $curr;
    }

    return $total;
}

var_dump(romanToInt('III')); // 3
var_dump(romanToInt('LVIII')); // 58
var_dump(romanToInt('MCMXCIV')); // 1994
var_dump(romanToInt('IV')); // 4
