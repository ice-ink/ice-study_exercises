<?php

function maxProfit(array $prices): int
{
    $minPrice = PHP_INT_MAX;
    $best = 0;

    foreach ($prices as $price) {
        $minPrice = min($minPrice, $price);
        $best = max($best, $price - $minPrice);
    }

    return $best;
}

var_dump(maxProfit([7, 1, 5, 3, 6, 4])); // 5
var_dump(maxProfit([7, 6, 4, 3, 1])); // 0
var_dump(maxProfit([1, 2])); // 1
var_dump(maxProfit([1])); // 0
